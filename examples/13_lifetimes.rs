//! Exercise 13: lifetimes — how the compiler checks that references stay valid.
//! Most lifetimes are inferred; you annotate them when a function or struct
//! relates the lifetimes of two or more references.
//! Run: cargo run --example 13_lifetimes

// The `<'a>` says: the returned reference lives at least as long as the
// shorter of the two inputs — so the caller can't outlive the data it
// points into.
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() { x } else { y }
}

// A struct holding a reference must declare a lifetime: an `Excerpt` can
// never outlive the string it borrows from.
struct Excerpt<'a> {
    text: &'a str,
}

impl<'a> Excerpt<'a> {
    fn first_word(&self) -> &str {
        // Elision: with `&self`, the output lifetime is inferred to match
        // the borrow of self — no annotation needed here.
        self.text.split_whitespace().next().unwrap_or("")
    }
}

// Lifetime elision: for `&str -> &str` with a single input, the compiler
// assumes the output borrows from the input. No `<'a>` required.
fn trim_both_ends(s: &str) -> &str {
    s.trim()
}

fn main() {
    let a = String::from("long string");
    let b = String::from("short");
    let result = longest(&a, &b);
    println!("longest = {result}");

    let text = String::from("Rust lifetimes keep references honest");
    let excerpt = Excerpt { text: &text };
    println!("first word = {}", excerpt.first_word());

    println!("trimmed = '{}'", trim_both_ends("  padded  "));

    // 'static is the lifetime of the whole program — string literals have it.
    let slogan: &'static str = "Fearless concurrency";
    println!("static str = {slogan}");

    // This would NOT compile — `result` would dangle after `short` drops:
    //
    // let result;
    // {
    //     let short = String::from("tiny");
    //     result = longest(&a, &short);
    // }                        // <- `short` dropped here
    // println!("{result}");    // error[E0597]: `short` does not live long enough
}
