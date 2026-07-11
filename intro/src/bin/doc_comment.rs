//! ドキュメンテーションのサンプル
fn main() {
    add2(1, 1);
    println!("Hello, world!");
}

/// 2つの値を加算する関数
///
/// # Arguments
/// * `a` - 第1の数
/// * `b` - 第2の数
///
/// # Examples
/// ```
/// let a = 5;
/// leb b = 4;
/// assert_eq!(9, add2(a, b));
/// ```
pub fn add2(a: i32, b: i32) -> i32 {
    a + b
}
