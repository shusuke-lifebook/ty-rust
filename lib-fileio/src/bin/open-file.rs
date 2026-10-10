use std::{fs::File, io::Read};

fn main() {
    let filename = "hashire_merosu.txt";
    let mut file = File::open(filename).unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    println!("{contents}");
}
