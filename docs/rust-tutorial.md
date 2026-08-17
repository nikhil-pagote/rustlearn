# Rust Tutorial

A beginner-friendly tour of the language, following the exercises in
`examples/`. Every example here is verified to compile and run against this
repo's toolchain (rustc 1.97.1, edition 2024).

## Exercises

| # | File | Topic |
|---|---|---|
| 01 | [01_hello.rs](../examples/01_hello.rs) | `fn main()`, `println!` |
| 02 | [02_variables.rs](../examples/02_variables.rs) | `let`, `mut`, shadowing, `const` |
| 03 | [03_strings.rs](../examples/03_strings.rs) | `String` vs `&str`, common string methods |
| 04 | [04_ownership_borrowing.rs](../examples/04_ownership_borrowing.rs) | move semantics, `.clone()`, `&`/`&mut` references |
| 05 | [05_functions.rs](../examples/05_functions.rs) | function syntax, expressions vs. statements |
| 06 | [06_control_flow.rs](../examples/06_control_flow.rs) | `if`, `loop`, `while`, `for`, `match` |
| 07 | [07_structs.rs](../examples/07_structs.rs) | structs, `impl`, methods vs. associated functions |
| 08 | [08_enums_pattern_matching.rs](../examples/08_enums_pattern_matching.rs) | enums with data, `Option`, `match`, `if let` |
| 09 | [09_modules.rs](../examples/09_modules.rs) | modules, `use`, visibility, code organization |
| 10 | [10_collections.rs](../examples/10_collections.rs) | `Vec`, `String`, `HashMap` |
| 11 | [11_traits_generics.rs](../examples/11_traits_generics.rs) | traits, trait bounds, generic functions |
| 12 | [12_lifetimes.rs](../examples/12_lifetimes.rs) | lifetime annotations, borrowed structs, lifetime elision |
| 13 | [13_error_handling.rs](../examples/13_error_handling.rs) | `Result`, the `?` operator, panic vs. recoverable errors |
| 14 | [14_closures_iterators.rs](../examples/14_closures_iterators.rs) | closures, `map`/`filter`/`fold`/`collect` |
| 15 | [15_smart_pointers.rs](../examples/15_smart_pointers.rs) | `Box`, `Rc`, `RefCell`, recursive data |
| 16 | [16_testing.rs](../examples/16_testing.rs) | unit tests, assertions, `#[should_panic]` |
| 17 | [17_concurrency.rs](../examples/17_concurrency.rs) | threads, channels, `Arc`, `Mutex` |
| 18 | [18_advanced_pattern_matching.rs](../examples/18_advanced_pattern_matching.rs) | guards, destructuring, ranges, `@`, `while let` |
| 19 | [19_unsafe.rs](../examples/19_unsafe.rs) | raw pointers, unsafe functions, safe abstractions |
| 20 | [20_macros.rs](../examples/20_macros.rs) | `macro_rules!`, repetition, macro expansion |

Run any of them with:

```bash
cargo run --example NN_name   # e.g. cargo run --example 04_ownership_borrowing
```

## Why this order

Strings (03) come right after variables since `String`/`&str` show up in every
exercise from here on. Ownership (04) comes right after that and before
functions/control flow — it is the concept that most differs from other
languages. Structs and enums establish user-defined data before modules and
collections. Traits/generics (11) come before lifetimes (12), while error
handling (13) follows `Result`'s introduction through enums. Smart pointers,
testing, and concurrency then build on ownership, while the final exercises
cover increasingly advanced language features.

## What's deliberately different from the exact Rust Book examples

- `04_ownership_borrowing.rs`'s `calculate_length` takes `&String` rather than
  the more idiomatic `&str`, matching the Rust Book's ownership chapter
  exactly — clippy flags this (`ptr_arg`) and it's explicitly allowed there,
  since the point of that exercise is references, not slices.
- `10_collections.rs` builds a `Vec` with `Vec::new()` + `.push()` instead of
  the `vec![]` macro, to actually demonstrate the mutating API; clippy's
  `vec_init_then_push` is allowed there for the same reason.
- `14_closures_iterators.rs` uses `.fold()` to compute a product instead of the
  shorter `.product()`, since the exercise's point is `fold` as the general
  reduce tool; clippy's `unnecessary_fold` is allowed there.

See [language-notes.md](language-notes.md) for repo-specific gotchas and
[troubleshooting.md](troubleshooting.md) for errors actually hit while working
in this repo.
