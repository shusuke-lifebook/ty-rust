fn main() {
    let data = ["一富士", "二鷹", "三なすび"];
    array_walk(data);
}

fn array_walk(data: [&str; 3]) {
    for value in data {
        println!("[{value}]");
    }
}
