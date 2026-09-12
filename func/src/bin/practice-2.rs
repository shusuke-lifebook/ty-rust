fn swap(a: i32, b: i32) -> (i32, i32) {
    (b, a)
}

fn main() {
    let mut a = 20;
    let mut b = 40;
    (a, b) = swap(a, b);
    println!("{a}, {b}");
}
