use std::{fs::OpenOptions, io::Read};

fn main() {
    let filename = "binary-data.txt";
    let mut options = OpenOptions::new();
    let mut file = options.read(true).open(filename).unwrap();
    let mut buf: [u8; 10] = Default::default();
    let bytes = file.read(&mut buf).unwrap();
    println!("{bytes} バイト読み込みました。");
    println!("読み込んだデータ：{:?}", &buf[..bytes]);
}
