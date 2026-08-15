use std::str::FromStr;

fn main() {
    // コンパイル時にゼロ除算を検出
    let a = 100;
    let b = 0;
    // let c = a / b; // コンパイルエラー

    // 実行時にゼロ除算でpanic
    let s = "0";
    let d = i32::from_str(s).unwrap();
    // let r = 10 / d; // 実行時にpanic

    // 浮動小数点によるゼロ除算
    let f = 10.0;
    let g = 0.0;
    println!("{f} / {g} = {}", f / g);
    println!("{f} % {g} = {}", f % g);
}
