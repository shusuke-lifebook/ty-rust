use regex::Regex;

fn main() {
    let str = "ホームページはhttps://www.naosan.jp/、\n\
        サポートサイトはhttps://www.naosan.jp/support、\n\
        会社案内はhttps://www.naosan.jp/aboutです。";
    let re = Regex::new(r"http(s)?://([\w-]+\.)+[\w-]+(/[a-z_0-9-./?%&=]*)?").unwrap();
    //println!("{}", re.replace(str, "<a href='$0'>$1,$2,$3</a>"));
    println!("{}", re.replace(str, "<a href='$0'>$0</a>"));
    println!("{}", re.replacen(str, 2, "<a href='$0'>$0</a>"));
    println!("{}", re.replace_all(str, "<a href='$0'>$0</a>"));
}
