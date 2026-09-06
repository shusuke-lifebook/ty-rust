// コメントを解除するとコンパイルエラーとなります。
fn main() {
    let mut f = add;
    println!("{}", f(123, 456));
    // f = sub;
    // println!("{}", f(123, 456));
    // f = mul;
    // println!("{}", f(123, 456));
}

fn add(x: i32, y: i32) -> i32 {
    x + y
}

fn sub(x: i32, y: i32) -> i32 {
    x - y
}

fn mul(x: i32, y: i32) -> i32 {
    x * y
}
