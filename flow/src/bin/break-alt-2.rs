use std::io;

fn main() {
    let mut sum = 0;
    println!("整数値を入力して下さい（0で終了）。");
    let res = loop {
        let mut num = String::new();
        io::stdin().read_line(&mut num).unwrap();
        num = num.replace("\n", "");
        //println!("{}", num);
        if let Ok(n) = (&num).parse::<i32>() {
            //println!("{}", n);
            if n == 0 {
                break sum;
            }
            sum += n;
        }
    };
    println!("合計値は {} です。", res);
}
