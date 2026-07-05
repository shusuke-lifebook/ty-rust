use std::io;

fn main() {
    println!("あなたのお名前を教えてください。");
    let mut name = String::new();
    io::stdin().read_line(&mut name).unwrap();
    println!("こんにちは! {name}")
}
