fn main() {
    let vec = Vec::from([
        "January".to_string(),
        "February".to_string(),
        "March".to_string(),
    ]);
    for month in &vec {
        println!("{month}");
    }
    println!();
}
