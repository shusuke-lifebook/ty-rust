use std::collections::BinaryHeap;

fn main() {
    let mut heap = BinaryHeap::new();
    heap.push("January");
    heap.push("Februray");
    heap.push("March");
    heap.push("April");

    for month in &heap {
        println!("{month}");
    }

    println!("{}", heap.len());
    println!("{}", heap.pop().unwrap());
    println!("{}", heap.peek().unwrap());
    println!("{}", heap.pop().unwrap());

    let vec = heap.into_sorted_vec();
    for month in vec {
        println!("{month}");
    }
}
