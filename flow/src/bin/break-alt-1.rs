use std::io;
use std::str::FromStr;

fn main() {
    let mut sum = 0;
    println!("整数値を入力して下さい（空入力で終了）。");
    loop {
        let mut num = String::new();
        io::stdin().read_line(&mut num).unwrap();
        num = num.replace("\r", "");
        num = num.replace("\n", "");
        //println!("{num:?}");
        //println!("{}", i32::from_str(num.as_str()).unwrap());
        if let Ok(n) = i32::from_str(num.as_str()) {
            sum += n;
        } else {
            println!("合計値は {} です。", sum);
            break;
        }
    }
}
