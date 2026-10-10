use regex::RegexBuilder;

fn main() {
    let str = "10人十色\n1を聞いて10を知る";
    let re = RegexBuilder::new(r"^\d+").multi_line(true).build().unwrap();
    let matched = re.find_iter(str);
    for m in matched {
        println!("{}", m.as_str());
    }
}
