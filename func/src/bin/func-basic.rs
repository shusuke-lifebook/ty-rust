fn add_integer(a: i32, b: i32) -> i32 {
    a + b
}

fn main() {
    let a = 100;
    let b = 200;
    let c = add_integer(a, b);
    println!("{a} + {b} = {c}");
}
