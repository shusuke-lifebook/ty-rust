use chrono::{Datelike, Local, TimeZone, Timelike};

fn main() {
    let datetime = Local.with_ymd_and_hms(2025, 4, 29, 23, 59, 59).unwrap();
    println!(
        "{}年{}月{}日（{}）{}時{}分{}秒{}ナノ秒",
        datetime.year(),
        datetime.month(),
        datetime.day(),
        datetime.weekday(),
        datetime.hour(),
        datetime.minute(),
        datetime.second(),
        datetime.nanosecond(),
    );
    println!(
        "本日の経過時間：{}秒 紀元1年から{}日",
        datetime.num_seconds_from_midnight(),
        datetime.num_days_from_ce()
    );
}
