#!/usr/bin/env bash
# One-line installer: install the skillsmith binary from git, then run the setup wizard.
#
# Usage:
#   curl -fsSL https://raw.githubusercontent.com/<org>/<repo>/<ref>/scripts/install.sh | bash
#
# Environment:
#   SKILLSMITH_GIT_URL   Git URL for skillsmith (required if not using default below)
#   SKILLSMITH_GIT_REF   Branch or tag (default: main)
#   SKILLSMITH_SKIP_SETUP  If set to 1, skip running `skillsmith setup` after install
#   SKILLSMITH_ALLOW_ROOT  If set, allow running as root (discouraged)

set -euo pipefail

# Must match [package].repository in Cargo.toml (HTTPS GitHub URL).
DEFAULT_GIT_URL="${SKILLSMITH_GIT_URL:-https://github.com/brady-ch/skillsmith}"
SKILLSMITH_GIT_URL="${SKILLSMITH_GIT_URL:-$DEFAULT_GIT_URL}"
SKILLSMITH_GIT_REF="${SKILLSMITH_GIT_REF:-main}"

if [ "$(id -u)" = 0 ] && [ -z "${SKILLSMITH_ALLOW_ROOT:-}" ]; then
  echo "skillsmith install: refusing to run as root (set SKILLSMITH_ALLOW_ROOT=1 to override)" >&2
  exit 1
fi

echo "skillsmith install: repository ${SKILLSMITH_GIT_URL} (ref ${SKILLSMITH_GIT_REF})"
echo "skillsmith install: ensure you trust this URL before continuing."

if ! command -v cargo >/dev/null 2>&1; then
  echo "skillsmith install: cargo not found. Install Rust from https://rustup.rs then retry." >&2
  exit 1
fi

if ! command -v git >/dev/null 2>&1; then
  echo "skillsmith install: git not found. Install git then retry." >&2
  exit 1
fi

echo "skillsmith install: cargo install skillsmith --locked --git ... (this may take a few minutes)"
cargo install skillsmith --locked --git "${SKILLSMITH_GIT_URL}" --branch "${SKILLSMITH_GIT_REF}"

if [ "${SKILLSMITH_SKIP_SETUP:-0}" = "1" ]; then
  echo "skillsmith install: skipped setup (SKILLSMITH_SKIP_SETUP=1). Run: skillsmith setup"
  exit 0
fi

echo "skillsmith install: starting interactive setup"
exec skillsmith setup
