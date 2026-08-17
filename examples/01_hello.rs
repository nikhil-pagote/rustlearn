//! Exercise 1: hello world.
//! cargo fmt -- examples/01_hello.rs
//! Run: cargo run --example 01_hello

//┌────────────────┬─────────────────────┬───────────────────────────────────────────────────────────────────┐
//│      Flag      │     Target kind     │                        Where it comes from                        │
//├────────────────┼─────────────────────┼───────────────────────────────────────────────────────────────────┤
//│ --bin NAME     │ a binary            │ src/bin/NAME.rs or [[bin]] in Cargo.toml                          │
//├────────────────┼─────────────────────┼───────────────────────────────────────────────────────────────────┤
//│ --example NAME │ an example program  │ examples/NAME.rs (cargo's convention, no Cargo.toml entry needed) │
//├────────────────┼─────────────────────┼───────────────────────────────────────────────────────────────────┤
//│ --test NAME    │ an integration test │ tests/NAME.rs                                                     │
//├────────────────┼─────────────────────┼───────────────────────────────────────────────────────────────────┤
//│ --bench NAME   │ a benchmark         │ benches/NAME.rs                                                   │
//├────────────────┼─────────────────────┼───────────────────────────────────────────────────────────────────┤
//│ --lib          │ the library target  │ src/lib.rs (no name needed, there's only one)                     │
//└────────────────┴─────────────────────┴───────────────────────────────────────────────────────────────────┘

fn main() {
    println!("Hello, World!");
    println!("Hello, {}", "World");
    println!("Hello, {name}", name = "World");
    println!("Hello, {}, {}", "World", "Rust");
    println!("Hello, {0}, {1}, {0}", "World", "Rust");
}
