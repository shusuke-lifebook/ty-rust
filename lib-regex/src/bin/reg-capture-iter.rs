use regex::Regex;

fn main() {
    let tel = "自宅の電話番号は 045-0000-9999です。モバイルは 090-111-8888です。";
    let re = Regex::new(r"(\d{2,4})-(\d{2,4})-(\d{4})").unwrap();
    let captured = re.captures_iter(tel);
    for c in captured {
        println!("マッチ文字列0：{}", c.get(0).unwrap().as_str());
        println!("マッチ文字列1：{}", c.get(1).unwrap().as_str());
        println!("マッチ文字列2：{}", c.get(2).unwrap().as_str());
        println!("マッチ文字列3：{}", c.get(3).unwrap().as_str());
    }
}
