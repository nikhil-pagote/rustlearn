//! Exercise 1: hello world.
//! Run: cargo run --example 01_hello

fn main() {
    println!("Hello, World!");
    println!("Hello, {}", "World");
    println!("Hello, {name}", name = "World");
    println!("Hello, {}, {}", "World", "Rust");
    println!("Hello, {0}, {1}, {0}", "World", "Rust");
}
