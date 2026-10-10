use regex::Regex;

fn main() {
    let tel = "電話番号は、045-0000-0000です。";
    let re = Regex::new(r"(?<area>\d{2,4})-(?<city>\d{2,4})-(?<local>\d{4})").unwrap();
    if let Some(matched) = re.captures(tel) {
        println!("市外局番: {}", matched.name("area").unwrap().as_str());
        println!("市内局番: {}", matched.name("city").unwrap().as_str());
        println!("加入者番号: {}", matched.name("local").unwrap().as_str());
    }
}
