extern crate chrono;
extern crate common;
extern crate env_logger;
#[macro_use]
extern crate log;

use std::time::Instant;
use chrono::{NaiveDate, Datelike, Weekday};

fn main() {
    env_logger::init();

    let start = Instant::now();

    let result = compute(1901, 2000);

    info!("Result is {}", result);

    let elapsed = start.elapsed();

    info!("{:?} millis elapsed", elapsed.as_millis());
}

fn compute(start_year: i32, end_year: i32) -> usize {
    (start_year..=end_year)
        .map(|year| {
            (1..=12)
                .map(|month| NaiveDate::from_ymd(year, month, 1))
                .filter(|date| date.weekday() == Weekday::Sun)
                .count()
        })
        .sum()
}