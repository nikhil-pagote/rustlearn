//! Exercise 17: testing — unit tests, assertions, expected failures.
//! `cargo test` compiles each target in test mode and runs every `#[test]`.
//! Run the program:  cargo run --example 17_testing
//! Run the tests:    cargo test --example 17_testing
//! (In a library you'd keep tests next to the code in src/, plus integration
//! tests under tests/ — same attribute syntax.)

fn add(a: i32, b: i32) -> i32 {
    a + b
}

fn divide(a: f64, b: f64) -> Result<f64, String> {
    if b == 0.0 {
        Err(String::from("division by zero"))
    } else {
        Ok(a / b)
    }
}

fn main() {
    println!("add(2, 3) = {}", add(2, 3));
    println!("divide(10.0, 4.0) = {:?}", divide(10.0, 4.0));
    println!("run `cargo test --example 17_testing` to execute the tests below");
}

// `#[cfg(test)]` means this module only compiles under `cargo test`.
#[cfg(test)]
mod tests {
    // `use super::*` pulls the functions being tested into scope.
    use super::*;

    #[test]
    fn test_add() {
        assert_eq!(add(2, 3), 5);
    }

    #[test]
    fn test_add_negative() {
        assert_eq!(add(-2, -3), -5);
        // assert!(cond, "message") adds context when it fails.
        assert!(add(0, 0) == 0, "adding zeros should give zero");
    }

    #[test]
    fn test_divide_ok() {
        // assert_eq! needs PartialEq + Debug on both sides.
        assert_eq!(divide(10.0, 4.0), Ok(2.5));
    }

    #[test]
    fn test_divide_by_zero() {
        // assert! / assert_ne! are the other common assertions.
        assert!(divide(1.0, 0.0).is_err());
    }

    // `#[should_panic]` asserts the test panics — used for `panic!` paths.
    #[test]
    #[should_panic(expected = "index out of bounds")]
    fn test_out_of_bounds_panics() {
        let v = vec![1, 2, 3];
        let _ = v[10]; // panics: index out of bounds
    }
}
