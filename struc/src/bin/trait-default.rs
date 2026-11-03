trait Plane {
    const Name: &str = "平面図形";
    fn calc_area(&self) -> f64 {
        0.0
    }
}

struct Triangle {
    base: f64,
    height: f64,
}

struct Rectangle {
    width: f64,
    height: f64,
}

struct Trapezoid {
    top_base: f64,
    bottom_base: f64,
    height: f64,
}

impl Plane for Triangle {
    const Name: &str = "三角形";
    fn calc_area(&self) -> f64 {
        (self.base * self.height) / 2.0
    }
}

impl Plane for Rectangle {
    const Name: &str = "四角形";
    fn calc_area(&self) -> f64 {
        self.width * self.height
    }
}

impl Plane for Trapezoid {}

fn main() {
    let triangle = Triangle {
        base: 3.0,
        height: 4.0,
    };
    let rectangle = Rectangle {
        width: 5.0,
        height: 6.0,
    };
    let trapezoid = Trapezoid {
        top_base: 3.0,
        bottom_base: 5.0,
        height: 4.0,
    };

    println!("{}の面積: {}", Triangle::Name, triangle.calc_area());
    println!("{}の面積: {}", Rectangle::Name, rectangle.calc_area());
    println!("{}の面積: {}", Trapezoid::Name, trapezoid.calc_area());
}
