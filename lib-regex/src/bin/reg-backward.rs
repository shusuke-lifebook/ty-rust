use regex::Regex;

fn main() {
    let s =
        "<p>サポートサイト<a href='https://www.wings.msn.to/'>https://www.wings.msn.to/</a></p>";
    let re = Regex::new(r"<a href='(.+?)'>\1</a>").unwrap();
    let matched = re.find(s).unwrap();
    if !matched.is_empty() {
        println!(
            "位置：{} マッチ文字列：{}",
            matched.start(),
            matched.as_str()
        );
    }
}
