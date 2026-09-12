type FuncType = fn(i32) -> i32;

fn main() {
    let data = [3, 5, 7, 11, 13];
    array_scan(data, square);
    array_scan(data, double);
}

fn array_scan(data: [i32; 5], calc: FuncType) {
    for value in data {
        println!("{value} -> {}", calc(value));
    }
}

fn square(x: i32) -> i32 {
    x * x
}

fn double(x: i32) -> i32 {
    x * 2
}
