fn main() {
    let mut s = String::new();
    let mut f = || {
        s.push_str("こんにちは！");
        println!("{s}");
    };
    f();
    f();
    f();
}
