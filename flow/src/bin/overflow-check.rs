fn main() {
    // オーバフローをチェック
    let a = 100_u8;
    let b = 200_u8;
    #[allow(arithmetic_overflow)]
    if let Some(c) = a.checked_add(b) {
        println!("{}", c);
    } else {
        println!("Overflow!");
    }
}
