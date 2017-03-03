extern crate chrono;

use chrono::{Datelike, Duration, TimeZone, UTC};

fn main() {
    let start = UTC.ymd(1901, 1, 6); // the first sunday of the 21st century
    let sundays = (1..)
        .map(|i| start + Duration::days(7 * i)) // incrementing by sundays
        .take_while(|d| d.year() <= 2000) // 20th century
        .filter(|d| d.day() == 1) // first of the month
        .count();

    println!("{}", sundays);
}
