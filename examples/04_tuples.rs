//! Exercise 4: Tuples — grouping values of different types.
//! Run: cargo run --example 04_tuples

fn main() {
    // A tuple groups values of different types into one compound value.
    let person: (&str, u8, f64) = ("Alice", 30, 65.5);
    println!("person = {person:?}");

    // Access elements by index (note the dot-number syntax).
    println!("name = {}", person.0);
    println!("age = {}", person.1);
    println!("weight = {}", person.2);

    // Destructure a tuple into separate variables with a pattern.
    let (name, age, weight) = person;
    println!("destructured: {name} is {age} years old and weighs {weight} kg");

    // Tuples can nest, and destructuring can match the nesting.
    let point: ((i32, i32), &str) = ((3, -7), "origin-ish");
    let ((x, y), label) = point;
    println!("nested: {label} at x={x}, y={y}");

    // A tuple is a handy way to return multiple values from a function.
    let (quotient, remainder) = divide(10, 3);
    println!("10 / 3 = {quotient} remainder {remainder}");

    // Tuple with a single element needs a trailing comma to not be just parentheses.
    let single: (i32,) = (5,);
    println!("single-element tuple = {single:?}");

    // The empty tuple `()` is called "unit" — it's the default return value
    // of functions (and blocks) that return nothing.
    let unit: () = ();
    println!("unit value = {unit:?}");

    // Tuples implement common traits if all their elements do (up to 12 elements).
    let a = (1, "one");
    let b = (2, "two");
    println!(
        "tuples compare lexicographically: {a:?} < {b:?} = {}",
        a < b
    );
}

/// Returns both the quotient and the remainder of integer division.
fn divide(dividend: i32, divisor: i32) -> (i32, i32) {
    (dividend / divisor, dividend % divisor)
}
