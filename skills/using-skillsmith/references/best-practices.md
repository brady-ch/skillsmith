# Skillsmith Best Practices

Use this checklist in skillsmith repo or when installing project defaults elsewhere.

## Core Workflow

- Work from repo root unless task says otherwise.
- Inspect current files before editing. No guessing.
- Use `cargo run -- recommend --intent "<task>" --format json --limit 10` to pick skill. Do not hand-scrape `skills/`.
- Use `cargo run -- explain --intent "<task>" --format json` when need routing reason.
- Use `cargo run -- validate` after changing skill layout, catalog metadata, or install behavior.

## Editing Rules

- Keep changes small and scoped.
- Preserve unrelated user changes.
- Prefer typed Rust data structures over ad hoc maps or unstructured parsing.
- Add or update tests when behavior change.

## Install Rules

- Use `skillsmith install --name <skill>` for manual installs.
- Use `--link` only for local catalog skills.
- Use `--force` only when replace managed install.
- For project setup, install default best-practice skill into project-local runtime dirs setup flow manages.

## Output Rules

- Prefer concrete next steps over vague advice.
- If task touches catalog or skill layout, verify with `validate`.
- If task touches Rust code, verify with tests or targeted checks before finish.
