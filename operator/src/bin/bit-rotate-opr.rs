fn main() {
    // 左ローテート
    let i = 0b11110000_u8;
    println!("{:08b}", i);
    println!("{:08b}", i.rotate_left(2));

    // 右ローテート
    let i = 0b00111111_u8;
    println!("{:08b}", i);
    println!("{:08b}", i.rotate_right(2));
}
