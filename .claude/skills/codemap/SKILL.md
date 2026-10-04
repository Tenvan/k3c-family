---
name: codemap
version: 1.0.0
description: 'Generate and maintain hierarchical per-folder `codemap.md` documentation for a repository, with hash-based change detection so only folders with changed files are re-mapped. Use when the user asks to map, document, or get oriented in an unfamiliar codebase, wants a repository atlas or a per-directory architecture overview, or wants existing codemaps refreshed after code changes. Expensive operation: run only on explicit request, never as a side effect of another task. Not for complexity metrics (use `complexity`).'
---

# Codemap Skill

Map a repository into hierarchical `codemap.md` files: one per relevant folder plus a root atlas. A bundled Node.js script selects the files that matter, records their hashes in `.codemap/state.json`, and on later runs reports which folders changed so only those codemaps are rewritten.

## When to Use

- User asks to map, document, or understand a repository or an unfamiliar module tree.
- User wants a repository atlas (root `codemap.md`) that agents read before working.
- Codemaps already exist and the user wants them refreshed after changes.

**When NOT to use:**

- The question is "where is X" or "who calls Y": Grep or the `explorer` agent is cheaper.
- The user wants complexity metrics: use `complexity`.
- Nobody asked for it. Mapping a repository costs many agent turns; never start it as a side effect.

## ⚠️ Execution Contract

1. NEVER infer or hallucinate command output.
2. NEVER skip a step marked `⚡ EXECUTE`.
3. NEVER proceed to the next step until the current step result is documented.
4. NEVER treat bundled skill files as if they were workspace source files.
5. NEVER overwrite a filled `codemap.md` blindly: read it, then update the affected sections.

## Skill Output (Entry / Exit)

Output these banners as fenced code blocks.

At the start of every execution output:

```text
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
🚀 SKILL START │ codemap
  Repository hierarchisch in codemap.md-Dateien kartieren
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
```

At the end of every execution output:

```text
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
✅ SKILL DONE  │ codemap
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
```

On abort / unresolvable error:

```text
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
❌ SKILL ABORTED │ codemap – <reason>
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
```

## Language Rule

**User-facing status messages and banners are German.** `codemap.md` content follows the language of the repository's existing documentation (default: English). Internal step labels (`✅ Step X`, `✅ Pre:`) stay English.

## Reference Files

- Read `references/examples.md` for a filled folder codemap, a root atlas, and include/exclude patterns per stack.
- Read `references/eval-notes.md` when checking whether a mapping run is complete.
- Paths under `references/` and `scripts/` are bundled skill files relative to this skill, not workspace files.

## Preconditions

- `node` (Node.js 18 or newer) on `PATH`; the script has no dependencies.
- Writable repository root. `.codemap/state.json` and all `codemap.md` files are meant to be committed.
- Logical `SKILL_FOLDER` must resolve (Step 0) before any script call.
- Subagent support (Agent tool) is optional; without it, codemaps are written sequentially by the main agent.

## Workflow (STRICT ORDER)

### Step 0 - Resolve `SKILL_FOLDER`

Each agent or subagent resolves the logical `SKILL_FOLDER` independently. Use the runner-provided directory of the current loaded `SKILL.md` (or the parent of its loaded path) first. If absent, check exactly these workspace-root candidates: `<workspace-root>/.claude/skills/codemap`, `<workspace-root>/.agents/skills/codemap`, `<workspace-root>/.codex/skills/codemap`, `<workspace-root>/.github/skills/codemap`, and `<workspace-root>/plugins/orga-code-quality/skills/codemap`. Accept exactly one candidate with a readable `SKILL.md` whose frontmatter name is `codemap` and readable `scripts/codemap.mjs`, `references/examples.md`, and `references/eval-notes.md`; zero or multiple matches emit SKILL ABORTED. Do not use shell environment variables, recursion, HOME, or CWD. Substitute the quoted absolute path directly as `"<absolute-SKILL_FOLDER>/..."`.

Document: `✅ Pre: SKILL_FOLDER: <path>`

⛔ DO NOT continue to step 1 until exactly one valid logical `SKILL_FOLDER` is resolved.

### Step 1 - Detect state and changes `⚡ EXECUTE`

`<repo-root>` is the current working directory unless the user names another folder.

> ⚠️ Real terminal execution required. DO NOT simulate or guess output.

```bash
node "<absolute-SKILL_FOLDER>/scripts/codemap.mjs" changes --root "<repo-root>"
```

Interpret the result:

| Output | mode |
| --- | --- |
| `No codemap state found. Run 'init' first.` (exit code 1) | `init` → continue with Step 2 |
| `No changes detected.` | `noop` → skip to Step 6 |
| lists of added / removed / modified files plus `folders affected` | `update` → skip to Step 3 |

Document: `✅ Step 1: mode=<init|update|noop>`

Initialize and keep this State Table updated:

| State key | Value |
|-----------|-------|
| SKILL_FOLDER | [set in step 0] |
| repo_root | [set] |
| mode | [set] |
| patterns | [unset] |
| affected_folders | [unset] |
| agent_file | [unset] |

⛔ DO NOT continue until `mode` is set.

### Step 2 - Initialize (mode `init` only) `⚡ EXECUTE`

1. Look at the top-level layout (package manifests, `src/`, `apps/`, `libs/`, language markers) and derive include/exclude globs for **core code and config only**:
   - Include: source trees and manifests, e.g. `src/**/*.ts`, `package.json`, `**/*.py`, `pyproject.toml`.
   - Exclude (mandatory): tests (`**/*.test.*`, `**/*.spec.*`, `tests/**`, `__tests__/**`), docs (`docs/**`, `**/*.md`, `LICENSE`), build output and dependencies (`node_modules/**`, `dist/**`, `build/**`, `**/__pycache__/**`, `*.min.js`), generated code, translations.
   - `.gitignore` entries are applied automatically by the script.
   - `references/examples.md` lists pattern sets per stack.
