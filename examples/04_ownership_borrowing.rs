//! Exercise 4: ownership, borrowing, references.
//! The concept that most distinguishes Rust from other languages — no
//! garbage collector, no manual free, checked at compile time.
//! Run: cargo run --example 04_ownership_borrowing

fn main() {
    // Move: String owns heap data. Assigning it moves ownership; `s1` is no
    // longer valid after this (uncomment the println! below to see the
    // compile error: "value borrowed here after move").
    let s1 = String::from("hello");
    let s2 = s1;
    // println!("{s1}"); // error[E0382]: borrow of moved value: `s1`
    println!("s2 = {s2}");

    // Clone: an explicit deep copy, if you actually want two owners.
    let s3 = s2.clone();
    println!("s2 = {s2}, s3 = {s3}");

    // Copy types (fixed-size, stack-only) don't move — they copy implicitly.
    let n1 = 5;
    let n2 = n1;
    println!("n1 = {n1}, n2 = {n2}");

    // Borrowing: `&s3` lends a reference without transferring ownership.
    let len = calculate_length(&s3);
    println!("'{s3}' has length {len}");

    // Mutable borrow: only one at a time, and not alongside any immutable one.
    let mut s4 = String::from("hello");
    append_world(&mut s4);
    println!("s4 = {s4}");
}

#[allow(clippy::ptr_arg)] // &String on purpose here; &str slices come later
fn calculate_length(s: &String) -> usize {
    s.len()
}

fn append_world(s: &mut String) {
    s.push_str(", world");
}
