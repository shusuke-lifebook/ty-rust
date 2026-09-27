fn main() {
    let mut s = String::from("Hello");
    let t = String::from("World");
    s += ", ";
    s += &t;
    println!("{s}");
}
