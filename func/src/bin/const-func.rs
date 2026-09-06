// コメントアウトを解除するとコンパイルエラーになります。
fn main() {
    const AAA: f64 = 1.0 + 12.0;
    const BBB: i64 = const_func();
    // const CCC: f64 = no_const_func();
}

const fn const_func() -> i64 {
    2 + 5
}

fn no_const_func() -> f64 {
    println!("const関数ではありません。");
    0.0
}
