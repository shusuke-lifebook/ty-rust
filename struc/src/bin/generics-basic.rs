struct MyList<T> {
    items: [T; 100],
    count: usize,
}

fn main() {
    let mut i32_list = MyList {
        items: [0; 100],
        count: 0,
    };
    let mut f64_list = MyList {
        items: [0.0; 100],
        count: 0,
    };
}
