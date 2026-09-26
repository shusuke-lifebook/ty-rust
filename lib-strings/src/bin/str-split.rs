fn main() {
    // split
    let str1 = "まぐろ,かつお,いわし".to_string();
    println!(
        "split： {}",
        str1.split(",").collect::<Vec<&str>>().join("&")
    );
    // splitn
    let str1 = "まぐろ,かつお,いわし".to_string();
    println!(
        "splitn： {}",
        str1.splitn(2, ",").collect::<Vec<&str>>().join("&")
    );
    // split_inclusive
    let str1 = "まぐろ\nかつお\nいわし".to_string();
    println!(
        "split_inclusive： {}",
        str1.split_inclusive("\n").collect::<Vec<&str>>().join("&")
    );
    // split_once
    let str1 = "fish=まぐろ=かつお".to_string();
    let tupple = str1.split_once("=").unwrap();
    println!("split_once：{} と {}", tupple.0, tupple.1);
    // split_terminator
    let str1 = "まぐろ\nかつお\nいわし\n".to_string();
    println!(
        "split_terminator： {}",
        str1.split_terminator("\n").collect::<Vec<&str>>().join("&")
    );
    // split_whitespace
    let str1 = "まぐろ　かつお　いわし".to_string();
    println!(
        "split_whitespace：{}",
        str1.split_whitespace().collect::<Vec<&str>>().join("＆")
    );
    // split_ascii_whitespace
    let str1 = "まぐろ かつお いわし".to_string();
    println!(
        "split_ascii_whitespace：{}",
        str1.split_ascii_whitespace()
            .collect::<Vec<&str>>()
            .join("＆")
    );
    // split_at
    let str1 = "まぐろ:かつお".to_string();
    let tupple = str1.split_at(9);
    println!("split_at：{} {}", tupple.0, tupple.1);
}
