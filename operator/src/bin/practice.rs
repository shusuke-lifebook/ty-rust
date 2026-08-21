fn main() {
    // 2.
    let mut a = 4;
    let mut b = 5;
    let mut c = -1;
    let mut d = 0;
    a += 2;
    b = c;
    c = d * a;
    d -= b;
    println!("{},{},{},{}", a, b, c, d);

    // 3.
    let x = 0;
    let y = 4;
    if x != 0 && y % x == 0 {
        println!("{} は {} で割り切れます。", y, x);
    }
}
