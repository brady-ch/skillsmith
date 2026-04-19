# Skillsmith (session bootstrap)

This file is injected at **Cursor Agent session start** when you install project hooks via `skillsmith setup`.

## Use the catalog

1. Point **`SKILLSMITH_REPO_ROOT`** at a checkout of the skillsmith repository that contains `catalog/catalog.toml` (the setup wizard prints this after cloning to your data directory).
2. From any directory, run for example:
   - `skillsmith recommend --intent "<your task>" --format json`
   - `skillsmith explain --intent "..."` 
   - `skillsmith install --name <skill>`
3. Open the `SKILL.md` and suggested reference paths from the JSON output.

## This project

Edit **this file** to add project-specific agent instructions. Keep it short; put long conventions in **`AGENTS.md`** if you use that pattern.
