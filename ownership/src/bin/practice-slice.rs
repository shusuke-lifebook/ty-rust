fn main() {
    let numbers = [100, 200, 300, 400, 500];
    let slice = &numbers[1..4];
    println!("スライスは {:?} です。", slice);
}
