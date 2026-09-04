use std::str::FromStr;

fn main() {
    let s = "123";
    // let s = "abc";
    let Ok(n) = i32::from_str(s) else { panic!() };
    println!("nは{n}に初期化されました。");
}
