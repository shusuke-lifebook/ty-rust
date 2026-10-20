fn main() {
    println!("絶対値： {}", (-3.14f64).abs());
    // println!("絶対値： {}", -3.14f64.abs()); // 正しい結果が得られない。
    println!("最大値： {}", 3.14f64.max(2.71));
    println!("最小値：{}", 3.14f64.min(2.71));
    println!("切り上げ：{}", 3.14f64.ceil());
    println!("切り下げ：{}", 3.14f64.floor());
    println!("四捨五入：{}", 3.14f64.round());
    println!("平方根：{}", 2.0f64.sqrt());
    println!("べき乗：{}", 2.0f64.powf(3.0));
    println!("符号：{}", 3.14f64.signum());
    println!("サイン：{}", 0.5f64.sin());
    println!("コサイン：{}", 0.5f64.cos());
    println!("タンジェント：{}", 0.5f64.tan());
    println!("自然対数：{}", 2.71828f64.ln());
    println!("常用対数：{}", 100.0f64.log10());
}
