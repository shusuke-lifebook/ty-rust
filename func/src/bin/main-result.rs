use std::{num::ParseIntError, str::FromStr};

fn main() -> Result<(), ParseIntError> {
    let num_str = "123";
    // let num_str = "abc";
    let num = match i32::from_str(num_str) {
        Ok(num) => num,
        Err(e) => return Err(e),
    };
    println!("{num}");
    Ok(())
}
