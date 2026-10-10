use chrono::{Local, TimeZone};

fn main() {
    let datetime = Local.with_ymd_and_hms(2025, 4, 29, 23, 59, 59).unwrap();
    println!("{datetime}")
}
