fn main() {
    let mut show = create_closure(100);
    show();
    show();
}

fn create_closure(init: i32) -> impl FnMut() {
    let mut value = init;
    move || {
        value += 1;
        println!("{value}");
    }
}
