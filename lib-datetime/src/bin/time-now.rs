use chrono::{Local, Utc};

fn main() {
    let utc_dt = Utc::now();
    let local_dt = Local::now();
    println!("{utc_dt}");
    println!("{local_dt}");
    let date = utc_dt.date_naive();
    let time = utc_dt.time();
    println!("{date}");
    println!("{time}");
}
