fn main() {
    let score = 75;
    let s = if score >= 90 {
        "秀"
    } else if score >= 80 {
        "優"
    } else if score >= 70 {
        "良"
    } else {
        "不可"
    };
    println!("{s}");
}
