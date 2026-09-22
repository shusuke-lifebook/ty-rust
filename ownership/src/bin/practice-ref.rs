fn main() {
    let s = String::from("Rust");
    print_string(&s);
    println!("こんにちは、{s} ！");
}

fn print_string(m: &String) {
    println!("「{m}」を受け取りました。");
}
