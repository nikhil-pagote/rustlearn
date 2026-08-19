//! Exercise 4: Tuples, Arrays, and Slices — compound types and views in Rust.
//! Run: cargo run --example 04_tuples_and_arrays
#![allow(unused)]

fn main() {
    tuple_demo();
    array_demo();
    slice_demo();
}

fn tuple_demo() {
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
        "tuples compare lexicographically: {a:#?} < {b:#?} = {}",
        a < b
    );
}

fn array_demo() {
    // An array groups values of the same type with a fixed length, allocated on the stack.
    let numbers: [i32; 5] = [1, 2, 3, 4, 5];
    println!("numbers = {numbers:?}");

    // Access elements by index (zero-based).
    println!("first element = {}", numbers[0]);
    println!("second element = {}", numbers[1]);

    // Array length.
    println!("array length = {}", numbers.len());

    // Initialize an array with the same value repeated: [value; length].
    let repeated: [i32; 5] = [0; 5];
    println!("repeated zeros = {repeated:?}");

    // Arrays are immutable by default, but can be mutable.
    let mut mutable_arr = [10, 20, 30];
    mutable_arr[0] = 99;
    println!("modified array = {mutable_arr:?}");

    // Iterating over an array.
    print!("iterating: ");
    for num in numbers {
        print!("{num} ");
    }
    println!();

    // Slicing an array (borrowing a portion of it).
    let slice: &[i32] = &numbers[1..4];
    println!("slice [1..4] = {slice:?}");

    // Safe access using .get() returns Option<&T> to avoid out-of-bounds panics.
    match numbers.get(10) {
        Some(val) => println!("found value: {val}"),
        None => println!("index 10 is out of bounds (handled safely)"),
    }
}

fn slice_demo() {
    // Slices are dynamically sized views (references) into a contiguous sequence.
    let numbers = [10, 20, 30, 40, 50, 60];

    // Range syntax for slicing:
    let middle: &[i32] = &numbers[1..4]; // elements at index 1, 2, 3 -> [20, 30, 40]
    let start_to_three: &[i32] = &numbers[..3]; // from start up to index 2 -> [10, 20, 30]
    let three_to_end: &[i32] = &numbers[3..]; // from index 3 to end -> [40, 50, 60]
    let entire: &[i32] = &numbers[..]; // whole array as a slice -> [10, 20, 30, 40, 50, 60]

    println!("middle slice: {middle:?}");
    println!("start to index 3: {start_to_three:?}");
    println!("index 3 to end: {three_to_end:?}");
    println!("entire slice len: {}", entire.len());

    // First and last elements from a slice (returns Option<&T>).
    println!("first element: {:?}", middle.first());
    println!("last element: {:?}", middle.last());

    // Mutable slice: modify underlying array through a borrowed slice.
    let mut data = [1, 2, 3, 4, 5];
    let slice_mut: &mut [i32] = &mut data[1..4];
    slice_mut[0] = 99; // changes data[1]
    println!("data after mutable slice edit: {data:?}");

    // Functions taking &[T] accept slices of arrays (and later Vecs).
    print_slice(&numbers[..3]);
}

fn print_slice(slice: &[i32]) {
    println!("print_slice received: {slice:?}");
}

/// Returns both the quotient and the remainder of integer division.
fn divide(dividend: i32, divisor: i32) -> (i32, i32) {
    (dividend / divisor, dividend % divisor)
}
