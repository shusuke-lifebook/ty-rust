fn main() {
    let data = "ローカルスコープ";
    {
        let data = "ブロックスコープ";
        println!("{data}");
    }
    println!("{data}");
}
