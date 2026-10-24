use std::collections::BTreeSet;

fn main() {
    let set = BTreeSet::from(["January", "February", "March", "April", "May"]);

    print_set(
        set.intersection(&BTreeSet::from([
            "January", "February", "May", "June", "July",
        ]))
        .cloned(),
    );
    print_set(
        set.difference(&BTreeSet::from(["January", "April", "July"]))
            .cloned(),
    );
    print_set(set.union(&BTreeSet::from(["May", "June", "July"])).cloned());
    print_set(
        set.symmetric_difference(&BTreeSet::from(["April", "May", "June", "July"]))
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
