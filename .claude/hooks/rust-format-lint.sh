#!/usr/bin/env bash
# PostToolUse hook: after a .rs file is edited, auto-FORMAT then LINT it.
#
#   format : `cargo fmt` on the whole workspace (rustfmt has no reliable
#            single-file mode that respects rustfmt.toml)
#   lint   : `cargo clippy` on the whole workspace
#
# Errors and warnings are fed back to Claude via exit code 2. jq isn't installed
# in this env, so file_path is parsed from the hook JSON with grep/sed.
set -u

input=$(cat)
file=$(printf '%s' "$input" \
  | grep -oE '"file_path"[[:space:]]*:[[:space:]]*"[^"]*"' \
  | head -1 | sed -E 's/.*:[[:space:]]*"([^"]*)"/\1/')

case "$file" in
  *.rs) ;;
  *) exit 0 ;;                       # not a Rust file — nothing to do
esac

cd "${CLAUDE_PROJECT_DIR:-.}" || exit 0
[ -f "$file" ] || exit 0

# 1) Format in place. A failure here is a real parse/syntax error — surface it.
fmt=$(cargo fmt --all 2>&1)
if [ $? -ne 0 ]; then
  { echo "⚠️  cargo fmt failed (syntax error?):"
    printf '%s\n' "$fmt" | grep -m5 -iE 'error' | sed 's/^/    /'; } >&2
  exit 2
fi

# 2) Lint / type-check.
out=$(cargo clippy --all-targets --quiet 2>&1); rc=$?

if [ "$rc" -ne 0 ]; then
  { echo "⚠️  cargo clippy failed:"
    printf '%s\n' "$out" | grep -m6 -iE 'error(\[|:)' | sed 's/^/    /'; } >&2
  exit 2
fi

warns=$(printf '%s\n' "$out" | grep -iE 'warning:')
if [ -n "$warns" ]; then
  { echo "ℹ️  cargo clippy warnings:"
    printf '%s\n' "$warns" | head -5 | sed 's/^/    /'; } >&2
  exit 2
fi
exit 0
