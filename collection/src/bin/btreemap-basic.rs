use std::collections::BTreeMap;

fn main() {
    let mut map = BTreeMap::new();
    map.insert("Apple", "りんご");
    map.insert("Orange", "みかん");
    map.insert("Banana", "ばなな");

    println!("{}", map.len());
    println!("{}", map.contains_key(&"Apple"));

    println!("{}", map["Apple"]);
    println!("{}", map.get("Apple").unwrap());

    for key in map.keys() {
        println!("{key}: {}", map[key]);
    }

    let first = map.first_key_value().unwrap();
    println!("{}: {}", first.0, first.1);
    let last = map.last_key_value().unwrap();
    println!("{}: {}", last.0, last.1);
    println!("{:?}", map);

    let first = map.pop_first().unwrap();
    println!("{}: {}", first.0, first.1);
    let last = map.pop_last().unwrap();
    println!("{}: {}", last.0, last.1);
    map.insert("Avocado", "アボカド");
    println!("{:?}", map);
}
