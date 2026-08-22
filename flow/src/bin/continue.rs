fn main() {
    let mut sum = 0;
    let mut counter = 1;
    while counter <= 1000 {
        counter += 1;
        if counter % 2 == 0 {
            continue;
        }
        sum += counter;
    }
    println!("偶数を除いた合計値は{sum}です。");
}
