#!/usr/bin/env bash
# Cursor project hook: runs portable session-start from .skillsmith/hooks/
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/../.." && pwd)"
exec bash "${ROOT}/.skillsmith/hooks/portable-session-start.sh"
