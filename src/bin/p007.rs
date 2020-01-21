extern crate env_logger;
#[macro_use]
extern crate log;

use std::time::Instant;

fn main() {
    env_logger::init();

    let start = Instant::now();

    let result = compute(10_001);

    info!("Result is {}", result);

    let elapsed = start.elapsed();

    info!("{:?} millis elapsed", elapsed.as_millis());
}

fn compute(n: u64) -> u64 {
    match n {
        1 => 2,
        2 => 3,
        _ => {
            let mut counter = 2;

            for i in (5..std::u64::MAX).step_by(2) {
                if is_prime(i) {
                    counter+=1;

                    if counter == n {
                        return i;
                    }
                }
            }

            0
        }
    }
}

fn is_prime(n: u64) -> bool {
    match n {
        1 => true,
        2 => true,
        3 => true,
        5 => true,
        7 => true,
        _ => {
            let sqrt = (n as f64).sqrt() as u64;

            (3..=sqrt).step_by(2)
                .all(|divisor| n % divisor != 0)
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn first_prime() {
        assert_eq!(2, super::compute(1));
    }

    #[test]
    fn second_prime() {
        assert_eq!(3, super::compute(2));
    }

    #[test]
    fn sixth_prime() {
        assert_eq!(13, super::compute(6));
    }

    #[test]
    fn fifty_first_prime() {
        assert_eq!(233, super::compute(51));
    }
}