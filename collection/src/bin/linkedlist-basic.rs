use std::collections::LinkedList;

fn main() {
    let mut list = LinkedList::from(["水曜", "木曜", "金曜"]);
    for weekday in &list {
        println!("{weekday}");
    }

    println!("{}", list.len());
    println!("{}", list.front().unwrap());
    println!("{}", list.back().unwrap());
    println!("{}", list.contains(&"水曜"));

    list.push_front("火曜");
    list.push_front("月曜");
    list.push_back("土曜");
    list.push_back("日曜");
    println!("{:?}", list);

    let sunday = list.pop_back().unwrap();
    list.push_front(sunday);
    println!("{:?}", list);
}
