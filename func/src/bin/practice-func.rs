fn main() {
    println!("{}", calc_bmi(65.0, 175.0));
}

fn calc_bmi(weight: f64, mut height: f64) -> f64 {
    height /= 100.0;
    height *= height;
    weight / height
}
