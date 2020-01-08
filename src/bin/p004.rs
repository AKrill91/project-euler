extern crate env_logger;
#[macro_use]
extern crate log;

use std::time::Instant;

fn main() {
    env_logger::init();

    let start = Instant::now();

    let result = compute(3);

    info!("Result is {}", result);

    let elapsed = start.elapsed();

    info!("{:?} millis elapsed", elapsed.as_millis());
}

fn is_palindrome(n: u64) -> bool {
    let chars: Vec<char> = n.to_string().chars().collect();

    let len = chars.len();
    let half = if len % 2 == 0 {
        len / 2
    } else {
        len / 2 - 1
    };

    (0..=half)
        .all(|i| chars[i] == chars[len - i - 1])
}

fn compute(num_digits: u32) -> u64 {
    let start = 10u64.pow(num_digits - 1);
    let end = 10u64.pow(num_digits) - 1;
    (start..=end).rev()
        .map(|n| {
            (start..n).rev()
                .map(|i| n * i)
                .find(|&i| is_palindrome(i))
        })
        .filter(|o| o.is_some())
        .map(|o| o.unwrap())
        .max()
        .unwrap()
}

#[cfg(test)]
mod tests {
    #[test]
    fn two_digits() {
        assert_eq!(9009, super::compute(2));
    }
}