fn main() {
    let a = 1;
    let b = -1;
    let s = if a > 0 {
        if b > 0 {
            "変数aは正数、変数bも整数です。"
        } else {
            "変数aは正数ですが、変数bは正数ではありません。"
        }
    } else {
        "変数aは正数ではありません。"
    };
    println!("{s}");
}
