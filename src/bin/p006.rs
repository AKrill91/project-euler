extern crate env_logger;
#[macro_use]
extern crate log;

use std::time::Instant;

fn main() {
    env_logger::init();

    let start = Instant::now();

    let result = compute(100);

    info!("Result is {}", result);

    let elapsed = start.elapsed();

    info!("{:?} millis elapsed", elapsed.as_millis());
}

fn compute(limit: u64) -> u64 {
    let sum_of_squares: u64 = (1..=limit)
        .map(|i| i * i)
        .sum();

    let sum: u64 = (1..=limit).sum();
    let square_sum = sum * sum;

    square_sum - sum_of_squares
}

#[cfg(test)]
mod tests {
    #[test]
    fn first_ten() {
        assert_eq!(2640, super::compute(10));
    }
}