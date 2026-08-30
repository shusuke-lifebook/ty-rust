use std::str::FromStr;

fn main() {
    let s = "123";
    match i32::from_str(s) {
        Ok(n) => println!("{}", n),
        Err(e) => println!("{}", e),
    }
}
