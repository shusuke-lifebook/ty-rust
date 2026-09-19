// コメントアウトを解除するとコンパイルエラーになります。
fn main() {
    let mut s = String::from("こんにちは");
    let r = &s;
    s.clear();
    // println!("{r}");
}
