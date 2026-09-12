fn main() {
    let show = create_closure();
    show(100);
}

fn create_closure() -> impl Fn(i32) {
    |x| println!("{x}")
}
