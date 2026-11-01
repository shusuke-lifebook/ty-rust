use std::collections::BTreeMap;

fn main() {
    let mut kuku = BTreeMap::new();
    for n in 1..=9 {
        for m in 1..=9 {
            kuku.insert(format!("{n} x {m}"), n * m);
        }
    }
    for (k, v) in &kuku {
        println!("{k} = {v}");
    }
}
