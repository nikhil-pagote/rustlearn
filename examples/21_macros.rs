//! Exercise 21: macros — code that writes code at compile time.
//! You've used macros since exercise 1 (`println!`, `vec!` — note the `!`).
//! Here we write our own with `macro_rules!`, Rust's declarative macros.
//! Run: cargo run --example 21_macros

// macro_rules! matches patterns against the *source code* passed in and
// expands to the template on the right. `$( ... ),*` means "comma-separated
// repetition"; `$e:expr` captures any expression.
macro_rules! my_vec {
    ($($x:expr),*) => {
        {
            let mut temp = Vec::new();
            $(temp.push($x);)*
            temp
        }
    };
}

// Multiple arms make a macro behave differently per input shape — like a
// match statement over syntax.
macro_rules! describe {
    ($n:expr) => {
        format!("value is {}", $n)
    };
    ($n:expr, $unit:expr) => {
        format!("value is {} {}", $n, $unit)
    };
}

// Macros shine when a function can't do the job: variadic arguments,
// compile-time stringification, or capturing file/line info.
macro_rules! log_expr {
    ($e:expr) => {
        // stringify! turns tokens into a string literal without evaluating.
        println!("{} = {}", stringify!($e), $e)
    };
}

fn main() {
    // Built-in macros you've already met, with a fresh look at what they do.
    let v = vec![1, 2, 3];
    println!("vec! made {v:?}");
    println!("formatted with {}", format_args!("args and macros"));

    // Our vec-like macro: any number of elements, unlike a fixed-arity fn.
    let a = my_vec![1, 2, 3];
    let b = my_vec!["just", "one"];
    println!("my_vec! = {a:?}, {b:?}");

    // The multi-arm macro picks an expansion based on how it's called.
    println!("{}", describe!(42));
    println!("{}", describe!(42, "km/h"));

    // stringify! sees the source tokens — something no function can access.
    log_expr!(2 + 3);
    log_expr!(v.len());

    // Derived macros are the other big family: #[derive(Debug)] on a struct
    // generates an impl automatically — that's a *procedural* macro, defined
    // in a separate proc-macro crate, not with macro_rules!.
}
