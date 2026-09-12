type OperationFunc = fn(&str);

fn main() {
    let data = ["一富士", "二鷹", "三なすび"];
    let func: OperationFunc = add_bracket;
    array_walk(data, func);
}

fn array_walk(data: [&str; 3], output: OperationFunc) {
    for value in data {
        output(value);
    }
}

fn add_bracket(s: &str) {
    println!("[{s}]");
}
