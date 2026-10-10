use regex::RegexBuilder;

fn main() {
    let tel = "自宅の電話番号は 045-0000-0000です。モバイルは 090-000-0000です。";
    let result = r"
        \d{2,4} # 市外局番
        -\d{2,4} # 市内局番
        -\d{4} # 加入者番号
        ";
    let re = RegexBuilder::new(result)
        .ignore_whitespace(true)
        .build()
        .unwrap();
    let matched = re.find_iter(tel);
    for m in matched {
        println!(
            "位置: {} 長さ: {} マッチ文字列: {}",
            m.start(),
            m.len(),
            m.as_str()
        );
    }
}
