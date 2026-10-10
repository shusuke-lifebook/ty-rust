use chrono::{DateTime, Datelike, Timelike};

fn main() {
    let datetime =
        DateTime::parse_from_str("2025/04/29 14:20:17 +09:00", "%Y/%m/%d %H:%M:%S %z").unwrap();
    println!("{}日{}時", datetime.day(), datetime.hour());
}
