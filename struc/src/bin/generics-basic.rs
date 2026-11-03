struct MyList<T> {
    items: [T; 100],
    count: usize,
}

impl<T> MyList<T> {
    // 取得のメソッドを実装
    fn get(&self, index: usize) -> Option<&T> {
        if index < self.count {
            Some(&self.items[index])
        } else {
            None
        }
    }

    // 追加のメソッドを実装
    fn add(&mut self, item: T) -> Option<&T> {
        if self.count < 100 {
            self.items[self.count] = item;
            self.count += 1;
            Some(&self.items[self.count - 1])
        } else {
            None
        }
    }
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
    i32_list.add(42);
    f64_list.add(3.14);
    if let Some(value) = i32_list.get(0) {
        println!("Integer value: {value}");
    }
    if let Some(value) = f64_list.get(0) {
        println!("Float value: {value}");
    }
}
