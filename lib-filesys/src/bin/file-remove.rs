use std::fs;

fn main() {
    let dir_name = "temp_dir";
    let filename = "temp_dir/hello.txt";
    fs::remove_dir_all(dir_name).unwrap();
    fs::create_dir(dir_name).unwrap();
    fs::write(filename, "Hello").unwrap();
    fs::remove_file(filename).unwrap();
    fs::remove_dir(dir_name).unwrap();
}
