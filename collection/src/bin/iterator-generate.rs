fn main() {
    let vec1 = vec![1, 5, 5, 7, 7, 9];
    let vec2 = vec![2, 4, 6, 8];
    let vec3 = vec![1, 2, 3, 4, 5, 6, 7, 8];
    let vec4 = vec!["100", "abc", "200", "xyz"];

    let vec = vec1.iter().enumerate();
    for (i, c) in vec {
        println!("{i}: {c}");
    }
    println!();

    vec1.iter().map(|x| x * 2).for_each(|x| println!("{x}"));
    println!();
    vec3.iter()
        .filter(|&&x| x % 2 == 0)
        .for_each(|x| println!("{x}"));
    println!();
    vec4.iter()
        .filter_map(|&x| x.parse::<i32>().ok())
        .for_each(|x| println!("{x}"));
    println!();
    vec4.iter()
        .filter_map(|&x| x.parse::<i32>().ok())
        .inspect(|x| println!("Parsed: {x}"))
        .map(|x| x * 2)
        .for_each(|x| println!("{x}"));
    println!();
    vec2.iter()
        .zip(vec4.iter())
        .for_each(|(x, y)| println!("{x} - {y}"));
    println!();
}
