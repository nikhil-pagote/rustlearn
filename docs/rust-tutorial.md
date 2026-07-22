# Rust Tutorial

A beginner-friendly tour of the language, following the exercises in
`examples/`. Every example here is verified to compile and run against this
repo's toolchain (rustc 1.97.1, edition 2024).

## Exercises

| # | File | Topic |
|---|---|---|
| 01 | [01_hello.rs](../examples/01_hello.rs) | `fn main()`, `println!` |
| 02 | [02_variables.rs](../examples/02_variables.rs) | `let`, `mut`, shadowing, `const` |
| 03 | [03_ownership_borrowing.rs](../examples/03_ownership_borrowing.rs) | move semantics, `.clone()`, `&`/`&mut` references |
| 04 | [04_functions.rs](../examples/04_functions.rs) | function syntax, expressions vs. statements |
| 05 | [05_control_flow.rs](../examples/05_control_flow.rs) | `if`, `loop`, `while`, `for`, `match` |
| 06 | [06_collections.rs](../examples/06_collections.rs) | `Vec`, `String`, `HashMap` |
| 07 | [07_structs.rs](../examples/07_structs.rs) | structs, `impl`, methods vs. associated functions |
| 08 | [08_enums_pattern_matching.rs](../examples/08_enums_pattern_matching.rs) | enums with data, `Option`, `match`, `if let` |
| 09 | [09_traits_generics.rs](../examples/09_traits_generics.rs) | traits, trait bounds, generic functions |
| 10 | [10_error_handling.rs](../examples/10_error_handling.rs) | `Result`, the `?` operator, panic vs. recoverable errors |
| 11 | [11_closures_iterators.rs](../examples/11_closures_iterators.rs) | closures, `map`/`filter`/`fold`/`collect` |

Run any of them with:

```bash
cargo run --example NN_name   # e.g. cargo run --example 03_ownership_borrowing
```

## Why this order

Ownership (03) comes right after variables and before functions/control flow —
it's the concept that most differs from other languages, and every later
exercise (structs, traits, collections) depends on understanding move vs.
borrow semantics. Enums with pattern matching (08) come before traits/generics
(09) because `Option`/`Result` — used pervasively from exercise 10 onward —
are themselves enums.

## What's deliberately different from the exact Rust Book examples

- `03_ownership_borrowing.rs`'s `calculate_length` takes `&String` rather than
  the more idiomatic `&str`, matching the Rust Book's ownership chapter
  exactly — clippy flags this (`ptr_arg`) and it's explicitly allowed there,
  since the point of that exercise is references, not slices.
- `06_collections.rs` builds a `Vec` with `Vec::new()` + `.push()` instead of
  the `vec![]` macro, to actually demonstrate the mutating API; clippy's
  `vec_init_then_push` is allowed there for the same reason.
- `11_closures_iterators.rs` uses `.fold()` to compute a product instead of the
  shorter `.product()`, since the exercise's point is `fold` as the general
  reduce tool; clippy's `unnecessary_fold` is allowed there.

See [language-notes.md](language-notes.md) for repo-specific gotchas and
[troubleshooting.md](troubleshooting.md) for errors actually hit while working
in this repo.
