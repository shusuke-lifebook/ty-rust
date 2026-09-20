fn main() {
    let a = 100;
    let r: &i32;
    r = &a;
    println!("{:p}, {r:p}", &a);
}
