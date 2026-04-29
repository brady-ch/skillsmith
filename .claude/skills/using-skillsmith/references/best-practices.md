# Skillsmith Best Practices

Use this checklist when working in the skillsmith repository or when installing the project defaults into another checkout.

## Core Workflow

- Work from the repository root unless a task says otherwise.
- Inspect the current files before editing; do not guess at repo state.
- Use `cargo run -- recommend --intent "<task>" --format json --limit 10` to pick a skill instead of hand-scraping `skills/`.
- Use `cargo run -- explain --intent "<task>" --format json` when you need routing reasons.
- Use `cargo run -- validate` after changing skill layout, catalog metadata, or install behavior.

## Editing Rules

- Keep changes small and scoped to the request.
- Preserve unrelated user changes.
- Prefer typed Rust data structures over ad hoc maps or unstructured parsing.
- Add or update tests when behavior changes.

## Install Rules

- Use `skillsmith install --name <skill>` for manual installs.
- Use `--link` only for local catalog skills.
- Use `--force` only when replacing a managed install is the expected behavior.
- For project setup, install the default best-practice skill into the project-local runtime directories the setup flow manages.

## Output Rules

- Prefer concrete next steps over vague advice.
- If the task touches the catalog or skill layout, verify with `validate`.
- If the task touches Rust code, verify with tests or targeted checks before finishing.

