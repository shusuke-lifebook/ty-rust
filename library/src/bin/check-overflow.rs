fn main() {
    let a = 100i8;
    let b = 30i8;
    if let Some(result) = a.checked_add(b) {
        println!("{a} ＋ {b} ＝ {result}");
    } else {
        println!("{a} ＋ {b} はオーバーフローしました。");
    }
}
