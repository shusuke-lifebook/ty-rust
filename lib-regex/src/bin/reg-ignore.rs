use regex::RegexBuilder;

fn main() {
    let str = "仕事用はwings@example.comです。プライベート用はYAMA@example.comです。";
    let re = RegexBuilder::new(r"([a-z0-9.!#$%&'*+/=?^_{|}~-]+)@([a-z0-9-]+(\.[a-z0-9-]+)*)")
        .case_insensitive(true)
        .build()
        .unwrap();
    let matched = re.find_iter(str);
    for m in matched {
        println!("{}", m.as_str());
    }
}
