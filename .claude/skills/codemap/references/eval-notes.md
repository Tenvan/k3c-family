# Eval Notes

Use these checks when reviewing `codemap`.

## Acceptance criteria

- The skill runs only on explicit request, never as a side effect of another task.
- Each agent/subagent resolves logical `SKILL_FOLDER` from the runner location or the five workspace-root candidates, validates `SKILL.md` and `scripts/codemap.mjs`, aborts on zero/multiple matches, and substitutes the quoted absolute path without a shell binding.
- Step 1 runs `changes` for real and derives `mode` from the actual output (`init`, `update`, `noop`).
- Include/exclude globs cover core code and config only; tests, docs, translations, build output, and dependencies are excluded.
- An ambiguous stack triggers an `interviewe` block before scanning; a clear stack does not.
- Only affected folders get a new or updated `codemap.md`; unchanged folders keep their file byte for byte.
- Filled codemaps are updated section by section after reading them, not regenerated blindly.
- Every codemap has the four sections Responsibility, Design, Flow, Integration with technical terminology and real paths.
- Paths returned by subagents are verified against the file system before they land in a codemap.
- `update` runs after the codemaps are written so the state matches the new hashes.
- The root `codemap.md` lists every folder codemap with a Responsibility summary and a relative link.
- The `## Repository Map` section is appended to existing `CLAUDE.md` / `AGENTS.md` files (or a new `CLAUDE.md`) exactly once.
- The workflow keeps a State Table and `⛔ DO NOT continue` gates for dependent steps.
- The final report is German and names mode, folders, state file, registration target, and skipped work.

## Non-goals

- Replacing Grep or the `explorer` agent for "where is X" questions.
- Measuring complexity or quality; that is the `complexity` skill.
- Writing TODO lists, review findings, or wishful architecture into codemaps.
