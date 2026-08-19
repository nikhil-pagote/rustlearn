//! Exercise 14: Result, the `?` operator, panic vs. recoverable errors.
//! Run: cargo run --example 14_error_handling

use std::num::ParseIntError;

fn parse_and_double(input: &str) -> Result<i32, ParseIntError> {
    // `?` returns early with the Err if parsing fails, otherwise unwraps Ok.
    let n: i32 = input.parse()?;
    Ok(n * 2)
}

fn main() {
    match parse_and_double("21") {
        Ok(n) => println!("doubled = {n}"),
        Err(e) => println!("parse error: {e}"),
    }

    match parse_and_double("not a number") {
        Ok(n) => println!("doubled = {n}"),
        Err(e) => println!("parse error: {e}"),
    }

    // `unwrap_or` / `unwrap_or_else` provide a fallback instead of panicking.
    let fallback = parse_and_double("nope").unwrap_or(-1);
    println!("fallback = {fallback}");

    // Panics are for unrecoverable bugs, not expected failure paths — e.g.
    // an index that must be in range by construction. Left commented out
    // since it would abort the program:
    // let v = vec![1, 2, 3];
    // println!("{}", v[10]); // panics: index out of bounds
}
