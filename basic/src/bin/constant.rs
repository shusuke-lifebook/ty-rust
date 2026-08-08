fn main() {
    // 定数を使わない
    let price = 2000.0;
    let sum = price * 1.1;

    // 定数を使う
    const TAX_RATE: f64 = 1.1;
    let price = 2000.0;
    let sum = price * TAX_RATE;
    println!("{sum}")
}
