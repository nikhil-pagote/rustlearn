# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Status

This is a Rust learning workspace, developed inside a devcontainer rather than
on the host. Broad tutorial/quickstart notes live outside this repo; verified,
repo-specific findings (things confirmed against *this* environment) live
in-repo under `docs/` — start at `docs/README.md`.

## Environment

- The Rust toolchain (rustup, cargo, rustfmt, clippy) is only installed inside
  the devcontainer (`.devcontainer/Containerfile` + `devcontainer.json`) — not
  on the host.
- The container is plain Ubuntu 24.04 + `rustup` (stable toolchain, installed
  via the official rustup.rs script) + `build-essential` (provides the `cc`
  linker Rust needs) + Node.js (so `npx`-launched Claude Code MCP plugins work)
  + the Claude Code CLI itself.

## Commands

Run inside the devcontainer:
- `cargo build` — compile the library.
- `cargo run --example NN_name` — run a numbered exercise under `examples/`.
- `cargo test` — run tests (unit tests in `src/`, integration tests in `tests/`).
- `cargo fmt` — format the whole workspace.
- `cargo clippy --all-targets` — lint.

## Project structure

```
Rust/
├── src/
│   └── lib.rs       # shared library code, once exercises produce reusable functions/structs/traits
├── examples/        # one runnable program per topic, 01_hello.rs .. 21_macros.rs
├── docs/            # verified, repo-specific learning notes (start at README.md)
├── tests/           # integration tests — empty for now
├── CARGO_CHEATSHEET.md
└── Cargo.toml
```

- `examples/NN_name.rs` are standalone programs with `fn main()`, run via
  `cargo run --example NN_name` — this is cargo's built-in convention, no
  `Cargo.toml` changes needed per exercise. Numbered in the order topics are
  covered.
- `src/lib.rs` is empty until there's real shared code to put there — don't
  add modules speculatively. Exercises `use rustlearn::...` once it has content.
- `tests/` — integration tests go here as `tests/*.rs`, each compiled as its
  own crate against `src/lib.rs`. Empty until there's shared code worth
  testing at that level; exercises can also carry inline `#[test]` modules.
