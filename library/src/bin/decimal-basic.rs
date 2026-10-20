use rust_decimal::{Decimal, dec};

fn main() {
    println!("{}", Decimal::MAX);
    println!("{}", Decimal::MIN);

    let number = dec!(0.1);
    println!("{number}");
    let number = Decimal::new(123, 2);
    println!("{number}");

    let float_a = dec!(0.7);
    let float_b = dec!(0.1);
    let float_c = dec!(10.0);
    println!(
        "({float_a} + {float_b}) * {float_c} = {}",
        ((float_a + float_b) * float_c).floor()
    );

    println!("{}", dec!(0.2) * dec!(3.0) == dec!(0.6));
}
