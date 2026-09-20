fn main() {
    let mut s = String::default();
    get_string(&mut s);
    println!("{s}");
}

fn get_string(s: &mut String) {
    *s = String::from("文字列をゲットしました！");
}
/*
fn main() {
    let mut s = String::default();
    s = get_string();
    println!("{s}");
}

fn get_string() -> String {
    String::from("文字列をゲットしました！")
}
*/
