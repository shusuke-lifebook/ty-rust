use chrono::{Days, Local, Months, TimeDelta, TimeZone};

fn main() {
    let datetime = Local.with_ymd_and_hms(2025, 4, 29, 23, 59, 59).unwrap();
    let days = Days::new(10);
    println!("{}", datetime.checked_add_days(days).unwrap());
    println!("{}", datetime.checked_sub_days(days).unwrap());
    let months = Months::new(3);
    println!("{}", datetime.checked_add_months(months).unwrap());
    println!("{}", datetime.checked_sub_months(months).unwrap());
    let delta = TimeDelta::seconds(100);
    println!("{}", datetime.checked_add_signed(delta).unwrap());
    println!("{}", datetime.checked_sub_signed(delta).unwrap());
}
