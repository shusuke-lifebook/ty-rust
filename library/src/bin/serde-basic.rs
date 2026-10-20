use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
struct User {
    name: String,
    age: u32,
}

fn main() {
    let user = User {
        name: "Toshi".to_string(),
        age: 25,
    };
    let json = serde_json::to_string(&user).unwrap();
    println!("JSON: {json}");

    let deserialized_user: User = serde_json::from_str(&json).unwrap();
    println!("Deserialized User: {:?}", deserialized_user);
}
