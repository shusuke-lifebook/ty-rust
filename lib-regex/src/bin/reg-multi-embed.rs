use regex::Regex;

fn main() {
    let str = "10人十色\n1を聞いて10を知る";
    //let re = Regex::new(r"^\d+").unwrap();
    let re = Regex::new(r"(?m:^\d+)").unwrap();
    let matched = re.find_iter(str);
    for m in matched {
        println!("{}", m.as_str());
    }
}
