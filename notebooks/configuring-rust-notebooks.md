# Configuring Rust in Jupyter Notebooks

How `notebooks/*.ipynb` get Rust cell execution in this repo, and how to
reproduce/verify the setup. Based on the general approach in
[Interactive Rust with Jupyter Notebooks](https://ratulmaharaj.com/posts/interactive-rust-with-jupyter-notebooks/),
adapted to what's actually installed in this devcontainer.

## The pieces

Rust-in-Jupyter works via **EvCxR** ("An evaluation context for Rust") — a
Rust REPL exposed as a Jupyter kernel. No Python is required to *run* it,
only to register/launch it in the classic Jupyter Notebook UI.

- `evcxr_jupyter` — the kernel binary. Installed via `cargo install`, since
  it's a Rust program.
- A **kernelspec** — a small JSON file telling any Jupyter-protocol frontend
  how to launch the kernel binary.
- A frontend that speaks the Jupyter protocol — either `jupyter notebook`
  (needs Python) or VS Code's Jupyter extension (needs neither Python nor a
  `jupyter` install).

## What's already set up in this repo's devcontainer

`.devcontainer/Containerfile` installs and registers the kernel at image
build time:

```dockerfile
RUN cargo install --locked evcxr_jupyter \
    && evcxr_jupyter --install \
    && rm -rf /root/.cargo/registry
```

`evcxr_jupyter --install` writes the kernelspec to
`/root/.local/share/jupyter/kernels/rust/kernel.json`:

```json
{
  "argv": ["/root/.cargo/bin/evcxr_jupyter", "--control_file", "{connection_file}"],
  "display_name": "Rust",
  "language": "rust",
  "interrupt_mode": "message"
}
```

`.devcontainer/devcontainer.json` adds the `ms-toolsai.jupyter` VS Code
extension. VS Code launches the kernel binary directly over ZeroMQ ("raw
kernel" mode) using that kernelspec — **no `pip install notebook`, no
`jupyter` CLI, no Python at all** are present or needed in this container.
That's a deliberate divergence from the blog post's instructions (which
target the classic `jupyter notebook` web UI and assume a Python
environment); evcxr bundles a pure-Rust ZeroMQ implementation, so there's no
libzmq/cmake dependency either.

Verify the pieces are present:

```bash
which evcxr_jupyter                                   # /root/.cargo/bin/evcxr_jupyter
cat /root/.local/share/jupyter/kernels/rust/kernel.json
```

## Using it

1. Open a `.ipynb` file under `notebooks/` in VS Code (e.g. `01_rust_tour.ipynb`).
2. In the kernel picker (top right of the notebook editor), choose **Rust**.
3. Run cells — each cell is evaluated by the EvCxR REPL, which keeps state
   (variables, `use` imports, function/struct defs) across cells in the same
   run, same as any other Jupyter kernel.

## Reproducing this outside the devcontainer

If you ever need `evcxr_jupyter` on a machine without this repo's container:

```bash
# 1. Rust toolchain (rustc, cargo, rustup)
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# 2. The kernel itself (builds from source, takes a few minutes)
cargo install evcxr_jupyter

# 3. Register the kernelspec with Jupyter
evcxr_jupyter --install
```

At that point, either:
- install VS Code's Jupyter extension and pick the "Rust" kernel, or
- `pip install notebook && jupyter notebook` for the classic browser UI —
  "Rust" will appear in the kernel dropdown when creating a new notebook.

## Gotchas

- `evcxr_jupyter --version` isn't a recognized flag on this build — use
  `which evcxr_jupyter` to confirm it's installed, and check the kernelspec
  JSON above to confirm registration.
- Because this container has no Python/`jupyter` CLI, `jupyter kernelspec
  list` won't work here — VS Code reads
  `~/.local/share/jupyter/kernels/*/kernel.json` directly.
