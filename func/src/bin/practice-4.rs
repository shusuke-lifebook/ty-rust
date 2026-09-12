fn main() {
    let data = [3.14152, 1.41421, 2.71828];
    array_walk(data, print_sqrt);
}

fn array_walk(data: [f64; 3], output: fn(n: f64)) {
    for value in data {
        output(value);
    }
}

fn print_sqrt(n: f64) {
    println!("{}", n.sqrt());
}
