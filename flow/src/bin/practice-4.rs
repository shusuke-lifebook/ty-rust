fn main() {
    let language = "Rust";
    let result =
        if language == "C++" || language == "C#" || language == "Java" || language == "Rust" {
            "コンパイラ言語"
        } else if language == "Python" || language == "Ruby" || language == "PHP" {
            "スクリプト言語"
        } else {
            "不明"
        };
    println!("{result}");
}
