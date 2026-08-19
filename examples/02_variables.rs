//! Exercise 2: variables, mutability, shadowing, constants.
//! Run: cargo run --example 02_variables

const MAX_SCORE: u32 = 100;

fn main() {
    // Immutable by default.
    let x = 5;
    println!("x = {x}");
    //x += 1;   // error: cannot assign twice to immutable variable `x`
    println!("{0} x {0} = {1}", x, x * x);
    let x: i32 = 10; // type annotation
    println!("x = {x}");
    let x: bool = true; // type annotation, shadowing
    println!("x = {x}");

    // `mut` is required to reassign.
    let mut y = 10;
    y += 1;
    println!("y = {y}");

    // Shadowing: a new binding, can even change type, old one is gone.
    let spaces = "   ";
    let spaces = spaces.len();
    println!("spaces (as count) = {spaces}");

    // Constants: must have a type annotation, must be a compile-time value,
    // conventionally SCREAMING_SNAKE_CASE.
    println!("MAX_SCORE = {MAX_SCORE}");

    let v: Vec<_> = vec![1, 2, 3]; // type annotation
    println!("v = {:?}", v); //:? denotes debug formatting, prints the vector as [1, 2, 3]
    scalar_demo();
}

fn scalar_demo() {
    // Scalar types: integers, floating-point numbers, booleans, characters.

    //Integers: signed and unsigned, 8, 16, 32, 64, 128 bits. The default is i32.
    let _x: i8 = 0; // 8-bit signed integer the range: -2**(8-1) ~ 2**(8-1)-1 = -128..127
    let _x: i16 = 0; // 16-bit signed integer, the range: -2**(16-1) ~ 2**(16-1)-1 = -32768..32767
    let _x: i32 = 42; // 32-bit signed integer
    let _x: isize = 42; // pointer-sized signed integer, default 32-bit on 32-bit systems, 64-bit on 64-bit systems

    //Unsigned Integers: u8, u16, u32, u64, u128, usize. The default is u32.
    let _x: u8 = 0; // 8-bit unsigned integer, the range: 0..2**8-1 = 0..255
    let _x: u16 = 0; // 16-bit unsigned integer, the range: 0..2**16-1 = 0..65535
    let _x: u32 = 42; // 32-bit unsigned integer
    let _x: usize = 42; // pointer-sized unsigned integer, default 32-bit on 32-bit systems, 64-bit on 64-bit systems

    //Floats: 32-bit and 64-bit floating-point numbers. The default is f64.
    #[allow(clippy::approx_constant)]
    let _x: f64 = 3.14; // 64-bit floating-point number
    let _x: bool = true; // boolean
    let _x: char = 'R'; // character
}
