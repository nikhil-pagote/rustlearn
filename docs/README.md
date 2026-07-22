# Rust Learning Notes

In-repo notes capturing things verified **against this actual environment**
(stable Rust via rustup, in the devcontainer) — repo-specific findings that
should travel with the code, as opposed to general Rust docs/books.

## Index

- **[Rust tutorial](rust-tutorial.md) — start here.** A beginner-friendly tour
  of the language, every exercise verified against this exact toolchain.
- [New project checklist](new-project-checklist.md) — generic steps for
  starting *any* Rust project from scratch (not tied to this repo's choices).
- [Toolchain & environment](toolchain-and-environment.md) — rustup vs. cargo,
  why they're separate tools, everyday commands.
- [Repo setup](setup.md) — how this workspace specifically was scaffolded
  (cargo init, devcontainer, Claude Code tooling), step by step.
- [Language notes](language-notes.md) — HashMap iteration order, attributes on
  `let` statements, edition 2024, gotchas that differ from general docs.
- [Troubleshooting](troubleshooting.md) — fixes for errors we've actually hit.

See also `../CARGO_CHEATSHEET.md` for the cargo command reference.

Add a page per topic here as exercises surface things worth recording (gotchas,
environment quirks, decisions). Don't add pages speculatively; The Rust Book
and std docs already cover the language well, so only what's specific to
*this* repo's setup belongs here.
