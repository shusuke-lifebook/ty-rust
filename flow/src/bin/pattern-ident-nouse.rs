use std::str::FromStr;

fn main() {
    let s = "123";
    match i32::from_str(s) {
        Ok(_) => println!("うまく変換されました。"),
        Err(_) => println!("変換エラーです。"),
    }
}
