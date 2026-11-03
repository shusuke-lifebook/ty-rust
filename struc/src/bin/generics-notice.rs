struct MyList<T> {
    items: [T; 100],
    count: usize,
}

impl<T> MyList<T> {
    fn new() -> Self {
        MyList {
            items: [0; 100],
            count: 0,
        }
    }
}

fn main() {
    let list = MyList::new();
}
