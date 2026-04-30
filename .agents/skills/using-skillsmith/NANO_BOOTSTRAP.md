# Skillsmith (nano bootstrap)

省言為本。載入務少。

**Invariant**

- 此為 skillsmith 目錄與 catalogue。
- 設環境變數 `SKILLSMITH_REPO_ROOT` 指向本 repo（內含 `catalog/catalog.toml`）。
- 路由：**MCP** `skillsmith mcp serve` 之工具，或終端：`skillsmith recommend --intent "<task>" --format json --limit 4`。
- 每 skill：讀該 skill 之 `SKILL.md` → `references/reference-router.md` → **僅一所指** `.md`。勿載全 `references/`。
- 行文縮：`compression-skill-designer`；不可逆、安危、不清晰時方可詳。
- consumer 文案、CLI、catalog 摘要、rules 仍以英文為表（見 `best-practices-english.md` / `references/best-practices-wenyan.md`）。

**Full bootstrap**：`SKILLSMITH_HOOK_BOOTSTRAP=full` 載入整份 `skills/using-skillsmith/SKILL.md`。

規約全文：[`docs/token-first-spec.md`](../../docs/token-first-spec.md)。
