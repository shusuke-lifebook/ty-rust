fn main() {
    // メソッドを用いた除算と剰余算
    let mut int_a: i32;
    let mut int_b: i32;
    int_a = 123;
    int_b = 10;
    println!(
        "{int_a} / {int_b} = {}...{}",
        int_a.div_euclid(int_b),
        int_a.rem_euclid(int_b)
    );
    int_a = -123;
    int_b = 10;
    println!(
        "{int_a} / {int_b} = {}...{}",
        int_a.div_euclid(int_b),
        int_a.rem_euclid(int_b)
    );
    int_a = 123;
    int_b = -10;
    println!(
        "{int_a} / {int_b} = {}...{}",
        int_a.div_euclid(int_b),
        int_a.rem_euclid(int_b)
    );
    int_a = -123;
    int_b = -10;
    println!(
        "{int_a} / {int_b} = {}...{}",
        int_a.div_euclid(int_b),
        int_a.rem_euclid(int_b)
    );
}
