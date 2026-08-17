//! Exercise 9: enums, Option, match, if let.
//! Run: cargo run --example 09_enums_pattern_matching

#[derive(Debug)]
enum Shape {
    Circle { radius: f64 },
    Rectangle { width: f64, height: f64 },
}

fn area(shape: &Shape) -> f64 {
    match shape {
        Shape::Circle { radius } => std::f64::consts::PI * radius * radius,
        Shape::Rectangle { width, height } => width * height,
    }
}

fn main() {
    let shapes = [
        Shape::Circle { radius: 2.0 },
        Shape::Rectangle {
            width: 3.0,
            height: 4.0,
        },
    ];
    for shape in &shapes {
        println!("{shape:?} has area {:.2}", area(shape));
    }

    // `Option<T>` is Rust's null-safe alternative to a null pointer.
    let maybe_number: Option<i32> = Some(5);
    match maybe_number {
        Some(n) => println!("got {n}"),
        None => println!("got nothing"),
    }

    // `if let` is shorthand for a match with one interesting arm.
    let config: Option<&str> = None;
    if let Some(value) = config {
        println!("config = {value}");
    } else {
        println!("no config set, using default");
    }
}
