enum Shape {
    Circle(f64),
    Rectangle(f64, f64),
    Square(f64),
}

fn main() {
    let my_shape = Shape::Circle(2.0);
    let my_shape1 = Shape::Rectangle(2.0, 5.0);
    let my_shape2 = Shape::Square(2.0);

    println!("{}", area(my_shape));
    println!("{}", area(my_shape1));
    println!("{}", area(my_shape2));
}

fn area(shape: Shape) -> f64 {
    let area = match shape {
        Shape::Circle(r) => 3.14 * r * r,
        Shape::Rectangle(l, b) => l * b,
        Shape::Square(s) => s * s,
    };

    area
}
