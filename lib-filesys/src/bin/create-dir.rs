use std::fs;

fn main() {
    fs::create_dir("temp_dir").unwrap();
    fs::create_dir_all("temp_dir/one/two/three").unwrap();
}
