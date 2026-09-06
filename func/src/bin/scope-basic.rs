static SDATA: char = 'S';
const CDATA: char = 'C';

fn main() {
    println!("{SDATA}");
    println!("{CDATA}");
    println!("{}", show());
}

fn show() -> char {
    //static sdata: char = 'S';
    //const cdata: char = 'C';
    let data = 'L';
    data
}
