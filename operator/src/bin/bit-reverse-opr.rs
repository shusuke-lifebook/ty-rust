fn main() {
    let a: u8 = 0b10110101;
    println!("{:08b} ➡ {:08b}", a, a.reverse_bits());
}
