fn main() {
    // let x: !; // コンパイルエラー
    // x = diverges(); // コンパイルエラー
}

fn diverges() -> ! {
    panic!("この関数からは戻りません。");
}
