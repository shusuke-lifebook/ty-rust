use std::str::FromStr;

fn main() {
    // 整数文字列の数値への変換
    println!("{}", i32::from_str("123").unwrap());
    println!("{}", u32::from_str("456").unwrap());

    println!("{}", "123".parse::<i32>().unwrap());

    // 浮動小数点数文字列の数値への変換
    println!("{}", f32::from_str("1.414").unwrap());
    println!("{}", f64::from_str("1.4142e10").unwrap());
    println!("{}", f64::from_str("1.").unwrap());
    println!("{}", f64::from_str(".1").unwrap());
    println!("{}", f64::from_str("inf").unwrap());
    println!("{}", f64::from_str("NaN").unwrap());

    // その他の文字列の変換
    println!("{}", bool::from_str("true").unwrap());
    println!("{}", bool::from_str("false").unwrap());

    // 基数を指定した変換
    println!("{}", i32::from_str_radix("123", 10).unwrap());
    println!("{}", u32::from_str_radix("aaaa", 16).unwrap());
    println!("{}", u32::from_str_radix("777", 8).unwrap());
    println!("{}", u32::from_str_radix("1010101010", 2).unwrap());

    // 文字列への変換
    let s = i32::from_str("-123").unwrap().to_string();
    println!("{s}");

    // 参考：文字列リテラルからの変換
    println!("{}", "-123".parse::<i32>().unwrap());
    println!("{}", "456".parse::<u32>().unwrap());
    println!("{}", "3.14".parse::<f32>().unwrap());
    println!("{}", "1.4142e10".parse::<f64>().unwrap());
    println!("{}", "1.".parse::<f64>().unwrap());
    println!("{}", ".1".parse::<f64>().unwrap());
    println!("{}", "inf".parse::<f64>().unwrap());
    println!("{}", "NaN".parse::<f64>().unwrap());
    println!("{}", "true".parse::<bool>().unwrap());
    println!("{}", "false".parse::<bool>().unwrap());
}
