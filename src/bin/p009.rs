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

fn compute(goal: u64) -> u64 {
    for a in 1..goal {
        let remainder = goal - a;
        for b in 1..remainder - 1 {
            let c = goal - a - b;

            let c_square = c * c;
            let square_sum = a * a + b * b;

            if c_square == square_sum {
                info!("Triangle is {} x {} x {}", a, b, c);
                return a * b * c;
            }
        }
    }

    0
}

#[cfg(test)]
mod tests {
    #[test]
    fn three_four_five() {
        assert_eq!(60, super::compute(12));
    }
}