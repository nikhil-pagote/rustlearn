---
name: new-exercise
description: Scaffold and verify a new numbered Rust exercise under examples/. Use when the user wants to add a new exercise, practice file, or example .rs program for a topic (e.g. "add an exercise for closures", "new exercise on error handling").
---

# new-exercise

Create the next runnable exercise in `examples/`, and verify it actually
compiles and runs before finishing.

## Steps

1. **Pick the number.** List `examples/` and use the next zero-padded number
   after the highest existing `NN_*.rs` (e.g. after `10_iterators.rs` → `11_`).
   Derive a short snake_case name from the topic: `NN_<topic>.rs`.

2. **Write the file** as a standalone `fn main()` program (cargo's `examples/`
   convention — no changes to `Cargo.toml` needed, cargo auto-discovers it).
   Start with a doc comment: what the exercise shows + the run command. Keep it
   focused and print results so running it is a visible check.

3. **Verify it runs** (do not skip):
   ```bash
   cargo run --example NN_<topic>
   ```
   Fix any compiler error or clippy warning and re-run until it passes cleanly.

4. **Link it (optional).** If it maps to a learning topic worth cross-referencing,
   add a row to the exercises index in `docs/README.md`.

5. **Report** the file path, what it demonstrates, and the actual output.
