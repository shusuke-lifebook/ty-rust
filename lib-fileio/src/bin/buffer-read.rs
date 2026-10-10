use std::{
    fs::File,
    io::{BufReader, Read},
};

fn main() {
    let filename = "sample.txt";
    let file = File::open(filename).unwrap();
    let mut br = BufReader::new(file);
    let mut s = String::new();
    let _ = br.read_to_string(&mut s).unwrap();
    println!("{s}");
}
