// コメントアウトを解除するとコンパイルエラーになります。

fn main() {
    let mut s = String::from("Hello, Rust!!");
    let t = &s[0..5];
    //s.clear();
    println!("スライスは {t} です。");
}
