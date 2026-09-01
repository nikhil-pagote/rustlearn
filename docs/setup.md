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

## Container runtime: Podman

Podman is the intended runtime here; Docker is present on the host only for an
unrelated NVIDIA operator dependency. Nothing in the repo is Docker-specific —
`"build": { "dockerfile": ... }` is just the dev container spec's key name, and
the file is already called `Containerfile`, which is Podman's native name — so
switching is purely a client-side setting:

- **Zed** (the editor used here): set `"use_podman": true` at the **top level**
  of Zed's `settings.json`. Zed then offers to reopen the project in the
  container when it sees `.devcontainer/devcontainer.json`, or use
  `projects: initialize dev container` / `Project: Open Remote` from the
  command palette.
- **VS Code Dev Containers extension:** set `"dev.containers.dockerPath":
  "podman"` in user settings. The extension shells out to that binary; no
  socket or service needs to be running.
- **devcontainer CLI:** pass `--docker-path podman`.

```jsonc
// Zed settings.json
{
  "use_podman": true
}
```

```bash
# or, from a host terminal
devcontainer up --docker-path podman --workspace-folder .
```

### Zed dev container caveats

Zed's dev container support is young, and three open upstream issues are worth
recognising by their symptoms rather than rediscovering:

- **Editing `devcontainer.json` does not rebuild anything.** Zed never restarts
  a container on config change, so an edit to the Containerfile or this file
  looks like it did nothing. Stop the container by hand
  (`podman kill <name>`; add `podman rmi` to force an image rebuild) and reopen
  the project.
- **A stopped container is not restarted**, so after a reboot Zed execs into a
  container that isn't running and reports
  `can only create exec sessions on running containers`
  ([zed#48483](https://github.com/zed-industries/zed/issues/48483)). Run
  `podman start <name>` first.
- **`mkdir: cannot create directory '.zed_server': Permission denied`**
  ([zed#54257](https://github.com/zed-industries/zed/issues/54257)) — Zed
  uploads its server binary by exec'ing as a non-root uid that cannot write in
  the container. `"remoteUser": "root"` is set in `devcontainer.json` partly to
  keep Zed exec'ing as root here; it is also simply the truth about this image,
  whose toolchain all lives under `/root` (`/root/.cargo/bin`,
  `/root/.local/bin`, and the evcxr kernelspec in `/root/.local/share`).

`podman-compose` is unsupported by Zed (it has no `--format=json`), which costs
nothing here — this repo builds from a Containerfile, not a compose file.

### What rootless Podman changes

Rootless Podman maps container root to your host uid, so the repo's files show
up **root-owned inside the container** rather than as uid 1000. That silently
removes the `dubious ownership` error, because git is then root looking at
root-owned files. The `safe.directory` line in the Containerfile stays anyway:
it costs nothing and it is what makes the image work for anyone running Docker,
where host uids pass through unchanged.

It does **not** fix the ssh credential problem. Ownership stops being the
complaint, but `ssh` independently rejects any config with group-write bits,
and a stock `0664 ~/.ssh/config` still trips that check no matter who owns it.
Copying the files in and forcing `0600`, as `.devcontainer/local/` does,
remains necessary under Podman.

Mount options are kept to the portable subset for this reason — the personal
config deliberately omits `consistency=cached`, which is a Docker Desktop
(macOS) hint, a no-op on Linux, and not something Podman's `--mount` parser
accepts. On an SELinux host (Fedora, RHEL) bind mounts would additionally need
a `,z` or `,Z` relabel suffix; Pop!_OS uses AppArmor, so that is not needed
here.

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
