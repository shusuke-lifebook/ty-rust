use regex::RegexBuilder;

fn main() {
    let str = "あけまして\nおめでとうございます。";
    let re = RegexBuilder::new(r"^.+")
        .dot_matches_new_line(true)
        .build()
        .unwrap();
    let matched = re.find_iter(str);
    for m in matched {
        println!("{}", m.as_str());
    }
}
