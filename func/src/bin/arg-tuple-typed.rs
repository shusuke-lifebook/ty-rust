type Coordinate = (i32, i32, i32, i32);

fn main() {
    let source = (0, 0, 100, 200);
    let dest = (200, 300, 100, 200);
    draw_image(source, dest);
}

fn draw_image(source: Coordinate, dest: Coordinate) {
    println!("{:?} to {:?}", source, dest);
}
