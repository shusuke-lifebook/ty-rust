use regex::Regex;

fn main() {
    let tel = "自宅の電話番号は 045-0000-0000です。モバイルは 090-000-0000です。";
    let re = Regex::new(r"(\d{2,4})-(\d{2,4})-(\d{4})").unwrap();
    let captured = re.captures(tel).unwrap();
    if captured.len() == 4 {
        println!("マッチ文字列0：{}", captured.get(0).unwrap().as_str());
        println!("マッチ文字列1：{}", captured.get(1).unwrap().as_str());
        println!("マッチ文字列2：{}", captured.get(2).unwrap().as_str());
        println!("マッチ文字列3：{}", captured.get(3).unwrap().as_str());
    }
}
