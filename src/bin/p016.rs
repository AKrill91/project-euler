extern crate common;
extern crate env_logger;
#[macro_use]
extern crate log;

use std::time::Instant;

fn main() {
    env_logger::init();

    let start = Instant::now();

    let result = compute(2,1_000);

    info!("Result is {}", result);

    let elapsed = start.elapsed();

    info!("{:?} millis elapsed", elapsed.as_millis());
}

fn compute(base: u64, exponent: u32) -> u64 {
    if exponent == 0 {
        return 1;
    } else if exponent == 1 {
        return base;
    }

    let chunk_size = 10;
    let chunk_pow: u64 = 10u64.pow(chunk_size);

    let mut chunks: Vec<u64> = vec!(base);

    for _ in 2..=exponent {
        let num_chunks = chunks.len();
        let mut carry = 0;

        for chunk_index in 0..num_chunks {
            let current = chunks[chunk_index];
            let next = current * base + carry;
            carry = next / chunk_pow;
            let new = next % chunk_pow;
            chunks[chunk_index] = new;

            if carry > 0 && num_chunks == chunk_index + 1 {
                chunks.push(carry);
            }
        }
    }

    sum_digits(chunks)
}

fn sum_digits(chunks: Vec<u64>) -> u64 {
    let mut sum = 0;

    for chunk in chunks {
        let mut remainder = chunk;
        while remainder >= 10 {
            sum += remainder % 10;
            remainder /= 10;
        }
        sum += remainder;
    }

    sum
}

#[cfg(test)]
mod tests {
    #[test]
    fn sum_digits_fifteen() {
        let _ = env_logger::builder().is_test(true).try_init();
        assert_eq!(6, super::sum_digits(vec!(15)));
    }

    #[test]
    fn sum_digits_two_to_fifteen() {
        let _ = env_logger::builder().is_test(true).try_init();
        assert_eq!(26, super::sum_digits(vec!(32_678)));
    }

    #[test]
    fn sum_digits_multiple_chunks() {
        let _ = env_logger::builder().is_test(true).try_init();
        assert_eq!(26, super::sum_digits(vec!(32, 678)));
    }

    #[test]
    fn two_to_one() {
        let _ = env_logger::builder().is_test(true).try_init();
        assert_eq!(2, super::compute(2, 1));
    }

    #[test]
    fn two_to_three() {
        let _ = env_logger::builder().is_test(true).try_init();
        assert_eq!(26, super::compute(2, 15));
    }

    #[test]
    fn two_to_fifteen() {
        let _ = env_logger::builder().is_test(true).try_init();
        assert_eq!(26, super::compute(2, 15));
    }

    #[test]
    fn two_to_twenty_seven() {
        let _ = env_logger::builder().is_test(true).try_init();
        assert_eq!(35, super::compute(2, 27));
    }

    #[test]
    fn two_to_hundred() {
        let _ = env_logger::builder().is_test(true).try_init();
        assert_eq!(115, super::compute(2, 100));
    }

    #[test]
    fn two_to_five_hundred() {
        let _ = env_logger::builder().is_test(true).try_init();
        assert_eq!(679, super::compute(2, 500));
    }
}