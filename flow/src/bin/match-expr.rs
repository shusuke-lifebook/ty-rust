fn main() {
    let rank = 'S';
    let res = match rank {
        'S' => "素晴らしく優秀な成績です。",
        'A' => "とても優秀な成績です。",
        'B' => "優秀な成績です。",
        'C' => "合格です。",
        _ => "不合格です。",
    };
    println!("{res}");
}
