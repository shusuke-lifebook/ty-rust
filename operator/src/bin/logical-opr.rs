fn main() {
    // 整数のビット論理演算
    let a = 12;
    let b = 0b1010;
    println!("{0} ➡ {0:04b} & {1:04b} ➡ {2:04b} ➡ {2}", a, b, a & b);
    println!("{0} ➡ {0:04b} | {1:04b} ➡ {2:04b} ➡ {2}", a, b, a | b);
    println!("{0} ➡ {0:04b} ^ {1:04b} ➡ {2:04b} ➡ {2}", a, b, a ^ b);

    // ビット論理演算（論理値型）
    let x = true;
    let y = false;
    let z = true;
    println!("{x} & {y} ➡ {}", x & y);
    println!("{x} | {y} ➡ {}", x | y);
    println!("{x} ^ {z} ➡ {}", x ^ z);

    // 論理単項演算
    let m: i8 = 6;
    let n = true;
    println!("{0} ➡ !{0:b} ➡ {1:b} ➡ {1}", m, !m);
    println!("{0} ➡ !{0} ➡ {1}", n, !n);
}
