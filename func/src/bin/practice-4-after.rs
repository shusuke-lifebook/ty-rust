fn main() {
    let data = [3.14152, 1.41421, 2.71828];
    array_walk(data, |n| println!("{}", n.sqrt()));
}

fn array_walk(data: [f64; 3], output: impl Fn(f64)) {
    for value in data {
        output(value);
    }
}
