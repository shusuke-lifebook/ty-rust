use std::fs;

fn main() {
    fs::copy("hashire_merosu.txt", "merosu.txt").unwrap();
}
