//! Exercise 15: closures and iterator adapters.
//! Rust's equivalent "expressive/advanced" showcase slot (the Mojo repo used
//! SIMD for this; Rust's day-to-day idiomatic power tool is iterators).
//! Run: cargo run --example 15_closures_iterators

fn main() {
    // Closures capture their environment; `add_n` borrows `n`.
    let n = 5;
    let add_n = |x: i32| x + n;
    println!("add_n(10) = {}", add_n(10));

    let numbers = [1, 2, 3, 4, 5, 6];

    // Iterator adapters are lazy until consumed (here, by `collect`).
    let evens_doubled: Vec<i32> = numbers
        .iter()
        .filter(|&&x| x % 2 == 0)
        .map(|&x| x * 2)
        .collect();
    println!("evens_doubled = {evens_doubled:?}");

    let total: i32 = numbers.iter().sum();
    let count_gt_3 = numbers.iter().filter(|&&x| x > 3).count();
    println!("total = {total}, count_gt_3 = {count_gt_3}");

    // `fold` reduces to a single value with an explicit accumulator.
    #[allow(clippy::unnecessary_fold)] // showing fold as the general tool; product() is next
    let product = numbers.iter().fold(1, |acc, &x| acc * x);
    println!("product = {product}");
}
