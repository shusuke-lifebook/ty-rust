use regex::Regex;

fn main() {
    let str = "あけまして\nおめでとうございます。";
    //let re = Regex::new(r"^.+").unwrap();
    let re = Regex::new(r"(?s:^.+)").unwrap();
    let matched = re.find_iter(str);
    for m in matched {
        println!("{}", m.as_str());
    }
}
