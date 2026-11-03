struct User {
    name: String,
    email: String,
    gender: char,
    birth: u16,
    is_active: bool,
    last_login: Option<String>,
}

impl User {
    const EXPLANTION: &str = "ユーザーの構造体です。";
    fn get_signature(&self) -> String {
        format!("{} <{}>", self.name, self.email)
    }
}

fn main() {
    println!("{}", User::EXPLANTION);
    let taro = User {
        name: String::from("Taro Yamada"),
        email: String::from("taro@example.com"),
        gender: 'm',
        birth: 1960,
        is_active: true,
        last_login: None,
    };
    println!("{} さんの署名は {} です。", taro.name, taro.get_signature());
}
