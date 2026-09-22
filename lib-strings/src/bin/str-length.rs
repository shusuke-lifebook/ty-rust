use unicode_segmentation::UnicodeSegmentation;

fn main() {
    let str1 = "Rustファンクラブ".to_string();
    println!("{}", str1.chars().count());

    let str2 = "𩸽焼き".to_string();
    println!("{}", str2.chars().count());

    println!("{}", str1.len());

    let str3 = "頑張れ🇯🇵".to_string();
    println!("{}", str3.chars().count());
    println!("{}", str3.graphemes(true).count());
}
