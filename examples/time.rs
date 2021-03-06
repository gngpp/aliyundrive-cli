use chrono::{FixedOffset, TimeZone};
use std::str::FromStr;
use std::time::SystemTime;

fn main() {
    // drive::r#const::TIME_FORMAT
    let result = chrono::DateTime::parse_from_rfc3339("2022-06-22T03:15:02Z").unwrap();
    let format = result
        .with_timezone(&FixedOffset::east(8 * 3600))
        .format(drive::standard::TIME_FORMAT);
    println!("{}", format.to_string());
}
