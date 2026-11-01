fn main() {
    let vec1 = vec![1, 5, 5, 7, 7, 9];

    let vec: Vec<i64> = vec1.iter().map(|&x| x * 2).collect();
    for c in &vec {
        println!("{}", c);
    }

    // print_list(&vec);
}

fn print_list(list: &Vec<i32>) {
    let joined = list
        .iter()
        .map(|x| x.to_string())
        .collect::<Vec<_>>()
        .join(", ");
    println!("{}", joined)
}
