fn main() {
    let data = ["一富士", "二鷹", "三なすび"];
    let func: fn(&str) = add_bracket_and_count;
    array_walk(data, func);
}

fn array_walk(data: [&str; 3], output: fn(&str)) {
    for value in data {
        output(value);
    }
}

fn add_bracket_and_count(s: &str) {
    println!("[{s}:{}]", s.chars().count());
}
