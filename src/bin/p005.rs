extern crate env_logger;
#[macro_use]
extern crate log;

use std::time::Instant;

fn main() {
    env_logger::init();

    let start = Instant::now();

    let result = compute(20);

    info!("Result is {}", result);

    let elapsed = start.elapsed();

    info!("{:?} millis elapsed", elapsed.as_millis());
}

fn compute(max: usize) -> u64 {
    let step = max * (max - 1);

    ((step as u64)..std::u64::MAX)
        .step_by(step)
        .find(|&n| {
            (1..=(max as u64))
                .all(|i| n % i == 0)
        })
        .unwrap()
}

#[cfg(test)]
mod tests {
    #[test]
    fn one_to_ten() {
        assert_eq!(2520, super::compute(10));
    }
}