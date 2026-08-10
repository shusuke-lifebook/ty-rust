fn main() {
    let a = 100;
    let b = 200;
    // …さまざまな計算処理…
    println!("a:{:?}, b:{:?}", a, b);
    // …さまざまな計算処理…

    let a = 100;
    let b = dbg!(a * 2) + 100;
    dbg!(b);
}
