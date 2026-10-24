use std::collections::HashMap;

fn main() {
    let mut map = HashMap::from([
        ("Apple", "りんご"),
        ("Orange", "みかん"),
        ("Banana", "ばなな"),
    ]);

    map.insert("Orange", "蜜柑");
    map.insert("Peach", "もも");
    map.remove("Banana");
    for (key, value) in &map {
        println!("{key}: {value}");
    }
}
