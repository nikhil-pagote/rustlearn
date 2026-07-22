//! Exercise 5: if/else, loop, while, for, match as control flow.
//! Run: cargo run --example 05_control_flow

fn main() {
    let n = 7;
    if n % 2 == 0 {
        println!("{n} is even");
    } else {
        println!("{n} is odd");
    }

    // `loop` repeats forever until `break`; `break <value>` returns a value.
    let mut counter = 0;
    let result = loop {
        counter += 1;
        if counter == 5 {
            break counter * 2;
        }
    };
    println!("loop result = {result}");

    let mut countdown = 3;
    while countdown > 0 {
        println!("countdown: {countdown}");
        countdown -= 1;
    }

    for i in 1..=3 {
        println!("for i = {i}");
    }

    let items = ["a", "b", "c"];
    for item in items.iter() {
        println!("item = {item}");
    }

    // `match` must be exhaustive; `_` catches anything not listed.
    let day = 3;
    let name = match day {
        1 => "Monday",
        2 => "Tuesday",
        3 => "Wednesday",
        _ => "some other day",
    };
    println!("day {day} is {name}");
}
