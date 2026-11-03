struct Point<T> {
    x: T,
    y: T,
}

impl<T> Point<T> {
    fn new(x: T, y: T) -> Self {
        Point { x, y }
    }

    fn get_x(&self) -> &T {
        &self.x
    }

    fn get_y(&self) -> &T {
        &self.y
    }
}

fn main() {
    let integer_point = Point::new(5, 10);
    let float_point = Point::new(1.5, 2.5);

    println!(
        "Integer Point: ({}, {})",
        integer_point.get_x(),
        integer_point.get_y()
    );
    println!(
        "Float Point: ({}, {})",
        float_point.get_x(),
        float_point.get_y()
    );
}
