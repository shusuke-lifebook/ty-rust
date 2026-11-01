fn main() {
    let numbers = vec![1, 2, 3, 4, 5];

    let double: Vec<_> = numbers.iter().map(|x| x * 2).collect();
    println!("{:?}", double);

    let filtered: Vec<_> = numbers.iter().filter(|&&x| x > 3).collect();
    println!("{:?}", filtered);

    let sum: i32 = numbers.iter().sum();
    println!("{sum}");

    let all_even = numbers.iter().all(|x| x % 2 == 0);
    println!("{all_even}");

    let any_four = numbers.iter().any(|x| *x == 4);
    println!("{any_four}");
}
