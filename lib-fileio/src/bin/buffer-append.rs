use std::{
    fs::OpenOptions,
    io::{BufWriter, Write},
};

use chrono::Local;

fn main() {
    let filename = "data.log";
    let mut options = OpenOptions::new();
    let file = options.append(true).open(filename).unwrap();
    let mut bw = BufWriter::new(file);
    writeln!(bw, "{}", Local::now()).unwrap();
}
