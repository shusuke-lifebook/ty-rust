// コメントアウトを解除するとコンパイルエラーになります。
fn main() {
    let mut s = String::from("こんにちは");
    let r = &mut s;
    // print_string(&s);
    *r = String::from("さようなら");
    print_string(&s);
}

fn print_string(m: &String) {
    println!("「{m}」を受け取りました。");
}
