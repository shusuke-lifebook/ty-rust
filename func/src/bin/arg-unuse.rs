fn main() {
    func(100, 0);
    func2(100, 0);
}

fn func(x: i32, _y: i32) -> i32 {
    // x + y yを使っていたコード
    x
}

fn func2(x: i32, _: i32) -> i32 {
    x
}
