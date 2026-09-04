fn main() {
    let mut v = vec![1, 2, 3];
    while let Some(top) = v.pop() {
        println!("{top}を取り出しました");
    }
}
