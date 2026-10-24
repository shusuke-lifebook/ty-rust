use std::collections::HashMap;

fn main() {
    let mut map = HashMap::from([
        ("Apple", "りんご"),
        ("Orange", "みかん"),
        ("Banana", "ばなな"),
    ]);

    println!("{}", map.len());
    println!("{}", map.contains_key(&"Apple"));

    println!("{}", map["Apple"]);
    println!("{}", map.get("Apple").unwrap());

    map.insert("Kiwi", "キウイ");

    for key in map.keys() {
        println!("{key}: {}", map[key]);
    }

    for value in map.values() {
        println!("{value}");
    }

    map.remove(&"Apple");
    for (key, value) in &map {
        println!("{key}: {value}");
    }
}
