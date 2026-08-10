fn main() {
    let mut int_a: i32;
    let mut int_b: i32;

    // 基本的な算術演算
    int_a = 3;
    int_b = 4;
    println!("{int_a} + {int_b} = {}", int_a + int_b);

    int_a = 6;
    int_b = 2;
    println!("{int_a} - {int_b} = {}", int_a - int_b);

    int_a = 3;
    int_b = 2;
    println!("{int_a} * {int_b} = {}", int_a * int_b);

    int_a = 5;
    int_b = 2;
    println!("{int_a} / {int_b} = {}", int_a / int_b);
    int_a = 5;
    int_b = 2;
    println!("{int_a} % {int_b} = {}", int_a % int_b);

    // 異なるデータ型間での算術演算
    let mut float_a: f64;
    // float_a = 1 + 2.5; // コンパイルエラー
    float_a = 1.0 + 2.5;
    float_a = 1 as f64 + 2.5;

    // 浮動小数点数型の誤差を確認
}
