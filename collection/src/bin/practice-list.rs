fn main() {
    let mut vec = vec!["Monday", "Friday", "Tuesday", "Saturday"];
    vec[3] = "Thursday";
    vec.remove(1);
    vec.insert(2, "Wednesday");
    for day in &vec {
        println!("{day}");
    }
}
