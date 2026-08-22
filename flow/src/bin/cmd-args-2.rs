use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    println!("{}", args[3]);
    println!("{}", args[2]);
    println!("{}", args[1]);
    println!("{}", args[0]);
}
