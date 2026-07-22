# Repo Setup

How this workspace was scaffolded, in order. Useful if you need to recreate it,
extend it to a second crate, or understand why a given file exists.

## 1. Initialize the cargo project

```bash
cargo init --name rustlearn
```

This created `Cargo.toml` (package `rustlearn`, edition 2024), `src/main.rs`
(default `fn main()`), `.gitignore` (`/target`), and a git repo (none existed
in the directory before this).

## 2. Switch from a binary to a library + examples layout

Deleted the generated `src/main.rs` and added `src/lib.rs` instead (empty stub
for shared code later). Exercises live as standalone programs under
`examples/`, run via cargo's built-in `cargo run --example NN_name` —
no `Cargo.toml` edits needed per exercise, cargo auto-discovers files there.

First exercise added: `examples/01_hello.rs`, verified with:

```bash
cargo run --example 01_hello
```

## 3. Add the devcontainer

Modeled on the sibling `mojo` learning repo (`../mojo/.devcontainer/`): the
Rust toolchain lives inside a container, not on the host.

- `.devcontainer/Containerfile` — Ubuntu 24.04 + `build-essential` (linker) +
  rustup (stable profile) + `rustfmt`/`clippy` components + Node.js (so
  `npx`-launched Claude Code MCP plugins work) + the Claude Code CLI itself.
- `.devcontainer/devcontainer.json` — `workspaceFolder: /workspaces/Rust`
  (must match the Containerfile's `WORKDIR`), `postCreateCommand: cargo build`.

## 4. Add Claude Code project tooling

- `.claude/settings.json` — allowlists `cargo`/`git` read-ish commands, and
  wires a `PostToolUse` hook on file edits.
- `.claude/hooks/rust-format-lint.sh` — runs `cargo fmt` then
  `cargo clippy --all-targets` after any `.rs` edit; feeds errors/warnings
  back to Claude via exit code 2.
- `.claude/skills/new-exercise/SKILL.md` — scaffolds the next numbered file
  under `examples/` and verifies it runs before finishing.

## 5. Docs and CLAUDE.md

`docs/README.md` as the notes index, this file, and the top-level `CLAUDE.md`
summarizing environment/commands/structure for future Claude Code sessions.

## Verified once, on the host

Before committing to the devcontainer-only approach, `rustc`/`cargo` were
found to already be present on the host (`~/.cargo/bin`, version 1.97.1) and
`cargo run --example 01_hello` was tested there. The devcontainer is still the
supported way to work in this repo going forward — the host check was just to
confirm the example compiled before the container existed.
