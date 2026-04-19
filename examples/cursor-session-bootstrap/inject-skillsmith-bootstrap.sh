#!/usr/bin/env bash
# Cursor project hook: delegates to repo-root hooks/session-start with Cursor JSON output.
# Kept under .cursor/hooks/ per Cursor hook conventions; logic lives in hooks/session-start.

set -euo pipefail

ROOT="$(cd "$(dirname "$0")/../.." && pwd)"
exec env SKILLSMITH_HOOK_PLATFORM=cursor bash "${ROOT}/hooks/session-start"
