fn main() {
    println!("{}", calc_sum(1000));
}

fn calc_sum(mut x: i32) -> i32 {
    let mut r = 0;
    while x > 0 {
        r += x;
        if r > 10_000 {
            return r;
        }
        x -= 1;
    }
    r
}
