use chrono::{TimeZone, Utc};

fn main() {
    let datetime = Utc.with_ymd_and_hms(2025, 4, 29, 23, 59, 59).unwrap();
    println!("{}", datetime.format("%c").to_string());
    println!("{}", datetime.format("%Y/%m/%d(%b) %H:%M:%S").to_string());
}
