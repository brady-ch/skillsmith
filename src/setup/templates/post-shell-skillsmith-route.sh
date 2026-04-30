#!/usr/bin/env bash
# Cursor postToolUse: Shell follow-up — inject Skillsmith routing after `skillsmith recommend`.
set -euo pipefail
if ! command -v skillsmith >/dev/null 2>&1; then
  printf '%s\n' '{}'
  exit 0
fi
exec skillsmith hook post-shell-recommend-followup --max-chars "${SKILLSMITH_HOOK_ROUTE_MAX_CHARS:-400}"
