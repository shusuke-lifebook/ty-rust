// コメントアウトを解除するとコンパイルエラーになります。
fn main() {
    let mut string1 = String::new();
    string1 = "Hello, Rust!".to_string();
    let string2 = String::from("こんにちは、Rust!");
    println!("{string1}");
    println!("{string2}");
    let string3 = string1;
    println!("{string3}");

    // println!("{string1}"); // コンパイルエラー
}
