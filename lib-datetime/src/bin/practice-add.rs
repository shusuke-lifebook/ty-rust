use chrono::{Days, Local};

fn main() {
    let datetime = Local::now();
    let days = Days::new(14);
    println!("{}", datetime.checked_add_days(days).unwrap());
}
