use std::{
    fs::File,
    io::{BufRead, BufReader},
};

fn main() {
    let filename = "sample.txt";
    let file = File::open(filename).unwrap();
    let mut br = BufReader::new(file);
    let mut s = String::new();
    while br.read_line(&mut s).unwrap() > 0 {
        println!("{s}");
        s.clear();
    }
    //let mut s = Vec::new();
    //while br.read_until(b'\n', &mut s).unwrap() > 0 {
    //    print!("{s:?}");
    //    s.clear();
    //}
}
