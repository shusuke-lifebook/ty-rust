use std::collections::VecDeque;

fn main() {
    let mut deque = VecDeque::new();
    deque.push_front("Mercury");
    deque.push_front("Venus");
    deque.push_front("Earth");

    for planet in &deque {
        println!("{planet}");
    }

    println!("{}", deque.front().unwrap());
    println!("{}", deque.get(1).unwrap());
    println!("{}", deque.pop_front().unwrap());
    deque.push_back("Mars");
    println!("{}", deque.back().unwrap());
}
