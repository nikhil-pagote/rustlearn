# Cargo Cheatsheet

Quick reference for this Rust workspace. Tested with **cargo 1.97.1 / rustc 1.97.1**.
Run everything **inside the devcontainer** — the Rust toolchain is not on the host.

> This project is a single crate (`rustlearn`, edition 2024) with a library
> (`src/lib.rs`), numbered exercises as examples (`examples/*.rs`), and
> integration tests (`tests/*.rs`). No workspace `[workspace]` split — just one
> `Cargo.toml`.

---

## rustup (toolchain management)

`rustup` manages Rust toolchains/versions themselves — `cargo`/`rustc` are
installed *by* rustup, not the other way around. This is what
`.devcontainer/Containerfile` uses to set up the toolchain in this repo.

| Command | What it does |
|---|---|
| `curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs \| sh` | Install rustup (first-time setup; this repo's Containerfile does this with `-y --profile default`). |
| `rustup component add rustfmt clippy` | Install the formatter/linter for the active toolchain (not bundled by default on every profile). |
| `rustup show` | Active toolchain, installed targets, installed components. |
| `rustup update` | Update all installed toolchains to the latest release. |
| `rustup toolchain list` | List installed toolchains (e.g. `stable`, `nightly`). |
| `rustup toolchain install nightly` | Install an additional toolchain alongside `stable`. |
| `rustup default <toolchain>` | Change the global default toolchain. |
| `rustup override set <toolchain>` | Pin a toolchain for just the current directory (writes to rustup's own state, not the repo). |

A `rust-toolchain.toml` at the repo root pins the toolchain *in the repo*
(portable across machines/CI, unlike `rustup override`) — not currently used
here since the devcontainer already fixes the toolchain; add one if this ever
runs outside a container.

---

## Creating a new project

| Command | What it does |
|---|---|
| `cargo new <name>` | Scaffold a new **binary** package (`src/main.rs`) in a new `<name>/` directory; also runs `git init` + writes `.gitignore` if not already in a repo. |
| `cargo new --lib <name>` | Same, but a **library** package (`src/lib.rs`) instead. |
| `cargo init [--lib]` | Same as above, but inside a directory that already exists (this is how this repo itself was scaffolded — see `docs/setup.md`). |

For the full generic walkthrough of starting a Rust project from scratch
(toolchain, `Cargo.toml` basics, directory layout, first commit), see
`docs/new-project-checklist.md`.

---

## Building & running

| Command | What it does |
|---|---|
| `cargo build` | Compile the library (debug profile). |
| `cargo build --release` | Compile with optimizations. |
| `cargo run --example NN_name` | Run a numbered exercise under `examples/`. |
| `cargo check` | Type-check without producing a binary (fastest feedback loop). |

```bash
cargo run --example 01_hello     # build + run examples/01_hello.rs
cargo build --examples           # compile all examples without running any
```

---

## Formatting & linting

| Command | What it does |
|---|---|
| `cargo fmt` | Format the whole workspace in place (rustfmt). |
| `cargo fmt -- --check` | Check formatting without modifying files (CI-style). |
| `cargo clippy --all-targets` | Lint, including examples and tests. |
| `cargo clippy --fix --all-targets` | Auto-apply clippy's suggested fixes. |

---

## Testing

| Command | What it does |
|---|---|
| `cargo test` | Run unit tests (in `src/`) and integration tests (in `tests/`). |
| `cargo test <name>` | Run only tests whose name matches `<name>`. |
| `cargo test -- --nocapture` | Show `println!` output even for passing tests. |
| `cargo test --doc` | Run doc-comment examples (`/// ```...```` blocks). |

---

## Managing dependencies

| Command | What it does |
|---|---|
| `cargo add <crate>` | Add a dependency and update `Cargo.toml` + `Cargo.lock`. |
| `cargo add <crate> --dev` | Add a dev-only dependency (tests/examples/benches). |
| `cargo remove <crate>` | Remove a dependency. |
| `cargo update` | Update `Cargo.lock` to the newest allowed versions. |
| `cargo update -p <crate>` | Update just one package in the lock. |
| `cargo tree` | Show the dependency tree. |

`Cargo.lock` is the source of truth for reproducibility — **commit it**
alongside `Cargo.toml` (true for binaries and, here, for the examples/tests too).
Don't hand-edit the lock; let `cargo add`/`cargo update` regenerate it.

---

## Inspecting the setup

| Command | What it does |
|---|---|
| `cargo --version` / `rustc --version` | Toolchain versions (this project: 1.97.1 / 1.97.1). |
| `cargo metadata` | Full project/dependency graph as JSON. |
| `cargo doc --open` | Build and open local docs for this crate + dependencies. |

---

## Global tools (outside this project)

`cargo install` installs binaries available everywhere, independent of any project:

```bash
cargo install <tool>
cargo install --list
cargo uninstall <tool>
```

---

## Troubleshooting

**Nothing hit yet** — this section fills in as real errors come up (mirroring
`docs/troubleshooting.md`). Two guardrails already baked into the setup:

- `.devcontainer/Containerfile`'s `WORKDIR` and `devcontainer.json`'s
  `workspaceFolder` must both be `/workspaces/Rust` — if they diverge, a stale
  path can end up baked into build artifacts after a container rebuild.
- If `cargo`/`rustfmt`/`clippy` seem to disappear after a toolchain change,
  check `rustup show` — a new default toolchain may not have every component
  installed (`rustup component add rustfmt clippy`).

**Env seems corrupt / weird resolution** — rebuild it clean:

```bash
rm -rf target && cargo build
```
