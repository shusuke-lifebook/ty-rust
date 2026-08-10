fn main() {
    const DISCOUNT: f64 = 0.9;
    let price = 1000;
    let net_price = (price as f64 * DISCOUNT) as i32;
    println!("値引き後の価格は{net_price}円です。");
    // 結果：値引き後の価格は900円です。
}
