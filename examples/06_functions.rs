//! Exercise 6: functions, expressions vs. statements, return values.
//! Run: cargo run --example 06_functions

fn main() {
    greet("Ferris");

    let sum = add(2, 3);
    println!("2 + 3 = {sum}");

    // Blocks are expressions: the last line with no semicolon is the value.
    let doubled = {
        let x = sum;
        x * 2
    };
    println!("doubled = {doubled}");

    println!("classify(4) = {}", classify(4));
    println!("classify(7) = {}", classify(7));
}

fn greet(name: &str) {
    println!("Hello, {name}!");
}

// Return type after `->`; the final expression (no semicolon) is returned.
fn add(a: i32, b: i32) -> i32 {
    a + b
}

// `if` is an expression too, so it can be the function's return value.
fn classify(n: i32) -> &'static str {
    if n % 2 == 0 { "even" } else { "odd" }
}
