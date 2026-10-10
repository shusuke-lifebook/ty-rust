use std::path::Path;

fn main() {
    let file_str = "src/bin/path-is.rs";
    let dir_str = "target";
    let path_file = Path::new(file_str);
    let path_dir = Path::new(dir_str);
    println!("{file_str} はファイル： {}", path_file.is_file());
    println!("{dir_str} はディレクトリ： {}", path_dir.is_dir());
    println!("{dir_str} は絶対パス： {}", path_dir.is_absolute());
    println!("{dir_str} は相対パス： {}", path_dir.is_relative());
    println!("{dir_str} はルートを持つ： {}", path_dir.has_root());
    println!("{file_str} は存在する： {}", path_file.exists());
    println!(
        "{file_str} は 'src' で始まる： {}",
        path_file.starts_with("src")
    );
    println!(
        "{file_str} は 'path_is.rs' で終わる： {}",
        path_file.ends_with("path_is.rs")
    );
}
