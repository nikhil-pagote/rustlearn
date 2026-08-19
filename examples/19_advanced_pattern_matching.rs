//! Exercise 19: advanced pattern matching — guards, bindings, destructuring,
//! and the rest of the `match` toolbox beyond the basics from exercise 8.
//! Run: cargo run --example 19_advanced_pattern_matching

struct Point {
    x: i32,
    y: i32,
}

enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(u8, u8, u8),
}

fn describe(msg: &Message) -> String {
    match msg {
        Message::Quit => "quit".to_string(),
        // Destructure struct-variant fields directly in the pattern.
        Message::Move { x, y } => format!("move to ({x}, {y})"),
        // Match guards: an extra `if` condition on the arm.
        Message::Write(text) if text.is_empty() => "write nothing".to_string(),
        Message::Write(text) => format!("write {} chars", text.len()),
        // Ranges in tuple patterns.
        Message::ChangeColor(255, 0, 0) => "red".to_string(),
        Message::ChangeColor(r, g, b) => format!("rgb({r}, {g}, {b})"),
    }
}

fn main() {
    // Destructuring a struct and a tuple in `let` bindings.
    let p = Point { x: 3, y: -7 };
    let Point { x, y } = &p;
    println!("point = ({x}, {y})");

    let (name, score) = ("Ada", 42);
    println!("{name} scored {score}");

    // Match guards and ranges on a plain number.
    let n = 42;
    match n {
        0 => println!("zero"),
        1..=9 => println!("single digit"),
        n if n % 2 == 0 => println!("{n} is even and >= 10"),
        _ => println!("{n} is odd and >= 10"),
    }

    // `@` bindings: test a range AND bind the value at the same time.
    let age = 35;
    match age {
        a @ 0..=17 => println!("minor, age {a}"),
        a @ 18..=64 => println!("adult, age {a}"),
        a => println!("senior, age {a}"),
    }

    // Ignoring parts of a pattern with `_` and `..`.
    let pair = (10, 20, 30);
    match pair {
        (first, .., last) => println!("first = {first}, last = {last}"),
    }

    // `while let` and `for` patterns.
    let mut stack = vec![1, 2, 3];
    while let Some(top) = stack.pop() {
        println!("popped {top}");
    }

    for msg in [
        Message::Move { x: 1, y: 2 },
        Message::Write(String::new()),
        Message::Write(String::from("hello")),
        Message::ChangeColor(255, 0, 0),
        Message::ChangeColor(1, 2, 3),
        Message::Quit,
    ] {
        println!("{}", describe(&msg));
    }
}
