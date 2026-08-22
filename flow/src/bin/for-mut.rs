fn main() {
    let scores = [100, 50, 65, 70, 80];

    for mut score in scores {
        score /= 2;
        println!("点数は{score}です。");
    }
}
