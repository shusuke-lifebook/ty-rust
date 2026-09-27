use regex::Regex;

fn main() {
    let telno = ["080-0000-0000", "045-000-0001", "225-0001"];
    let re = Regex::new(r"\d{2,4}-\d{2,4}-\d{4}").unwrap();
    for t in telno {
        if re.is_match(t) {
            println!("{t} がマッチしました");
        } else {
            println!("{t} はマッチしません");
        }
    }
}
