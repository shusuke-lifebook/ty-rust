use std::io;

fn main() {
    let mut sum = 0;
    println!("整数値を入力して下さい（正の数のみ。0で終了）。");
    loop {
        let mut num = String::new();
        io::stdin().read_line(&mut num).unwrap();
        num = num.replace("\n", "");
        //println!("{}", num);
        if let Ok(n) = (&num).parse::<i32>() {
            //println!("{}", n);
            if n == 0 {
                println!("合計値は {} です。", sum);
                break;
            }
            if n < 0 {
                println!("正の数を入力して下さい。");
                continue;
            }
            sum += n;
        } else {
            println!("数値でないようです。");
            continue;
        }
    }
}
