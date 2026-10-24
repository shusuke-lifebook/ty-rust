use std::collections::HashSet;

fn main() {
    let set = HashSet::from(["January", "February", "March", "April", "May"]);

    print_set(
        set.intersection(&HashSet::from([
            "January", "February", "May", "June", "July",
        ]))
        .cloned(),
    );
    print_set(
        set.difference(&HashSet::from(["January", "April", "July"]))
            .cloned(),
    );
    print_set(set.union(&HashSet::from(["May", "June", "July"])).cloned());
    print_set(
        set.symmetric_difference(&HashSet::from(["April", "May", "June", "July"]))
            .cloned(),
    );
}

fn print_set<'a>(iter: impl IntoIterator<Item = &'a str>) {
    let joined = iter
        .into_iter()
        .map(|x| x.to_string())
        .collect::<Vec<_>>()
        .join(", ");
    println!("{}", joined);
}
