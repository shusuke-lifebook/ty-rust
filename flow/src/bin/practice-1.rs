fn main() {
    let mut sum = 0;
    let mut i = 100;
    while i <= 200 {
        if i % 2 != 0 {
            i += 1;
            continue;
        }
        sum += i;
        i += 1;
    }
    println!("{sum}");
}
