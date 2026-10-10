use std::{
    fs::File,
    io::{BufRead, BufReader},
};

fn main() {
    let filename = "sample.txt";
    let file = File::open(filename).unwrap();
    let br = BufReader::new(file);
    for line in br.lines() {
        println!("{}", line.unwrap());
    }
}
