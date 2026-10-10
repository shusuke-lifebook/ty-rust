use std::path::Path;

fn main() {
    let path_file = Path::new("src/bin/path-is.rs");
    let path_buf = path_file.to_path_buf();
    let path_buf_file = path_file.with_file_name("source.c");
    let path_buf_ext = path_file.with_extension("bak");
    println!("to_path_buf： {}", path_buf.as_path().to_str().unwrap());
    println!(
        "with_file_name： {}",
        path_buf_file.as_path().to_str().unwrap()
    );
    println!(
        "with_extension： {}",
        path_buf_ext.as_path().to_str().unwrap()
    );
}
