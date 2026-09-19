fn main() {
    let s: &String;
    let c = 'w';
    let pos = search_position(s, c);
    println!("文字 '{c}' は \"{s}\" の {pos} 文字目です。");
}

fn search_position(s: &String, c: char) -> usize {
    let pos = s.find(c).unwrap();
    pos
}
