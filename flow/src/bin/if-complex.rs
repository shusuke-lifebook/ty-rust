fn main() {
    let rank = "S";
    let s = if rank == "S" {
        "素晴らしく優秀な成績です。"
    } else if rank == "A" {
        "とても優秀な成績です。"
    } else if rank == "B" {
        "優秀な成績です。"
    } else if rank == "C" {
        "合格です。"
    } else {
        "不合格です。"
    };
    println!("{s}");
}
