use std::collections::VecDeque;

fn main() {
    let mut deque = VecDeque::new();
    deque.push_front("January");
    deque.push_front("February");
    deque.push_front("March");
    deque.push_front("April");

    for month in &deque {
        println!("{month}");
    }

    println!("{}", deque.len());
    println!("{}", deque.contains(&"February"));
    println!("{}", deque.pop_back().unwrap());
    println!("{}", deque.back().unwrap());
    println!("{}", deque.pop_back().unwrap());
}
