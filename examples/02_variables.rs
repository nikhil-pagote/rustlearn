//! Exercise 2: variables, mutability, shadowing, constants.
//! Run: cargo run --example 02_variables

const MAX_SCORE: u32 = 100;

fn main() {
    // Immutable by default.
    let x = 5;
    println!("x = {x}");

    // `mut` is required to reassign.
    let mut y = 10;
    y += 1;
    println!("y = {y}");

    // Shadowing: a new binding, can even change type, old one is gone.
    let spaces = "   ";
    let spaces = spaces.len();
    println!("spaces (as count) = {spaces}");

    // Constants: must have a type annotation, must be a compile-time value,
    // conventionally SCREAMING_SNAKE_CASE.
    println!("MAX_SCORE = {MAX_SCORE}");
}
