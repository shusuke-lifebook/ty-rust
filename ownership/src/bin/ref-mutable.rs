fn main() {
    let mut s = String::from("こんにちは、");
    println!("変更前：{s}");
    change_string(&mut s);
    println!("変更後：{s}");
}

fn change_string(s: &mut String) {
    s.push_str("Rust! ");
}
