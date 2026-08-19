//! Exercise 12: traits (shared behavior) and generics (shared code).
//! Run: cargo run --example 12_traits_generics

trait Summary {
    fn summarize(&self) -> String;
}

struct Article {
    headline: String,
}

struct Tweet {
    username: String,
    text: String,
}

impl Summary for Article {
    fn summarize(&self) -> String {
        format!("Article: {}", self.headline)
    }
}

impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("@{}: {}", self.username, self.text)
    }
}

// Generic function, constrained by a trait bound: works for any `T` that
// implements `Summary`, resolved at compile time (monomorphization).
fn print_summary<T: Summary>(item: &T) {
    println!("{}", item.summarize());
}

// Generic over any type that implements `PartialOrd` (comparable).
fn largest<T: PartialOrd + Copy>(list: &[T]) -> T {
    let mut largest = list[0];
    for &item in list {
        if item > largest {
            largest = item;
        }
    }
    largest
}

fn main() {
    let article = Article {
        headline: String::from("Rust 2024 edition ships"),
    };
    let tweet = Tweet {
        username: String::from("rustlang"),
        text: String::from("borrow checker for the win"),
    };

    print_summary(&article);
    print_summary(&tweet);

    let numbers = vec![34, 50, 25, 100, 65];
    println!("largest number = {}", largest(&numbers));

    let chars = vec!['y', 'm', 'a', 'q'];
    println!("largest char = {}", largest(&chars));
}
