fn main() {
    let mut f = create_closure();
    f();
    f();
    f();
}

fn create_closure() -> impl FnMut() {
    let mut s = String::new();
    move || {
        s.push_str("こんにちは！");
        println!("{s}");
    }
}
