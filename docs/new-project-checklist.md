# Starting a New Rust Project From Scratch

A generic checklist, independent of any one project's specific choices. See
[setup.md](setup.md) for how these choices actually played out in *this* repo.

## 1. Toolchain

- Install rustup if not already present:
  `curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh`
- Add the components you'll actually use: `rustup component add rustfmt clippy`
- Optional — pin a toolchain per-project so it's reproducible across machines,
  via `rust-toolchain.toml` at the root:
  ```toml
  [toolchain]
  channel = "stable"
  ```

## 2. Scaffold the package

- `cargo new <name>` — binary package (`src/main.rs`); also creates a git repo
  and `.gitignore` if none exist.
- `cargo new --lib <name>` — library package (`src/lib.rs`) instead.
- `cargo init [--lib]` — same as above, but inside a directory that already
  exists (doesn't create the directory itself).
- `src/main.rs` and `src/lib.rs` can coexist in one package (a binary that
  depends on its own library) — pick lib-only, bin-only, or both based on what
  you're actually building, not speculatively.

## 3. `Cargo.toml` basics

- `name`, `version` (start at `0.1.0`), `edition` — use the latest stable
  edition (`2024`, stabilized in rustc 1.85+) unless you have a reason not to.
- Add `description`, `license`, `repository` only if you intend to publish to
  crates.io — skip them for a private or learning project.

## 4. Decide the directory layout up front — only what you need now

- `examples/` — runnable demo programs, `cargo run --example name`.
- `tests/` — integration tests, `cargo test`.
- `benches/` — performance benchmarks (needs a harness, e.g. the `criterion`
  crate — stable Rust has no built-in bench harness).
- `src/bin/` — extra named binaries sharing this package.

Don't create any of these until there's real content for them.

## 5. Format/lint config — optional, only if the defaults don't fit

- `rustfmt.toml` at root customizes `cargo fmt` (e.g. `max_width`).
- `clippy.toml` at root customizes clippy's thresholds.
- Both are genuinely optional — `cargo fmt`/`cargo clippy` work with zero
  config out of the box.

## 6. First commit

- `.gitignore` from `cargo new`/`cargo init` already excludes `/target`.
- Commit `Cargo.lock` too, even for a library — it makes builds of the
  examples/tests in this package reproducible. (Only pure libraries meant to
  be *depended on* by other people's projects conventionally omit the lock,
  since the consuming project's lock takes over.)

## 7. Common next steps — add only when actually needed

- CI (e.g. GitHub Actions running `cargo build`, `cargo test`,
  `cargo fmt -- --check`, `cargo clippy --all-targets`).
- A root `README.md` describing the project, once there's something to describe.
