fn main() {
    let a = [1, 2, 3, 4, 5];
    let s = &a[1..4];
    println!("スライスは {s:?} です。");
    println!("スライスの先頭は {:?} です。", s[0]);
}
