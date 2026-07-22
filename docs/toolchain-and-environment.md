# Toolchain & Environment

## rustup vs. cargo — why two tools?

**rustup** is a toolchain *manager*. It installs/switches Rust versions
(`stable`, `beta`, `nightly`), cross-compilation targets, and components like
`rustfmt`/`clippy`. It knows nothing about any specific project's code — its
job ends at "which compiler and tools are on `PATH`."

**cargo** is the *build tool and package manager*. It compiles code (invoking
`rustc` under the hood), resolves/downloads dependencies from crates.io, runs
tests/examples/benchmarks, and defines the project layout (`Cargo.toml`,
`src/`, `examples/`, `tests/`). It operates *within* whatever toolchain rustup
has currently active.

Split rather than one tool, because:

- Different lifecycles: you install/update a toolchain occasionally; you run
  `cargo build`/`cargo test` constantly. Keeping them separate keeps each
  tool simple — same idea as `nvm`/`rbenv` (version managers) being separate
  from `npm`/`bundler` (package managers) in other ecosystems.
- Multiple toolchains, one cargo interface: rustup can have `stable`,
  `nightly`, and cross targets installed side by side; cargo just uses
  whichever is active (or an explicit `+nightly` override) without needing to
  know how toolchains are stored on disk.
- cargo predates rustup and can technically run without it (OS-packaged
  `rustc`+`cargo`) — but rustup is the recommended path since it makes
  updating, pinning (`rust-toolchain.toml`), and adding components/targets
  trivial.

Concretely in this repo: `rustup component add rustfmt clippy`
(`.devcontainer/Containerfile`) installs the binaries; `cargo fmt` /
`cargo clippy` are cargo subcommands that shell out to those binaries once
rustup has put them on `PATH`. If a component is missing, cargo's error
message is the seam between the two tools made visible — it tells you to go
run the rustup command.

## `cargo` ships bundled with every toolchain — `rustfmt`/`clippy` don't always

Installing or updating a toolchain (`rustup toolchain install stable`,
`rustup update`) always brings `cargo` along with `rustc` — they ship as a
unit. Confirmed in this repo's environment: `rustup component list --installed`
lists `cargo-x86_64-unknown-linux-gnu` right alongside
`rustc-x86_64-unknown-linux-gnu`, and the real binary lives at
`~/.rustup/toolchains/<toolchain>/bin/cargo` (installed at the same time as
`rustc`). The `cargo` found on `PATH` (`~/.cargo/bin/cargo`) is rustup's proxy
shim, which dispatches to that real binary for whichever toolchain is active.

Whether `rustfmt`/`clippy` come along for free depends on the **install
profile** used:

| Profile | Includes |
|---|---|
| `minimal` | `rustc` + `cargo` only |
| `default` | adds `rustfmt`, `clippy`, `rust-docs` |
| `complete` | everything (no longer recommended upstream) |

This repo's `.devcontainer/Containerfile` installs with `--profile default`
*and still* has an explicit `rustup component add rustfmt clippy` line right
after — belt-and-suspenders, since which components a given profile bundles
can vary by rustup version. Without that line, `cargo fmt`/`cargo clippy`
fail with a "component not installed" error telling you to run that exact
command — that error message is the visible seam between rustup (owns the
binary) and cargo (just invokes it).

## Everyday commands

Run inside the devcontainer (rustup/cargo are not on the host):

```bash
rustup show                          # active toolchain + installed components
rustup component add rustfmt clippy  # install formatter/linter if missing
cargo build                          # compile the library
cargo run --example 01_hello         # run an exercise
```

Full reference: `../CARGO_CHEATSHEET.md` (includes a dedicated rustup section).
