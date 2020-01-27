extern crate common;
extern crate env_logger;
#[macro_use]
extern crate log;

use std::time::Instant;
use std::cmp::{max, min};

fn main() {
    env_logger::init();

    let start = Instant::now();

    let result = compute(20,20);

    info!("Result is {}", result);

    let elapsed = start.elapsed();

    info!("{:?} millis elapsed", elapsed.as_millis());
}

fn compute(width: u128, height: u128) -> u128 {
    let sum = width + height;
    let longest_side = max(width, height);
    let shortest_side = min(width, height);

    let top: Vec<u128> = (longest_side + 1..=sum).collect();
    let mut bottom: Vec<u128> = (1..=shortest_side).collect();

    let mut total = 1;

    for val in top {
        let mut i = 0;
        let mut adjusted_val = val;

        while i != bottom.len() {
            let bot = bottom[i];

            if adjusted_val % bot == 0 {
                bottom.remove(i);
                adjusted_val = adjusted_val / bot;
            } else {
                i += 1;
            }
        }

        total = total * adjusted_val;
    }

    let bottom_remainder: u128 = bottom.into_iter().product();

    total / bottom_remainder
}

#[cfg(test)]
mod tests {
    #[test]
    fn two_by_two() {
        let _ = env_logger::builder().is_test(true).try_init();
        assert_eq!(6, super::compute(2, 2));
    }

    #[test]
    fn six_by_four() {
        let _ = env_logger::builder().is_test(true).try_init();
        assert_eq!(210, super::compute(6,4));
    }
}