use std::io::{self, Write};

fn main() {
    let mut s = String::new();
    let stdin = io::stdin();
    let mut stdout = io::stdout();
    loop {
        write!(stdout, "> ").unwrap();
        stdout.flush().unwrap();
        if stdin.read_line(&mut s).unwrap() == 0 {
            break;
        } else {
            writeln!(stdout, "{}", s.trim()).unwrap();
            s.clear();
        }
    }
}
