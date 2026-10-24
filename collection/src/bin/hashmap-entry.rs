use std::collections::HashMap;

fn main() {
    let mut map = HashMap::from([
        ("Apple", "りんご"),
        ("Orange", "みかん"),
        ("Banana", "ばなな"),
    ]);

    map.entry("Orange").and_modify(|x| *x = "蜜柑");
    map.entry("Grape").insert_entry("ぶどう");
    map.entry("Pineapple").or_default();
    map.entry("Peach").or_insert("もも");
    map.entry("Parsimmon").or_insert_with(|| "かき");

    for (key, value) in &map {
        println!("{key}: {value}");
    }

    if let Some(v) = map.get_mut(&"Watermelon") {
        *v = "すいか";
    } else {
        map.insert("Watermelon", "すいか");
    }
}
