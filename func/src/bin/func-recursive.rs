fn factorial(n: u64) -> u64 {
    if n == 1 { n } else { n * factorial(n - 1) }
}

fn main() {
    let number = 5;
    let result = factorial(5);
    println!("{number} の階乗は {result} です。");
}
