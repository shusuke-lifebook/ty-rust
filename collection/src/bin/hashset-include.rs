use std::collections::HashSet;

fn main() {
    let set1 = HashSet::from(["January", "February", "March", "April", "May"]);
    let set2 = HashSet::from(["February", "March", "May"]);
    //let set3 = HashSet::from(["January", "February", "March", "April", "May"]);

    println!("{}", set1.contains(&"March"));

    println!("{}", set1.is_superset(&set2));
    println!("{}", set2.is_subset(&set1));

    println!("{}", set1.is_disjoint(&set2));
}
