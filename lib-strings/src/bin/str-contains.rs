fn main() {
    let str1 = "Hello, Rust!".to_string();
    println!("{}", str1.contains("Rust"));
    println!("{}", str1.starts_with("Hello"));
    println!("{}", str1.ends_with("Hello"));
}
