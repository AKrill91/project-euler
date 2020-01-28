extern crate common;
extern crate env_logger;
#[macro_use]
extern crate log;

use std::time::Instant;

const HUNDRED: &str = "hundred";
const HUNDRED_SEPARATOR: &str = "and";

fn main() {
    env_logger::init();

    let start = Instant::now();

    let result = compute(1_000);

    info!("Result is {}", result);

    let elapsed = start.elapsed();

    info!("{:?} millis elapsed", elapsed.as_millis());
}

fn compute(limit: u32) -> usize {
    (1..=limit)
        .map(|n| letter_count(n))
        .sum()
}

fn letter_count(n: u32) -> usize {
    get_words(n).into_iter()
        .map(|word| word.len())
        .sum()

}

fn get_words(n: u32) -> Vec<&'static str> {
    let mut remainder = n;
    let mut output = vec!();

    while remainder > 0 {
        let word = get_word(remainder);

        if word.len() > 0 {
            remainder = 0;
            output.push(word);
        } else if remainder >= 100 {
            let num_hundreds = remainder / 100;
            remainder %= 100;
            output.push(get_word(num_hundreds));
            output.push(HUNDRED);
            if remainder != 0 {
                output.push(HUNDRED_SEPARATOR);
            }
        } else if remainder > 20 {
            let num_tens = remainder / 10;
            remainder %= 10;
            output.push(get_word(num_tens * 10));
        }
    }

    output
}

fn get_word(n: u32) -> &'static str {
    match n {
        0 => "zero",
        1 => "one",
        2 => "two",
        3 => "three",
        4 => "four",
        5 => "five",
        6 => "six",
        7 => "seven",
        8 => "eight",
        9 => "nine",
        10 => "ten",
        11 => "eleven",
        12 => "twelve",
        13 => "thirteen",
        14 => "fourteen",
        15 => "fifteen",
        16 => "sixteen",
        17 => "seventeen",
        18 => "eighteen",
        19 => "nineteen",
        20 => "twenty",
        30 => "thirty",
        40 => "forty",
        50 => "fifty",
        60 => "sixty",
        70 => "seventy",
        80 => "eighty",
        90 => "ninety",
        1_000 => "onethousand",
        _ => ""
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn letter_count_one_hundred() {
        let _ = env_logger::builder().is_test(true).try_init();
        assert_eq!(10, super::letter_count(100));
    }

    #[test]
    fn letter_count_one_hundred_and_fifteen() {
        let _ = env_logger::builder().is_test(true).try_init();
        assert_eq!(20, super::letter_count(115));
    }

    #[test]
    fn letter_count_hundred_and_twenty_five() {
        let _ = env_logger::builder().is_test(true).try_init();
        assert_eq!(23, super::letter_count(125));
    }

    #[test]
    fn letter_count_three_hundred_and_fourty_two() {
        let _ = env_logger::builder().is_test(true).try_init();
        assert_eq!(23, super::letter_count(342));
    }

    #[test]
    fn letter_count_thousand() {
        let _ = env_logger::builder().is_test(true).try_init();
        assert_eq!(11, super::letter_count(1_000));
    }

    #[test]
    fn up_to_five() {
        let _ = env_logger::builder().is_test(true).try_init();
        assert_eq!(19, super::compute(5));
    }

    #[test]
    fn up_to_ten() {
        let _ = env_logger::builder().is_test(true).try_init();
        assert_eq!(39, super::compute(10));
    }
}