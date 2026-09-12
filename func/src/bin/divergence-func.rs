fn main() {
    diverges();
    println!("main関数を終了します。");
}

fn diverges() {
    panic!("この関数からは戻りません。");
}
