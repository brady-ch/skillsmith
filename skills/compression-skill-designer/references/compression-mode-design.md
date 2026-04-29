# Compression Mode Design

Use this guide to design a reusable communication skill that reduces output tokens while preserving technical correctness. Treat JuliusBrussee/caveman as a pattern reference, not text to copy.

## Design Goals

- Preserve exact technical meaning, identifiers, code, commands, file paths, error text, API names, and user-provided quoted text.
- Remove low-value language: filler, repeated acknowledgements, unnecessary hedging, decorative tone, and verbose transitions.
- Make the mode predictable: users should know when it is active, how terse it gets, and how to turn it off.
- Keep the skill portable: rules should work across code review, implementation updates, planning, and troubleshooting.

## Core Skill Structure

Define these sections in the target skill:

- `When To Use`: activation phrases such as "use terse mode", "compress output", "reduce tokens", or project-specific aliases.
- `Persistence`: whether the mode applies for one reply, the current task, or the full session.
- `Mode Levels`: optional light/default/ultra levels with clear differences.
- `Compression Rules`: what to remove and what must remain exact.
- `Safety Fallbacks`: when to temporarily return to normal explicit language.
- `Examples`: before/after samples for explanation, debugging, and command-output responses.

## Compression Rules

Good compression removes ceremony, not meaning.

- Drop pleasantries, filler, redundant setup, and restating the obvious.
- Prefer direct fragments when grammar is not needed for clarity.
- Use short common words when they do not lose precision.
- Keep technical terms exact when precision matters.
- Keep code blocks unchanged.
- Keep command output and quoted errors unchanged.
- Keep warnings explicit enough that risk cannot be missed.

Avoid:

- Removing negation or conditionals.
- Abbreviating unfamiliar domain terms without defining them.
- Compressing multi-step instructions until ordering is ambiguous.
- Turning uncertainty into false confidence.
- Reusing another skill's exact branding, prose, examples, or command names unless that is intentionally licensed and requested.

## Mode Levels

Use levels only if they change behavior enough to justify extra complexity.

| Level | Behavior |
| --- | --- |
| light | Full sentences, no filler, minimal hedging |
| default | Short fragments allowed, articles optional, technical terms exact |
| ultra | Maximum compression for status updates and simple answers; avoid for risky instructions |

Default to `light` when the audience may be non-technical or the content is safety-sensitive. Default to `default` for developer workflows where terse updates are useful.

## Safety Fallbacks

Temporarily use normal explicit language for:

- destructive commands or irreversible operations
- security, privacy, legal, medical, or financial warnings
- ambiguous requests where compressed wording could hide an assumption
- multi-step procedures where fragments could be executed in the wrong order
- user confusion, repeated questions, or direct requests for more clarity

Resume compressed style after the risky or ambiguous section is complete.

## Examples

Explanation:

- Verbose: "The problem is likely caused by the authentication middleware checking token expiry after the request handler runs."
- Compressed: "Bug likely in auth middleware. Expiry check runs after handler. Move check before handler."

Status update:

- Verbose: "I have finished inspecting the parser and found that the index validation already checks for missing reference files."
- Compressed: "Parser checked. Index validation already catches missing reference files."

Safety fallback:

- Compressed style is not enough for: "Delete database? Run command?"
- Use explicit warning: "Warning: this permanently deletes all rows and cannot be undone. Confirm backup exists before running it."

## Output Checklist

When producing a new compression skill, include:

- concise trigger metadata
- explicit off switch
- default level
- exact-preservation rules
- safety fallback rules
- two or more examples
- note that generated code, commit messages, and public documentation should stay normal unless the user explicitly wants compressed style there

