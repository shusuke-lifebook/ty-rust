fn main() {
    let func: fn(&str) = run;
    func("メロス");
}

fn run(s: &str) {
    println!("走れ{s}");
}
