---
name: commit-push
description: Commit the working tree and push to origin/master, following this repo's commit-message style and its devcontainer push quirks. Use when the user asks to commit, commit and push, or "save this to git".
---

# commit-push

Stage, commit and push the current work. This repo is a solo learning
workspace: **commit directly to `master`** and push there — every commit in
its history is a direct push, and there are no other branches. Don't create a
feature branch unless the user asks.

## Steps

1. **Look before staging.** Run `git status --short` and `git diff` (plus
   `git diff --cached` if anything is already staged). Never `git add -A`
   blind — read what changed first, and check for files that shouldn't be
   committed (secrets, `target/`, stray scratch files).

2. **Strip notebook outputs** if any `notebooks/*.ipynb` changed. Committed
   outputs churn the diff on every re-run, and a partially-run notebook stores
   a confusing mix. `.ipynb_checkpoints/` is already gitignored; the saved
   outputs inside the file are not.
   ```bash
   python3 - <<'EOF'
   import json, sys
   p = 'notebooks/01_rust_tour.ipynb'
   nb = json.load(open(p))
   for c in nb['cells']:
       if c.get('cell_type') == 'code':
           c['outputs'] = []
           c['execution_count'] = None
   json.dump(nb, open(p, 'w'), indent=1, ensure_ascii=False)
   open(p, 'a').write('\n')
   EOF
   ```
   Ask first if the user might want outputs committed for GitHub rendering.

3. **Group into coherent commits.** One commit per idea, not one per session.
   Unrelated changes that happen to be in the tree together (a devcontainer fix
   and an exercise edit) belong in separate commits — stage by path, not
   wholesale.

4. **Write the message in this repo's style** — check `git log` to confirm:
   - Imperative subject under ~72 chars, no trailing period, no `type:` prefix
     (`Add a Rust notebook running on the evcxr Jupyter kernel`).
   - A body explaining **why** and what was non-obvious — the constraint that
     forced the approach, the gotcha a future reader would trip on. Wrap at 72.
   - Skip the body only for genuinely trivial changes.
   - End with the `Co-Authored-By:` trailer.

   Pass the message via a heredoc (`git commit -F -`) so backticks and blank
   lines survive.

5. **Verify before pushing** if code changed — `cargo build`, or
   `cargo run --example NN_name` for an exercise. Notebook and docs edits don't
   need it. Don't push something you haven't seen work.

6. **Push:** `git push origin master`, then confirm with `git status -sb`
   (should read `## master...origin/master` with no ahead/behind).

7. **Report** the pushed range (`60bb0fe..4415acb`), one line per commit, and
   anything you deliberately left uncommitted.

## Push troubleshooting

`error: cannot run ssh: No such file or directory` — git has no built-in SSH,
it shells out to the `ssh` binary. The Containerfile installs `openssh-client`,
so this only appears in a container built before that was added. Unblock the
current session with:

```bash
apt-get update -qq && apt-get install -y -qq --no-install-recommends openssh-client
```

`--no-install-recommends` matters: the recommends pull in X11 libs that have
failed to fetch from the Ubuntu mirror here. Rebuild the container for the
permanent fix.

`Bad owner or permissions on /root/.ssh/config` / `detected dubious ownership`
— both come from the same mismatch: the container runs as root while
host-mounted files keep the host user's uid. Ownership is handled in the image
now (`git config --system --add safe.directory` in the Containerfile), so
`dubious ownership` should not reappear; if it does, the container predates
that line and needs a rebuild.

The SSH one means the container has no usable credentials, because the shared
`devcontainer.json` intentionally mounts none — a clone must work for anyone
with no per-machine setup. Pushing requires the gitignored personal config:

```bash
devcontainer up --config .devcontainer/local/devcontainer.json --workspace-folder .
```

It mounts the host keys at `/tmp/host-ssh` and `stage-credentials.sh` copies
them to `/root/.ssh` as root-owned `0600` files — copying, not mounting, is
what satisfies ssh's ownership and group-writable checks. To unblock a push in
a container started from the plain config, do the same by hand:

```bash
install -d -m 700 /root/.ssh-local
install -m 600 /tmp/host-ssh/id_ed25519_github /root/.ssh-local/
GIT_SSH_COMMAND='ssh -o IdentitiesOnly=yes -i /root/.ssh-local/id_ed25519_github' git push origin master
```

`gh` isn't installed and the old `~/.config/gh` mount carried no token, so
**SSH is the only working auth path**. Don't try to route a push through `gh`.
