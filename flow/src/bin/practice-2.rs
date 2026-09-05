use std::{env, str::FromStr};

fn main() {
    let args = env::args();
    for arg in args {
        if let Ok(i) = i32::from_str(arg.as_str()) {
            println!("{}", i * 2);
        }
    }
}
