# Troubleshooting

Fixes for errors actually hit while working in this repo — not a general
Rust error reference (rustc's own error explanations, via `rustc --explain
E0382`, already cover the language-level errors well).

## clippy warnings hit while writing the exercises

`cargo clippy --all-targets` flagged four things while writing
`examples/*.rs`. Three were kept as deliberate teaching choices (allowed
explicitly with a reason comment, scoped as narrowly as possible); one was a
genuine simplification with no teaching value lost, so it was fixed instead.

| Lint | Where | Resolution |
|---|---|---|
| `vec_init_then_push` | `11_collections.rs` | Allowed on `fn main()` — the exercise's point is demonstrating `Vec::new()` + `.push()`. |
| `ptr_arg` (`&String` vs `&str`) | `05_ownership_borrowing.rs` | Allowed on the function — matches the Rust Book's ownership chapter exactly; slices are a later topic. |
| `unnecessary_fold` | `15_closures_iterators.rs` | Allowed on the `let` statement — the exercise's point is `fold` as the general reduce tool, not the shortest way to a product. |
| `useless_vec` | `15_closures_iterators.rs` | **Fixed** — switched `vec![1, 2, 3, 4, 5, 6]` to an array literal `[1, 2, 3, 4, 5, 6]`; no pedagogical loss since the exercise only iterates, never mutates. |

Lesson: when clippy conflicts with an exercise's teaching intent, `#[allow(...)]`
with a one-line reason comment, scoped to the smallest item possible (a
statement or function, not the whole file) — not a blanket `#![allow(...)]` at
the crate root.

## Nothing else hit yet

This section grows as real compiler/runtime errors come up. Add an entry with:
the exact error, what caused it, and the fix — not a hypothetical.
