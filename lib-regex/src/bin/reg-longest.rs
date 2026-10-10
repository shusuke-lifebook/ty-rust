use regex::Regex;

fn main() {
    let str =
        "<p><strong>WINGS</strong>サイト<a href='index.html'><img src='wings.jpg'></img></a></p>";
    // let re = Regex::new(r"<.+>").unwrap();
    let re = Regex::new(r"<.+?>").unwrap();
    let matched = re.find_iter(str);
    for m in matched {
        println!("{}", m.as_str());
    }
}
