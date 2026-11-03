struct Debugger;

impl Debugger {
    fn log(&self, message: &str) {
        println!("[LOG] {message}")
    }
}

fn main() {
    let dbg = Debugger;
    dbg.log("main関数が始まりました。");
}
