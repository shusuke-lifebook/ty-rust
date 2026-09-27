use regex::Regex;

fn main() {
    let tel = "自宅の電話番号は 045-000-0000です。モバイルは 090-000-0000です。";
    let re = Regex::new(r"\d{2,4}-\d{2,4}-\d{4}").unwrap();
    let matched = re.find(tel).unwrap();
    if !matched.is_empty() {
        println!(
            "位置： {} マッチ文字列： {}",
            matched.start(),
            matched.as_str()
        );
    }
}
