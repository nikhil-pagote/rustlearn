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

## Host credentials: kept out of the shared config

The committed `.devcontainer/devcontainer.json` deliberately mounts **nothing**
from the host beyond the repo itself, so a fresh clone builds and runs with no
per-machine setup. That is a change from the original setup, which bind-mounted
`~/.ssh`, `~/.gitconfig` and `~/.config/gh` — all three turned out to be
actively broken rather than merely personal:

- The container runs as **root**, but bind-mounted host files keep the host
  user's uid (typically 1000). `ssh` aborts with `Bad owner or permissions` on
  any config it does not own, and separately on any config that is
  group-writable — a routine `0664 ~/.ssh/config` trips it. The mount is
  read-only, so `chmod` cannot repair it in place.
- `~/.gitconfig` mounted as a *single file* breaks writes: git rewrites config
  via temp-file + rename, which a file bind mount rejects with
  `Device or resource busy`. So `git config --global` fails inside the
  container.
- If the host lacks one of those paths, Docker helpfully creates it — as a
  **directory** — so a student with no `~/.gitconfig` gets a directory where
  git expects a file.
- `~/.config/gh` was mounted for a `gh` CLI the image never installs.

The `dubious ownership` error from the same uid mismatch is fixed for everyone
in the image instead, via `git config --system` in the Containerfile — system
config is the only level that works here, since `~/.gitconfig` may itself be an
unwritable mount.

### Getting your own credentials in (maintainer only)

`.devcontainer/local/` holds a gitignored personal variant that **copies**
credentials in rather than mounting them onto their final path: host `~/.ssh`
and `~/.gitconfig` are mounted read-only at `/tmp/host-*`, and
`stage-credentials.sh` copies them into `/root` as root-owned `0600` files.
Copying is what fixes both the ownership and the permissions complaint. Select
it with the Dev Containers config picker, or:

```bash
devcontainer up --config .devcontainer/local/devcontainer.json --workspace-folder .
```

Tradeoff worth knowing: this still gives the container read access to the
host's real SSH private key — anything running inside can act as you on
GitHub. Fine for a personal learning container you control; wouldn't do this
for a container running untrusted code, and it is precisely why the shared
config no longer does it by default.

## Verified once, on the host

Before committing to the devcontainer-only approach, `rustc`/`cargo` were
found to already be present on the host (`~/.cargo/bin`, version 1.97.1) and
`cargo run --example 01_hello` was tested there. The devcontainer is still the
supported way to work in this repo going forward — the host check was just to
confirm the example compiled before the container existed.
