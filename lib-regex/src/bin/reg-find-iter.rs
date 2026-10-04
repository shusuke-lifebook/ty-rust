use regex::Regex;

fn main() {
    let tel = "自宅の電話番号は 045-0000-0000です。モバイルは 090-000-0000です。";
    let re = Regex::new(r"\d{2,4}-\d{2,4}-\d{4}").unwrap();
    let matched = re.find_iter(tel);
    for m in matched {
        println!(
            "位置： {} 長さ： {} マッチ文字列： {}",
            m.start(),
            m.len(),
            m.as_str()
        );
    }
}
