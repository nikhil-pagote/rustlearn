//! Exercise 7: Vec, String, HashMap.
//! Run: cargo run --example 07_collections

use std::collections::HashMap;

#[allow(clippy::vec_init_then_push)] // deliberately showing Vec::new() + push()
fn main() {
    // Vec: growable array.
    let mut numbers: Vec<i32> = Vec::new();
    numbers.push(10);
    numbers.push(20);
    numbers.push(30);
    println!("numbers = {numbers:?}");

    let sum: i32 = numbers.iter().sum();
    println!("sum = {sum}");

    // Indexing panics out of bounds; `.get()` returns an Option instead.
    match numbers.get(5) {
        Some(v) => println!("index 5 = {v}"),
        None => println!("index 5 is out of bounds"),
    }

    // String: growable, UTF-8 encoded text.
    let mut s = String::from("hello");
    s.push_str(", world");
    s.push('!');
    println!("s = {s}");

    // HashMap: key/value store.
    let mut scores = HashMap::new();
    scores.insert("Alice", 90);
    scores.insert("Bob", 85);

    // `.entry().or_insert()` updates in place without a separate lookup.
    *scores.entry("Alice").or_insert(0) += 5;

    for (name, score) in &scores {
        println!("{name}: {score}");
    }
}
