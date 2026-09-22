fn main() {
    let mut s = String::from("東京特許許可局");
    let mut t = String::from("青巻紙赤巻紙黄巻紙");
    println!("クリア前：「{s}」, 「{t}」");
    s.clear();
    t = "".to_string();
    println!("クリア後：「{s}」, 「{t}」");
}
