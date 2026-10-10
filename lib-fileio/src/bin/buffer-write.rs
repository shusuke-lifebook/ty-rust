use std::{
    fs::File,
    io::{BufWriter, Write},
};

use chrono::Local;

fn main() {
    let filename = "data.log";
    let file = File::create(filename).unwrap();
    let mut bw = BufWriter::new(file);
    writeln!(bw, "{}", Local::now()).unwrap();
}
