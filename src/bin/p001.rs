extern crate env_logger;
#[macro_use]
extern crate log;

use std::time::Instant;

fn main() {
    env_logger::init();

    let start = Instant::now();

    let result = compute(1000);

    info!("Result is {}", result);

    let elapsed = start.elapsed();

    info!("{:?} millis elapsed", elapsed.as_millis());
}

fn compute(bound: u32) -> u32 {
    (0..bound).filter(|n| n % 3 == 0 || n % 5 == 0)
        .sum()
}

#[cfg(test)]
mod tests {
    #[test]
    fn sum_below_ten() {
        assert_eq!(23, super::compute(10));
    }
}