fn main() {
    println!("BMIは {} です。", calc_bmi(70.0, 1.75));
}

fn calc_bmi(weight: f64, mut height: f64) -> f64 {
    height *= 2.0;
    weight / height
}
