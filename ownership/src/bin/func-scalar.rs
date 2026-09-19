fn main() {
    let x = 1000;
    let y = double(x);
    println!("xは{x}、yは{y}です。");
}

fn double(a: i32) -> i32 {
    a * 2
}
