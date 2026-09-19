// コメントアウトを解除するとコンパイルエラーになります。
fn main() {
    let s1 = String::from("こんにちは");
    let r = &s1;
    println!("s1は {s1} です。");
    let s2 = s1;
    println!("s2は {s2} です。");
    // println!("rは {r} です。");
}
