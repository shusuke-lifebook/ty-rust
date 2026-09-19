fn main() {
    let a = String::from("Rust");
    let r = &a;
    let s = r;
    println!("{a}, {r}, {s}");
}
