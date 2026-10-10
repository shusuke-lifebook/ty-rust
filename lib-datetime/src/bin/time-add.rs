use chrono::{Days, Months, TimeDelta, TimeZone, Utc};

fn main() {
    let datetime = Utc.with_ymd_and_hms(2025, 4, 29, 23, 59, 59).unwrap();
    let day = Days::new(10);
    println!("{}", datetime.checked_add_days(day).unwrap());
    println!("{}", datetime.checked_sub_days(day).unwrap());
    let month = Months::new(3);
    println!("{}", datetime.checked_add_months(month).unwrap());
    println!("{}", datetime.checked_sub_months(month).unwrap());
    let hour = TimeDelta::hours(5);
    println!("{}", datetime.checked_add_signed(hour).unwrap());
    println!("{}", datetime.checked_sub_signed(hour).unwrap());
}
