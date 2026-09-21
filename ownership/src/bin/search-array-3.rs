// コメントアウトを解除するとコンパイルエラーになります。

fn main() {
    let mut a = [1, 2, 3, 4, 5];
    let n = 3;
    let pos = search_position(&a, n);
    // a[2] = 30;
    println!("{pos:?} が {a:?} に見つかりました。");
}

fn search_position(a: &[i32; 5], n: i32) -> &[i32] {
    let pos = a.iter().position(|&x| x == n).unwrap();
    &a[pos..pos + 1]
}
