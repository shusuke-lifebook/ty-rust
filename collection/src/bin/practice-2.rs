fn main() {
    let mut vec = vec![1, 2, 3, 4];
    vec[2] = 50;
    vec.insert(1, 5);
    vec.remove(4);
    for c in &vec {
        println!("{c}");
    }
}
