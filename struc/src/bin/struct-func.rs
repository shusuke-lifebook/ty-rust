struct User {
    name: String,
    email: String,
    gender: char,
    birth: u16,
    is_active: bool,
    last_login: Option<String>,
}

impl User {
    fn new() -> User {
        User {
            name: String::new(),
            email: String::new(),
            gender: char::default(),
            birth: u16::default(),
            is_active: bool::default(),
            last_login: None,
        }
    }
    fn from_str(name: &str, birth: u16, gender: char) -> User {
        User {
            name: name.to_string(),
            email: String::new(),
            birth,
            gender,
            is_active: true,
            last_login: None,
        }
    }
}

fn main() {
    let empty = User::new();
    println!(
        "{}:{}:{}:{}:{}:{:?}",
        empty.name, empty.email, empty.gender, empty.birth, empty.is_active, empty.last_login
    );
}
