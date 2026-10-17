fn main() {
    println!("絶対値： {}", (-5i32).abs());
    println!("べき乗： {}", (-5i32).pow(2));
    println!("平方根： {}", 8i32.isqrt());

    // 桁あふれチェック
    let a = 100i8;
    let b = 30i8;
    if let Some(result) = a.checked_add(b) {
        println!("{a} + {b} = {result}");
    } else {
        println!("{a} + {b} はオーバーフローしました。");
    }

    println!("絶対値： {}", -5i32.abs());
}
