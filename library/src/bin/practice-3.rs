use std::fs::OpenOptions;
use std::io::Read;

fn main() {
    let filename = "image.png";
    let mut options = OpenOptions::new();
    let mut file = options.read(true).open(filename).unwrap();
    let mut buf: [u8; 16] = Default::default();
    for i in 0..16 {
        print!("{:02x}: ", i * 16);
        let bytes = file.read(&mut buf).unwrap();
        for j in 0..bytes {
            print!("{:02x} ", buf[j]);
        }
        println!();
    }
}
