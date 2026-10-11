use std::{fs, path::Path};

fn main() {
    let path = Path::new("sample.dir");
    let path_buf = path.to_path_buf();
    fs::create_dir(&path_buf).unwrap();
}
