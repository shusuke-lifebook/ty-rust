fn main() {
    let bmi = 30.0;
    let s = if bmi >= 18.0 {
        "普通体系です。"
    } else if bmi >= 25.0 {
        "肥満体系です。"
    } else {
        "痩せ体系です。"
    };
    println!("{s}");
}
