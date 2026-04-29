# Commit Cadence

- Treat a change as commit-worthy only after it is complete and tested.
- Run the relevant targeted tests before staging.
- Check `git status` and confirm the diff matches the completed slice.
- Commit with a short imperative message that names the tested slice.
- If tests fail, fix them before committing; do not checkpoint broken work.
- If the slice is partial or exploratory, keep it uncommitted and continue until it is complete.
- If multiple independent slices are finished, commit each slice separately rather than batching unrelated work.
