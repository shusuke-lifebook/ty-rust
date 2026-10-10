use regex::RegexBuilder;

fn main() {
    let s = "お問い合わせはhoge@example.comまで";
    let r = RegexBuilder::new(r"([a-z0-9.!#$%&'*+/=?^_{|}~-]+)@([a-z0-9-]+(\.[a-z0-9-]+)*)")
        .case_insensitive(true)
        .build()
        .unwrap();
    println!("{}", r.replace(s, "<a href='mailto:$0'>$0</a>"));
}
