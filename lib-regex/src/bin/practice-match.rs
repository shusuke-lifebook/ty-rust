use regex::Regex;

fn main() {
    let s = "私の住所は〒225-0000 横浜市青葉町0-0-0です。\nあなたの住所は〒273-9999 船橋市海老川町9-9-9ですね。";
    let re = Regex::new(r"\d{3}-\d{4}").unwrap();
    let matched = re.find_iter(s);
    for m in matched {
        println!("{}", m.as_str());
    }
}
