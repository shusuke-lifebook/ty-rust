use regex::Regex;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let re = Regex::new(r"(?i:(http(s)?://([\w-]+\.)+[\w-]+(/[a-z_0-9-./?%&=]*)?))").unwrap();
    let filename = "urls.txt";
    let file = File::open(filename).unwrap();
    let mut br = BufReader::new(file);
    let mut s = String::new();
    while br.read_line(&mut s).unwrap() > 0 {
        let result = re.find_iter(&s);
        for m in result {
            println!("{}", m.as_str());
        }
        s.clear();
    }
}
