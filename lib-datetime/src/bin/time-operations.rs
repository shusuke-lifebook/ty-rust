use chrono::{Days, Local, Months, TimeDelta, TimeZone};

fn main() {
    let datetime1 = Local.with_ymd_and_hms(2025, 4, 29, 23, 59, 59).unwrap();
    let datetime2 = Local.with_ymd_and_hms(2018, 8, 1, 11, 22, 33).unwrap();
    let day = Days::new(10);
    println!("{}", datetime1 + day);
    println!("{}", datetime2 - day);
    let month = Months::new(3);
    println!("{}", datetime1 + month);
    println!("{}", datetime2 - month);
    let delta = TimeDelta::seconds(100);
    println!("{}", datetime1 + delta);
    println!("{}", datetime2 - delta);
    println!("{}", datetime1 == datetime2);
    println!("{}", datetime1 >= datetime2);
}
