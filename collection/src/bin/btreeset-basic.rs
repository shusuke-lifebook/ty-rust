use std::collections::BTreeSet;

fn main() {
    let mut set = BTreeSet::from(["January", "February", "March", "April"]);
    println!("{}", set.insert("April"));
    println!("{}", set.insert("May"));
    println!("{}", set.remove("February"));

    for month in &set {
        println!("{month}");
    }

    println!("{}", set.len());
    println!("{}", set.first().unwrap());
    println!("{}", set.last().unwrap());
}
