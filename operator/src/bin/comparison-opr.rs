fn main() {
    // 比較演算
    let a = 1;
    let b = 2;
    println!("a == b: {}", a == b);
    println!("a != b: {}", a != b);
    println!("a > b: {}", a > b);
    println!("a < b: {}", a < b);
    println!("a >= b: {}", a >= b);
    println!("a <= b: {}", a <= b);

    // さまざまなデータ型に対する比較演算
    println!("{}", 1 < 2);
    println!("{}", 10.0 <= 200.0);
    println!("{}", 'a' > 'Z');
    println!("{}", "ABC" <= "abc");

    // 配列型の比較
    let a1 = [0, 1, 3, 5, 7, 9];
    let a2 = [0, 10, 8, 6, 4, 2];
    println!("等しい? {}", a1 == a2);
    println!("等しくない? {}", a1 != a2);
    println!("大きい? {}", a1 > a2);
    println!("小さい? {}", a1 < a2);
    println!("等しいか大きい? {}", a1 >= a2);
    println!("等しいか小さい? {}", a1 <= a2);

    // タプル型の比較
    let t1 = (1, 2.0, 'a');
    let t2 = (2, 1.0, 'z');
    println!("等しい? {}", t1 == t2);
    println!("等しくない? {}", t1 != t2);
    println!("大きい? {}", t1 > t2);
    println!("小さい? {}", t1 < t2);
    println!("等しいか大きい? {}", t1 >= t2);
    println!("等しいか小さい? {}", t1 <= t2);
}
