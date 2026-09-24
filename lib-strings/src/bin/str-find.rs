fn main() {
    let s = "Hello, Rust, Rust!".to_string();
    if let Some(p) = s.find("Rust") {
        println!("先頭からの位置： {p}");
    }

    if let Some(p) = s.rfind("Rust") {
        println!("末尾からの位置： {p}");
    }

    if let Some(p) = s.find(char::is_lowercase) {
        println!("先頭からの位置： {p}");
    }

    if let Some(p) = s.find(|c| c == 'R') {
        println!("先頭からの位置： {p}");
    }
}
