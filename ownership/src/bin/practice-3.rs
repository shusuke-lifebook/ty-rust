fn main() {
    let mut get_fibonacci = create_closure();
    for count in 1..=10 {
        println!("{count}: {}", get_fibonacci());
    }
}

fn create_closure() -> impl FnMut() -> u32 {
    let mut count = 0;
    let mut a = 0_u32;
    let mut b = 1_u32;
    move || {
        let number = if count == 0 {
            a
        } else {
            if count == 1 {
                b
            } else {
                let c = a + b;
                a = b;
                b = c;
                c
            }
        };
        count += 1;
        number
    }
}
