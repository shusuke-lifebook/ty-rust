use rust_decimal::dec;

fn main() {
    println!("絶対値：{}", dec!(-3.14).abs());
    println!("最大値：{}", dec!(3.14).max(dec!(2.71)));
    println!("最小値：{}", dec!(3.14).min(dec!(2.71)));
    println!("切り上げ：{}", dec!(3.14).ceil());
    println!("切り下げ：{}", dec!(3.14).floor());
    println!("四捨五入：{}", dec!(3.14).round());
}
