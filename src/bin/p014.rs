extern crate common;
extern crate env_logger;
#[macro_use]
extern crate log;

use std::time::Instant;
use std::collections::HashMap;

fn main() {
    env_logger::init();

    let start = Instant::now();

    let result = compute(1_000_000);

    info!("Result is {}", result);

    let elapsed = start.elapsed();

    info!("{:?} millis elapsed", elapsed.as_millis());
}

fn compute(limit: u64) -> u64 {
    let mut best = (1,0);
    let mut cache = HashMap::new();

    cache.insert(1,0);

    for i in 2..limit {
        if cache.contains_key(&i) {
            continue;
        }

        let mut search = i;
        let mut stack = vec!();
        stack.push(i);

        while !cache.contains_key(&search) {
            search = if search % 2 == 0 {
                search / 2
            } else {
                3 * search + 1
            };
            stack.push(search);
        }

        stack.pop();

        let mut step = *cache.get(&search).unwrap();

        while !stack.is_empty() {
            let elem = stack.pop().unwrap();

            cache.insert(elem, step);
            step += 1;
        }

        if step > best.1 {
            best = (i, step);
        }
    }

    info!("Best is {}, with {} steps", best.0, best.1);

    best.0
}

#[cfg(test)]
mod tests {
    #[test]
    fn below_ten() {
        let _ = env_logger::builder().is_test(true).try_init();
        assert_eq!(9, super::compute(10));
    }
}