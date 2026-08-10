fn main() {
    // 異なる型の代入は拡大変換でもできない
    let i32_num: i32 = 100_i32;
    //let i64_num: i64 = i32_num;   // エラー

    // 値が収まると分かっていてもできない
    let i16_num = 100_i16;
    //let i8_num: i8 = i16_num;     // エラー

    // 型強制により代入できる
    let i32_num: i32 = 100;
    let i64_num: i64 = i32_num as i64;
    let i16_num = 100_i16;
    let i8_num: i8 = i16_num as i8;

    // 型強制：同一サイズの場合は値は変化せず解釈が変わる
    let u8_num: u8 = 128;
    let i8_num = u8_num as i8;
    println!("{}", i8_num);

    // 型強制：サイズが小さくなる場合には上位ビットが切り詰められる
    let u16_num: u16 = 400;
    let i8_num: i8 = u16_num as i8;
    println!("{}", i8_num);

    // 型強制：サイズが大きくなる場合には符号ビットが拡張される
    let i8_num: i8 = -127;
    let i16_num = i8_num as i16;
    println!("{}", i16_num);

    // 型強制：boolと整数間
    let bool_num = true;
    let i8_num = bool_num as i8;
    println!("{}", i8_num);
    let i8_num = 1;
    let bool_num = i8_num != 0;
    println!("{}", bool_num);

    // 型強制：浮動小数点数と整数間
    let f64_num = 3.14159;
    let i32_num = f64_num as i32;
    println!("{}", i32_num);
    let i32_num = 1;
    let f64_num = i32_num as f64;
    println!("{}", f64_num);
}
