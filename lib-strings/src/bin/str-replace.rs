fn main() {
    let str1 = "こんにちは、Rust!".to_string();
    println!("replace: {}", str1.replace("こんにちは", "こんばんは"));
    let str1 = "青巻紙赤巻紙黄巻紙".to_string();
    println!("replacen：{}", str1.replacen("巻紙", "信号", 2));
}
