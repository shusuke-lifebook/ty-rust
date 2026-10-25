fn main() {
    let vec = vec![1, 5, 5, 7, 7, 9];

    println!("{}", vec.iter().last().unwrap());
    let mut itr = vec.iter();
    println!("{}", itr.nth(0).unwrap());
    println!("{}", itr.nth(0).unwrap());
    println!("{}", itr.count());
    vec.iter().for_each(|&x| println!("{x}"));
}
