fn main() {
    let data = ["一富士", "二鷹", "三なすび"];
    array_walk(data, |s| println!("[{s}]"));
    array_walk(data, |s| println!("[{s}]:{}", s.chars().count()));
}

fn array_walk(data: [&str; 3], output: impl Fn(&str)) {
    for value in data {
        output(value);
    }
}
