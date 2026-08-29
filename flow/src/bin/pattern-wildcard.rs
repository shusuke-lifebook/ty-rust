fn main() {
    let rank = 'S';
    match rank {
        _ => println!("不合格です。"),
        'S' => println!("素晴らしく優秀な成績です。"),
        'A' => println!("とても優秀な成績です。"),
        'B' => println!("優秀な成績です。"),
        'C' => println!("合格です。"),
    }
}
