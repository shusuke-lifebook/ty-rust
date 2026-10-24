use std::collections::HashSet;

fn main() {
    let mut set = HashSet::from(["January", "February", "March", "April"]);
    println!("{}", set.insert("April"));
    println!("{}", set.insert("May"));
    println!("{}", set.remove(&"February"));
    for month in &set {
        println!("{month}");
    }
    println!("{}", set.len());
}