2. If the stack is ambiguous (monorepo with several languages, unclear which apps matter), ask before scanning. Wenn ein strukturiertes Fragetool verfügbar ist, nutze es für diese Fragen. Andernfalls gib den `interviewe`-Block sichtbar aus und warte auf die Antwort.

   **interviewe** *(pausiert – erst nach Antwort fortfahren)*

   - **Umfang:** Welche Verzeichnisse oder Apps sollen kartiert werden? → Pfade oder „alle"
   - **Ausschlüsse:** Gibt es generierten Code oder Legacy-Bereiche, die draußen bleiben sollen? → Pfade oder „keine"

3. Run init with the derived patterns (one `--include` / `--exclude` per glob):

> ⚠️ Real terminal execution required. DO NOT simulate or guess output.

```bash
node "<absolute-SKILL_FOLDER>/scripts/codemap.mjs" init --root "<repo-root>" \
  --include "src/**/*.ts" --include "package.json" \
  --exclude "**/*.test.ts" --exclude "dist/**" --exclude "node_modules/**"
```

The script writes `.codemap/state.json`, prints the list of folders, and drops an empty `codemap.md` template into every listed folder (plus the root). Existing `codemap.md` files are left untouched.

4. `affected_folders` = the printed folder list.

Document: `✅ Step 2: <n> files selected, <m> folders to map`

Update State Table (`patterns`, `affected_folders` set).

⛔ DO NOT continue to step 4 until step 2 is marked ✅ and `affected_folders` is set.

### Step 3 - Review changes (mode `update` only)

Take `affected_folders` from the `folders affected` section of the Step 1 output. Removed files still count: their folder's codemap must drop the stale references. If a folder has no selected files left, delete its `codemap.md` and remove its row from the root atlas.

Document: `✅ Step 3: <n> folders affected`

Update State Table (`affected_folders` set).

⛔ DO NOT continue to step 4 until `affected_folders` is set.

### Step 4 - Write or refresh the folder codemaps

One `codemap.md` per affected folder, deepest folders first so parents can summarize their children.

Delegation:

- With subagent support, spawn one subagent per folder. With `orga-core` installed, use `fixer`. Each subagent reads only its folder's selected files and the existing `codemap.md`, writes only that file, and returns the one-line **Responsibility** summary. Never two writers on the same file.
- Without subagent support, write the files yourself, one folder at a time.

Content contract (a filled example is in `references/examples.md`):

- `## Responsibility`: the folder's role in standard software-engineering terms (Service Layer, Adapter, CLI entry point ...).
- `## Design`: named patterns and key abstractions, the interfaces that matter.
- `## Flow`: how data and control enter and leave, as concrete call sequences.
- `## Integration`: consumers and dependencies by technical name (modules, hooks, events, endpoints).

Refreshing a filled codemap means reading it and rewriting only the sections that the changed files invalidate. Verify every path a subagent returns against the file system; subagents hallucinate paths.

Document: `✅ Step 4: <n> codemaps written`

⛔ DO NOT continue to step 5 until every affected folder is documented.

### Step 5 - Save the new state `⚡ EXECUTE`

> ⚠️ Real terminal execution required. DO NOT simulate or guess output.

```bash
node "<absolute-SKILL_FOLDER>/scripts/codemap.mjs" update --root "<repo-root>"
```

Document: `✅ Step 5: .codemap/state.json updated with <n> files`

### Step 6 - Root atlas

The root `codemap.md` is the entry point for any agent or human. Create or refresh it; in mode `noop` only check that it exists and lists every folder codemap.

1. `## Project Responsibility`: one paragraph on the project's purpose.
2. `## System Entry Points`: root-level files that matter (`package.json`, `src/index.ts`, `pyproject.toml` ...).
3. `## Directory Map`: one table row per folder codemap with its Responsibility summary and a relative link to the sub-map.

Document: `✅ Step 6: root codemap.md <created|updated|unchanged>`

### Step 7 - Register the atlas in the agent instructions

Agents only use the atlas if their instruction file points to it. At `<repo-root>`:

- `CLAUDE.md` is read by Claude Code, `AGENTS.md` by Codex and OpenCode. Append the section below to every one of these files that exists. If neither exists, create `CLAUDE.md` with it.
- Idempotent: a file that already contains a `## Repository Map` heading is left untouched.

```markdown
## Repository Map

A full codemap is available at `codemap.md` in the project root.

Before working on any task, read `codemap.md` to understand:
- Project architecture and entry points
- Directory responsibilities and design patterns
- Data flow and integration points between modules

For deep work on a specific folder, also read that folder's `codemap.md`.
```

Document: `✅ Step 7: Repository Map registered in <files>` or `⏭️ Step 7: already registered`

Update State Table (`agent_file` set).

### Step 8 - Report

Summarize in German: mode, mapped or refreshed folders, state file path, registration target, and what was deliberately skipped. Remind the user to commit `.codemap/state.json` together with the `codemap.md` files.

Document: `✅ Step 8: Report delivered`

## Rules

- Core code and config only. Tests, docs, translations, build output and dependencies never enter the state file.
- Only affected folders are rewritten; an unchanged folder keeps its codemap byte for byte.
- `.codemap/state.json` and every `codemap.md` are committed artifacts, not scratch files.
- Filled codemaps are updated section by section, never regenerated from scratch without reading them first.
- Codemaps describe what the code does, not what it should do: no TODO lists, no review comments.
- A run that maps zero folders is not a failure when the state reported no changes; say so and finish.
