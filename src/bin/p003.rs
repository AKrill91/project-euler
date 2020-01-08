extern crate env_logger;
#[macro_use]
extern crate log;

use std::time::Instant;

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

fn main() {
    env_logger::init();

    let start = Instant::now();

    let result = compute(600851475143);

    info!("Result is {}", result);

    let elapsed = start.elapsed();

    info!("{:?} millis elapsed", elapsed.as_millis());
}

fn compute(n: u64) -> u64 {
    let stop = (n as f64).sqrt() as u64;

    (2..stop)
        .filter(|&i| {
            n % i == 0
        })
        .flat_map(|i| {
            vec!(n / i, i)
        })
        .filter(|&i| is_prime(i))
        .max()
        .unwrap()
}

#[cfg(test)]
mod tests {
    #[test]
    fn largest_prime_example() {
        let _ = env_logger::builder().is_test(true).try_init();
        assert_eq!(29, super::compute(13195));
    }
}