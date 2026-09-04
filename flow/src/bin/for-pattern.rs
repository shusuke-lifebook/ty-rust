fn main() {
    let scores = [100, 50, 65, 70, 80];

    for (index, score) in scores.iter().enumerate() {
        println!("{} 番目の点数は {score} 点です。", index + 1);
    }
}
