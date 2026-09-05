fn main() {
    let language = "Rust";
    let result = match language {
        "C++" | "C#" | "Java" | "Rust" => "コンパイラー言語",
        "Python" | "Ruby" | "PHP" => "スクリプト言語",
        _ => "不明",
    };
    println!("{result}");
}
