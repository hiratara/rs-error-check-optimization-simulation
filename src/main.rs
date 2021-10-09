use rand::Rng;
use std::thread::sleep;
use std::time::{Duration, Instant};

static CHECKS: &'static [(f32, u64)] = &[(0.2, 1), (0.3, 2), (0.4, 4), (0.6, 5)];

fn main() {
    let mut rng = rand::thread_rng();
    let arg_samples: Vec<Vec<bool>> = (0..10000)
        .map(|_| CHECKS.iter().map(|v| rng.gen::<f32>() >= v.0).collect())
        .collect();
    let check_funs: Vec<_> = (0..CHECKS.len())
        .map(|i| {
            move |arg: &Vec<bool>| {
                sleep(Duration::from_millis(CHECKS[i].1));
                arg[i]
            }
        })
        .collect();
    let mut check_order: Vec<usize> = (0..CHECKS.len()).collect();

    let d1 = benchmark(&check_funs, &arg_samples, &check_order);
    println!("{}.{:03} sec", d1.as_secs(), d1.as_millis());

    check_order.sort_by(|&a, &b| {
        let cost_a: f32 = CHECKS[a].1 as f32 / CHECKS[a].0;
        let cost_b: f32 = CHECKS[b].1 as f32 / CHECKS[b].0;
        cost_a.partial_cmp(&cost_b).unwrap()
    });
    println!("optimized order: {:?}", check_order);
    let d2 = benchmark(&check_funs, &arg_samples, &check_order);
    println!("{}.{:03} sec (optimized)", d2.as_secs(), d2.as_millis());

    check_order.reverse();
    println!("worst order: {:?}", check_order);
    let d2 = benchmark(&check_funs, &arg_samples, &check_order);
    println!("{}.{:03} sec (worst)", d2.as_secs(), d2.as_millis());
}

fn benchmark<T>(
    check_funs: &Vec<T>,
    arg_samples: &Vec<Vec<bool>>,
    check_order: &Vec<usize>,
) -> Duration
where
    T: Fn(&Vec<bool>) -> bool,
{
    let timer = Instant::now();
    for arg in arg_samples {
        for &i in check_order {
            if !check_funs[i](arg) {
                // check fails, skip the rest of the checks
                break;
            }
        }
    }
    timer.elapsed()
}
