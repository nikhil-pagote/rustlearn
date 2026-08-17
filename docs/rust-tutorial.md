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
| 07 | [07_collections.rs](../examples/07_collections.rs) | `Vec`, `String`, `HashMap` |
| 08 | [08_structs.rs](../examples/08_structs.rs) | structs, `impl`, methods vs. associated functions |
| 09 | [09_enums_pattern_matching.rs](../examples/09_enums_pattern_matching.rs) | enums with data, `Option`, `match`, `if let` |
| 10 | [10_traits_generics.rs](../examples/10_traits_generics.rs) | traits, trait bounds, generic functions |
| 11 | [11_error_handling.rs](../examples/11_error_handling.rs) | `Result`, the `?` operator, panic vs. recoverable errors |
| 12 | [12_closures_iterators.rs](../examples/12_closures_iterators.rs) | closures, `map`/`filter`/`fold`/`collect` |

Run any of them with:

```bash
cargo run --example NN_name   # e.g. cargo run --example 04_ownership_borrowing
```

## Why this order

Strings (03) come right after variables since `String`/`&str` show up in every
exercise from here on. Ownership (04) comes right after that and before
functions/control flow — it's the concept that most differs from other
languages, and every later exercise (structs, traits, collections) depends on
understanding move vs. borrow semantics. Enums with pattern matching (09) come
before traits/generics (10) because `Option`/`Result` — used pervasively from
exercise 11 onward — are themselves enums.

## What's deliberately different from the exact Rust Book examples

- `04_ownership_borrowing.rs`'s `calculate_length` takes `&String` rather than
  the more idiomatic `&str`, matching the Rust Book's ownership chapter
  exactly — clippy flags this (`ptr_arg`) and it's explicitly allowed there,
  since the point of that exercise is references, not slices.
- `07_collections.rs` builds a `Vec` with `Vec::new()` + `.push()` instead of
  the `vec![]` macro, to actually demonstrate the mutating API; clippy's
  `vec_init_then_push` is allowed there for the same reason.
- `12_closures_iterators.rs` uses `.fold()` to compute a product instead of the
  shorter `.product()`, since the exercise's point is `fold` as the general
  reduce tool; clippy's `unnecessary_fold` is allowed there.

See [language-notes.md](language-notes.md) for repo-specific gotchas and
[troubleshooting.md](troubleshooting.md) for errors actually hit while working
in this repo.
