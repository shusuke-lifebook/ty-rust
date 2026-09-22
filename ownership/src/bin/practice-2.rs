use std::io;

fn main() {
    let mut first_name = String::new();
    let mut last_name = String::new();
    get_full_name(&mut first_name, &mut last_name);
    println!(
        "あなたのお名前は「{} {}」ですね。",
        last_name.trim(),
        first_name.trim()
    );
}

fn get_full_name(first_name: &mut String, last_name: &mut String) {
    println!("姓を入力してください：");
    io::stdin().read_line(last_name).unwrap();
    println!("名を入力してください：");
    io::stdin().read_line(first_name).unwrap();
}
