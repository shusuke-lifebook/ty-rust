fn main() {
    let mut s = String::from("Rust");
    s = print_string(s);
    println!("こんにちは、{s} !");
}

fn print_string(m: String) -> String {
    println!("「{m}」を受け取りました。");
    m
}
