fn fibonacci(n: u32) -> u32 {
    println!("fibonacci({n}) を計算しています。");
    if n == 0 {
        0
    } else if n == 1 {
        1
    } else {
        fibonacci(n - 1) + fibonacci(n - 2)
    }
}

fn main() {
    let n = 20;
    let fib_result = fibonacci(n);
    println!("フィボナッチ数列の {n} 番目の値は {fib_result} です。");
}
