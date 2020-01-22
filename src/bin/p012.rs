extern crate common;
extern crate env_logger;
#[macro_use]
extern crate log;

use std::time::Instant;

fn main() {
    env_logger::init();

    let start = Instant::now();

    let result = compute(500);

    info!("Result is {}", result);

    let elapsed = start.elapsed();

    info!("{:?} millis elapsed", elapsed.as_millis());
}

fn compute(num_divisors: u64) -> u64 {
    let mut triangle_sum = 0;

    for i in 1..std::u64::MAX {
        triangle_sum += i;

        info!("The {} triangle number is {}", i, triangle_sum);

        let factors = common::prime::factors(triangle_sum);

        let permutations: u64 = factors.into_iter()
            .map(|(_number, power)| power + 1)
            .product();

        info!("{} has {} permutations", triangle_sum, permutations);

        if permutations > num_divisors {
            return triangle_sum;
        }
    }

    0
}

#[cfg(test)]
mod tests {
    #[test]
    fn below_ten() {
        let _ = env_logger::builder().is_test(true).try_init();
        assert_eq!(28, super::compute(5));
    }
}