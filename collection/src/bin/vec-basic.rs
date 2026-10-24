fn main() {
    let mut vec1 = vec![1, 5, 7, 9];
    let vec2 = vec![2, 4, 6, 8];
    let vec3: Vec<f64> = Vec::new();

    vec1.insert(3, 11);
    vec1.push(13);

    for c in &vec1 {
        println!("{}", c);
    }

    println!("{}", vec1.pop().unwrap());
    println!("{}", vec1.len());
    println!("{}", vec3.is_empty());
    println!("{}", vec1[0]);
    println!("{}", vec1.contains(&1));
    println!("{}", vec1.remove(2));
    println!("{:?}", vec1);

    vec1.extend(vec2);
    println!("{:?}", vec1);

    vec1.reverse();
    println!("{:?}", vec1);

    vec1.sort();
    println!("{:?}", vec1);
}
