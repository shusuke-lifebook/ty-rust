fn main() {
    let s = dangling_function();
    println!("こんにちは、{s}!");
}

// fn dangling_function() -> &String {
//     let s = String::from("Rust");
//     &s
// }

fn dangling_function() -> String {
    let s = String::from("Rust");
    s
}
