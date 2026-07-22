# Language Notes

Things confirmed against **this repo's toolchain** (rustc/cargo 1.97.1, edition
2024) — not copied from general docs, which usually apply regardless of
version but occasionally don't.

## Attributes on `let` statements work on stable

`#[allow(clippy::...)]` directly above a `let` binding (not just above a whole
`fn`) compiles fine on stable 1.97.1 — no `#![feature(stmt_expr_attributes)]`
needed. Used in `examples/11_closures_iterators.rs` to scope a clippy allow to
one line instead of the whole function. This is narrower than attribute-on-item
placement (whole `fn`/`struct`/module), which is the more commonly documented
form.

## `HashMap` iteration order is not deterministic

`examples/06_collections.rs` iterates a `HashMap<&str, i32>` with a `for`
loop; the print order of entries varies between runs (confirmed: re-running
the same binary changes which of "Alice"/"Bob" prints first). This is
intentional — `HashMap` uses per-process randomized hashing (SipHash with a
random seed) to prevent hash-flooding DoS attacks. Use `BTreeMap` instead if
a stable/sorted iteration order matters.

## Devcontainer mount point must match the Containerfile's WORKDIR

Same guardrail as the sibling `mojo` repo: `.devcontainer/devcontainer.json`'s
`workspaceFolder` and `.devcontainer/Containerfile`'s `WORKDIR` are both
`/workspaces/Rust`. Unlike Mojo (which bakes absolute paths into a config
file), Rust/cargo don't have an equivalent stale-path failure mode — but
keeping them in sync is still correct so absolute paths in build output
(`cargo build --message-format=json`, debug info) stay consistent.

## Edition 2024

`Cargo.toml` sets `edition = "2024"` (stabilized in rustc 1.85, well below
this repo's 1.97.1). No edition-specific gotchas hit yet — nothing here beyond
noting it, since this section is for things actually verified against this
setup, not a copy of the edition guide.
