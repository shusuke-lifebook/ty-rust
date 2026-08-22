fn main() {
    let mut sum = 0;
    let mut counter = 1;
    loop {
        if counter > 1000 {
            break;
        }
        sum += counter;
        counter += 1;
    }
    println!("合計値は{sum}です。");
}
