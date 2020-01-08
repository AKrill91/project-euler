extern crate env_logger;
#[macro_use]
extern crate log;

use std::time::Instant;

struct Fibonnaci {
    current: u32,
    next: u32,
}

impl Fibonnaci {
    pub fn new() -> Fibonnaci {
        Fibonnaci { current: 1, next: 2 }
    }
}

impl Iterator for Fibonnaci {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        let current = self.current;
        let next = self.current + self.next;
        self.current = self.next;
        self.next = next;

        Some(current)
    }
}

fn main() {
    env_logger::init();

    let start = Instant::now();

    let result = compute(4_000_000);

    info!("Result is {}", result);

    let elapsed = start.elapsed();

    info!("{:?} millis elapsed", elapsed.as_millis());
}

fn compute(bound: u32) -> u32 {
    Fibonnaci::new()
        .take_while(|&n| {
            info!("Testing {}", n);
            n < bound
        })
        .filter(|&n| n % 2 == 0)
        .sum()
}


#[cfg(test)]
mod tests {
    #[test]
    fn sum_below_hundred() {
        assert_eq!(44, super::compute(100));
    }
}