# Tracking System

Use this reference when the repo needs a durable execution tracker.

## Default Repo Structure

Create this structure at the repo root if it is missing:

```text
.product/
  tracking.md
  routing.md
  items/
    _template/
      summary.md
      questions.md
      acceptance.md
      delivery-plan.md
      blockers.md
```

Then create one folder per tracked item under `.product/items/<item-id>/`.

## Tracking Rules

- `tracking.md` is the source of truth for status.
- Every item gets a stable `item-id` in kebab-case.
- Use Markdown checkboxes so progress is visible in diffs.
- Mark a checkbox complete only when the completion criteria in that item's `acceptance.md` are satisfied.
- If work is blocked, leave the main item unchecked and record the blocker in the item's `blockers.md`.

## Tracking Row Format

Use one row per item with:
- checkbox
- item id
- short title
- status
- owner
- link to the item folder

## Lifecycle

- New work starts as unchecked in `tracking.md`.
- When an item is accepted for execution, create its folder and minimum detail files.
- Update `questions.md`, `delivery-plan.md`, and `blockers.md` as reality changes.
- Check off the item in `tracking.md` only after acceptance is met.

