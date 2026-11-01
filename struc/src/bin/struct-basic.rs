struct User {
    name: String,
    email: String,
    gender: char,
    birth: u16,
    is_active: bool,
    last_login: Option<String>,
}

fn main() {
    let taro = User {
        name: String::from("Taro Yamada"),
        email: String::from("taro@example.com"),
        gender: 'm',
        birth: 1960,
        is_active: true,
        last_login: None,
    };

    let hanako = create_user("Hanako Yamada".to_string(), 1965, 'f');

    let someone = User {
        name: "Some Person".to_string(),
        email: String::new(),
        birth: 1945,
        gender: 'm',
        is_active: true,
        last_login: None,
    };

    let jiro = User {
        name: "Jiro Tanaka".to_string(),
        birth: 1920,
        ..someone
    };

    println!("名前は {} です。", taro.name);

    let mut saburo = User {
        name: "Saburo Suzuki".to_string(),
        email: String::new(),
        birth: 1970,
        gender: 'm',
        is_active: false,
        last_login: None,
    };

    saburo.gender = 'f';
    println!("性別は {} です。", saburo.gender);
}

fn create_user(name: String, birth: u16, gender: char) -> User {
    User {
        name: name,
        // name,　// 初期化省略記法
        email: String::new(),
        birth: birth,
        // birth,　// 初期化省略記法
        gender: gender,
        // gender,　// 初期化省略記法
        is_active: true,
        last_login: None,
    }
}
