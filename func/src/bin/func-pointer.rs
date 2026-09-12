fn func(x: i32) -> i32 {
    x * x
}

fn apply_twice(x: i32, f: fn(i32) -> i32) -> i32 {
    f(f(x))
}

fn main() {
    let f = apply_twice;
    println!("{}", f(3, func));
}
