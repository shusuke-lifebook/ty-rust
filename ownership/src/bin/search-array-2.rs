fn main() {
    let mut a = [1, 2, 3, 4, 5];
    let n = 3;
    let pos = search_position(&a, n);
    a[pos] = 30;
    println!("{n} は {a:?} の {} 番目にあります。", pos + 1);
}

fn search_position(a: &[i32; 5], n: i32) -> usize {
    let pos = a.into_iter().position(|&x| x == n).unwrap();
    pos
}
