fn main() {
    println!("{}", calc_sum(5, 8));
}

fn calc_sum(a: i32, b: i32) -> i32 {
    return a + b;
    // 以下のように式のみ記述しても良い
    // a + b
}
