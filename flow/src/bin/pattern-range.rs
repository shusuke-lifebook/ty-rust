fn main() {
    let bmi = 20.0;
    let judge = match bmi {
        ..18.0 => "痩せ体型",
        18.0..25.0 => "普通体型",
        25.0.. => "肥満体型",
        _ => "その他",
    };
    println!("{bmi} は {judge} です。");
}
