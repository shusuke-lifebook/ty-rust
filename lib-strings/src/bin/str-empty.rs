fn main() {
    let str1 = "文字あり".to_string();
    println!("{}", str1.is_empty());
    let str2 = "".to_string();
    println!("{}", str2.is_empty());

    let str1 = "文字あり";
    println!("{}", str1.is_empty());
    let str2 = "";
    println!("{}", str2.is_empty());

    let str1 = "文字あり".to_string();
    println!("{}", str1.len() == 0);
    let str2 = "".to_string();
    println!("{}", str2.len() == 0);

    let str1 = "文字あり";
    println!("{}", str1.len() == 0);
    let str2 = "";
    println!("{}", str2.len() == 0);
}
