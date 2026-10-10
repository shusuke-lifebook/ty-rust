use chrono::{DateTime, Local};

fn main() {
    let datetime1 =
        DateTime::parse_from_str("2025-04-29T23:59:59 +0900", "%Y-%m-%dT%H:%M:%S %z").unwrap();
    let datetime2 = "2025-04-29T23:59:59+09:00"
        .parse::<DateTime<Local>>()
        .unwrap();
    println!("{datetime1}");
    println!("{datetime2}");
}
