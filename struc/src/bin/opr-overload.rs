use std::ops::Add;

struct Point<T> {
    x: T,
    y: T,
}

impl<T> Add<Point<T>> for Point<T>
where
    T: Add<Output = T> + Copy,
{
    type Output = Point<T>;

    fn add(self, rhs: Point<T>) -> Self::Output {
        Point {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

fn main() {
    let a = Point { x: 10, y: 20 };
    let b = Point { x: 30, y: 40 };
    let c = a + b;
    println!("({}, {})", c.x, c.y);
}
