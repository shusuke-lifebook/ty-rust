use std::collections::HashSet;

fn main() {
    let vec = Vec::<i32>::new();

    let set: HashSet<&str> = HashSet::from(["January", "February", "March"]);
    let set = HashSet::<&str>::from(["January", "February", "March"]);
}
