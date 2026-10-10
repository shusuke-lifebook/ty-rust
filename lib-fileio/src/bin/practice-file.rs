use std::{
    env,
    fs::OpenOptions,
    io::{BufWriter, Write},
};

fn main() {
    let filename = "data.txt";
    let mut options = OpenOptions::new();
    let file = options.create(true).append(true).open(filename).unwrap();
    let mut bw = BufWriter::new(file);
    let args = env::args();
    writeln!(bw, "{}", args.collect::<Vec<_>>().join(",")).unwrap();
}
