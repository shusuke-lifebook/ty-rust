fn main() {
    let mut a = [1, 2, 3, 4, 5];
    let s = &mut a[1..4];
    s[0] = 20;
    println!("スライスは {s:?} です。");
    println!("配列は {a:?} です。");
}
