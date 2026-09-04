use std::str::FromStr;

fn main() {
    let s = "123";
    if let Ok(n) = i32::from_str(s) {
        println!("うまく変換されました: {}", n);
    } else {
        println!("変換エラーです。");
    }
}
