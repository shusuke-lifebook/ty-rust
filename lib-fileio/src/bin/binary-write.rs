use std::{fs::OpenOptions, io::Write};

fn main() {
    let filename = "binary-data.txt";
    let mut options = OpenOptions::new();
    let mut file = options
        .read(true)
        .write(true)
        .create(true)
        .open(filename)
        .unwrap();
    let mut buf: [u8; 10] = Default::default();
    for i in 0..10 {
        buf[i] = i as u8;
    }
    let bytes = file.write(&buf).unwrap();
    println!("{bytes}バイト書き込みました。");
}
