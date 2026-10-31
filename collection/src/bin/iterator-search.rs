fn main() {
    let vec1 = vec![1, 5, 5, 7, 7, 9];
    let vec2 = vec![2, 4, 6, 8];

    println!("{}", vec1.iter().all(|&x| x % 2 == 1));
    println!("{}", vec1.iter().any(|&x| x > 5));
    println!("{}", vec2.iter().sum::<i64>());
    println!("{}", vec2.iter().product::<i64>());
    println!("{}", vec1.iter().position(|&x| x == 5).unwrap());
    println!("{}", vec1.iter().rposition(|&x| x == 7).unwrap());
    println!("{}", vec1.iter().find(|&x| x == &7).unwrap());
    println!("{}", vec1.iter().fold(0, |a, x| a + x));
}
