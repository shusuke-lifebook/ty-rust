use regex::Regex;

fn main() {
    let tel = "電話番号は、045-0000-0000です。";
    let re = Regex::new(r"(\d{2,4})-(\d{2,4})-(\d{4})").unwrap();
    if let Some(matched) = re.captures(tel) {
        println!("市外局番: {}", matched.get(1).unwrap().as_str());
        println!("市内局番: {}", matched.get(2).unwrap().as_str());
        println!("加入者番号: {}", matched.get(3).unwrap().as_str());
    }
}
