fn main() {
    let s1 = String::from("こんにちは、Rust!");
    let c = 'R';
    let (s2, pos) = search_position(s1, c);
    println!("文字'{c}'は\"{s2}\"の{pos}文字目です。");
}

fn search_position(s: String, c: char) -> (String, usize) {
    let pos = s.find(c).unwrap();
    (s, pos)
}
