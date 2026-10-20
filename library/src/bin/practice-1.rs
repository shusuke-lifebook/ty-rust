use chrono::{Local, TimeDelta, TimeZone};

fn main() {
    // (1)
    let str = "となりのきゃくはよくきゃくくうきゃくだ";
    println!("{}", str.rfind("きゃく").unwrap());

    // (2)
    let loc = "熊谷";
    let temp = 40.124;
    println!("{}市の気温は{:.1}℃です。", loc, temp);

    // (3)
    let str = "ワタシの名前は花子です。";
    println!("{}", str.replace("ワタシ", "私"));

    // (4)
    let mut dt = Local::now();
    let td = TimeDelta::days(7) + TimeDelta::hours(3);
    dt = dt.checked_add_signed(td).unwrap();
    println!("{}", dt.format("%c").to_string());

    // (5)
    let dt1 = Local.with_ymd_and_hms(2022, 6, 2, 0, 0, 0).unwrap();
    let dt2 = Local.with_ymd_and_hms(2025, 7, 20, 0, 0, 0).unwrap();
    println!("{}", (dt2 - dt1).num_days())
}
