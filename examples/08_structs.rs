//! Exercise 8: structs, methods (impl blocks), associated functions.
//! Run: cargo run --example 08_structs

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    // Associated function (no `self`) — called as `Rectangle::square(10)`.
    fn square(size: u32) -> Rectangle {
        Rectangle {
            width: size,
            height: size,
        }
    }

    // Method (`&self`) — called as `rect.area()`.
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

fn main() {
    let rect = Rectangle {
        width: 30,
        height: 50,
    };
    println!("rect = {rect:?}");
    println!("area = {}", rect.area());

    let sq = Rectangle::square(20);
    println!("can rect hold sq? {}", rect.can_hold(&sq));
}
