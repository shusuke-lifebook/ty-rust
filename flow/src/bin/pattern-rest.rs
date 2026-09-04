fn main() {
    let t = (1, 2, 3, 4, 5);
    match t {
        (first, .., last) => println!("先頭は{first}で末尾は{last}です。"),
    }
}
