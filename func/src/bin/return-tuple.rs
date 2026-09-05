fn main() {
    let (d, r) = calc_div_rem(5, 2);
    println!("Div: {d}, Rem: {r}");
}

fn calc_div_rem(a: i32, b: i32) -> (i32, i32) {
    (a / b, a % b)
}
