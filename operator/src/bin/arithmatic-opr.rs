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
    let mut float_b: f64;
    let mut float_c: f64;
    float_a = 0.7;
    float_b = 0.1;
    float_c = 10.0;
    println!(
        "({float_a} + {float_b}) * {float_c} = {}",
        ((float_a + float_b) * float_c).floor()
    );

    // 等しくならない比較
    println!("{}", 0.2 * 3.0 == 0.6);

    // 整数で進める浮動小数点演算
    println!("{}", (2 * 3) as f64 / 10.0 == 0.6);

    // 算術単項演算子
    int_a = -4;
    println!("-({int_a}) = {}", -int_a);

    // 算術単項演算子「-」を符号なし型に使った場合
    let uint_a: u32 = 4;
    // println!("-{uint_a} = {}", -uint_a); // コンパイルエラー

    // 算術単項演算子「+」を使った場合
    int_a = 4;
    // println!("+{int_a} = {}", +int_a); // コンパイルエラー

    // 浮動小数点数型の除算と剰余算
    float_a = 13.0;
    float_b = 5.0;
    println!("{float_a} / {float_b} = {}", float_a / float_b);
    println!("{float_a} % {float_b} = {}", float_a % float_b);
}
