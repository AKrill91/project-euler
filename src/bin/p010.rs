extern crate env_logger;
#[macro_use]
extern crate log;
extern crate common;

use std::time::Instant;

fn main() {
    env_logger::init();

    let start = Instant::now();

    let result = compute(2_000_000);

    info!("Result is {}", result);

    let elapsed = start.elapsed();

    info!("{:?} millis elapsed", elapsed.as_millis());
}

fn compute(limit: u64) -> u64 {
    let sum: u64 = (3..=limit)
        .step_by(2)
        .filter(|&n| common::prime::is_prime(n))
        .sum();

    sum + 2
}

#[cfg(test)]
mod tests {
    #[test]
    fn below_ten() {
        assert_eq!(17, super::compute(10));
    }
}