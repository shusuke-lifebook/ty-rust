use std::collections::{HashSet, LinkedList};

fn main() {
    let set = HashSet::from(["January", "February", "March"]);

    let vec = vec!["January", "February", "March"];
    let linkedlist: LinkedList<_> = vec.iter().collect();
}
