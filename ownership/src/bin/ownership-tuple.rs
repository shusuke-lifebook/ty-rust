// コメントアウトを解除するとコンパイルエラーになります。
fn main() {
    let scalar_a = (1, 2, 3);
    let scalar_b = scalar_a;
    let sequence_a = (1, String::from("a"));
    let sequence_b = sequence_a;
    println!("スカラー型：{},{}", scalar_a.0, scalar_b.0);
    // println!("シーケンス型：{},{}", sequence_a.0, sequence_b.0);
}
