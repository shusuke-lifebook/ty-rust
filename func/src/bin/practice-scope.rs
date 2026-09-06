const CONST_VALUE: i32 = 100;

fn func() {
    let a = 10;
    const CONST_VALUE: i32 = 200;
    println!("a in func: {a}");
    println!("CONST_VALUE in func: {CONST_VALUE}");
}

fn main() {
    let mut a = 1;
    let mut b = 99;
    {
        a = 2;
        let b = 123;
        func();
        println!("a in block: {a}");
        println!("b in block: {b}");
    }
    println!("a in main: {a}");
    println!("b in main: {b}");
    println!("CONST_VALUE in main: {CONST_VALUE}");
}
