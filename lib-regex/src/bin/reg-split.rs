use regex::Regex;

fn main() {
    let st = "にわに2わうらにわに1わにわとりがいる";
    let re = Regex::new(r"\d{1,}わ").unwrap();
    println!("{}", re.split(st).collect::<Vec<_>>().join(","));
}
