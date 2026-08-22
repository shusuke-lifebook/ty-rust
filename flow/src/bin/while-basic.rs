fn main() {
    let val = 10;
    let mut count = 1;
    let mut sum = 0;
    while count <= val {
        sum += count;
        count += 1;
    }
    println!("{val}までの合計は{sum}です。");
}
