use std::ops::Range;

fn main() {
    let range1 = 1..4;
    let range2 = Range { start: 1, end: 4 };
    let a = [1, 2, 3, 4, 5];
    let s1 = &a[range1];
    let s2 = &a[range2];
    println!("スライス1は {s1:?} です。");
    println!("スライス2は {s2:?} です。");
}
