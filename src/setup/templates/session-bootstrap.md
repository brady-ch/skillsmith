# Skillsmith (session bootstrap)

This file is injected at **Cursor Agent session start** when you install project hooks via `skillsmith setup`.

## Use the catalog

1. Point **`SKILLSMITH_REPO_ROOT`** at a checkout of the skillsmith repository that contains `catalog/catalog.toml`.
2. Use `skillsmith recommend --intent "<your task>" --format json` before picking a skill by hand.
3. Use `skillsmith explain --intent "..." --format json` when you need routing reasons.
4. Use `skillsmith validate` after changing skill layout, catalog metadata, or install behavior.
5. Work from the repository root, inspect current files before editing, and preserve unrelated user changes.

## This project

The reusable default workflow lives in the `using-skillsmith` skill. Keep this file short and project-specific.
