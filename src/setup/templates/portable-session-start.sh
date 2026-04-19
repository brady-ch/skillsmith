#!/usr/bin/env bash
# Cursor-only SessionStart: injects .skillsmith/session-bootstrap.md from the project git root.
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"
# .skillsmith/hooks -> project root is parent of .skillsmith
PROJECT_ROOT="$(cd "${SCRIPT_DIR}/../.." && pwd)"
ROOT="$(git -C "${PROJECT_ROOT}" rev-parse --show-toplevel 2>/dev/null || echo "${PROJECT_ROOT}")"
BOOTSTRAP="${ROOT}/.skillsmith/session-bootstrap.md"
bootstrap_content="$(cat "${BOOTSTRAP}" 2>&1 || echo "Missing ${BOOTSTRAP}. Run: skillsmith setup (hooks) or create the file.")"

escape_for_json() {
  local s="$1"
  s="${s//\\/\\\\}"
  s="${s//\"/\\\"}"
  s="${s//$'\n'/\\n}"
  s="${s//$'\r'/\\r}"
  s="${s//$'\t'/\\t}"
  printf '%s' "$s"
}

bootstrap_escaped="$(escape_for_json "${bootstrap_content}")"
session_context="<EXTREMELY_IMPORTANT>\nSkillsmith session bootstrap (editable: .skillsmith/session-bootstrap.md):\n\n${bootstrap_escaped}\n</EXTREMELY_IMPORTANT>"

printf '{\n  "additional_context": "%s"\n}\n' "${session_context}"
