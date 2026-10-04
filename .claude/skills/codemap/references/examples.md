# Codemap Examples

Read this file when pattern selection, codemap content, or the root atlas format is unclear.

## Example prompts

- "Kartiere dieses Repository, ich kenne den Code nicht."
- "Erstelle einen Repository-Atlas mit einer codemap.md pro Ordner."
- "Die Codemaps sind veraltet, bitte aktualisieren."
- "Map the `apps/backend` tree only."

## Pattern sets per stack

Always exclude tests, docs, build output, and dependencies. Add generated code and translations when the repository has them.

### TypeScript / Node

```bash
--include "src/**/*.ts" --include "src/**/*.tsx" --include "package.json" --include "tsconfig.json" \
--exclude "**/*.test.ts" --exclude "**/*.spec.ts" --exclude "**/__tests__/**" \
--exclude "node_modules/**" --exclude "dist/**" --exclude "build/**" --exclude "**/*.d.ts"
```

### Python

```bash
--include "**/*.py" --include "pyproject.toml" \
--exclude "tests/**" --exclude "**/test_*.py" --exclude "**/conftest.py" \
--exclude "**/__pycache__/**" --exclude ".venv/**" --exclude "build/**"
```

### Monorepo (apps + libs)

Ask via `interviewe` which apps matter, then scope the include globs to them:

```bash
--include "apps/backend/src/**/*.ts" --include "libs/**/src/**/*.ts" --include "package.json" \
--exclude "**/*.test.ts" --exclude "**/*.spec.ts" --exclude "node_modules/**" --exclude "dist/**"
```

Use `--exception <path>` for a single file that an exclude glob would drop but that must stay in the map, for example a root `README.md` that doubles as the architecture overview.

## Example folder codemap

```markdown
# src/agents/

## Responsibility
Defines agent personalities and manages their configuration lifecycle (Configuration Layer).

## Design
Each agent is a prompt plus a permission set. Strategy pattern for model routing:
- default prompts per agent (`orchestrator.ts`, `explorer.ts`, ...)
- user overrides merged from the project config file
- permission wildcards expanded for skill and MCP access control

## Flow
1. Plugin loads → calls `getAgentConfigs()`
2. Reads the user config preset
3. Merges defaults with overrides
4. Applies permission rules (wildcard expansion)
5. Returns agent configs to the host

## Integration
- Consumed by: plugin entry point (`src/index.ts`)
- Depends on: config loader (`src/config/`), skills registry (`src/skills/`)
```

## Example root atlas

```markdown
# Repository Atlas: example-plugin

## Project Responsibility
A low-latency agent orchestration plugin focusing on specialised sub-agent delegation.

## System Entry Points
- `src/index.ts`: plugin initialisation and host integration
- `package.json`: dependency manifest and build scripts
- `example-plugin.json`: user configuration schema

## Directory Map
| Directory | Responsibility Summary | Detailed Map |
|-----------|------------------------|--------------|
| `src/agents/` | Defines agent personalities and manages model routing. | [View Map](src/agents/codemap.md) |
| `src/features/` | Core logic for session state and multiplexer integration. | [View Map](src/features/codemap.md) |
| `src/config/` | Configuration loading pipeline and environment variable injection. | [View Map](src/config/codemap.md) |
```

## Refresh example

Input shape:

- `changes` reports `~ src/config/loader.ts` and `- src/config/legacy.ts`, `2 folders affected: ./ src/`
- `src/config/codemap.md` exists and describes `legacy.ts` in its Flow section

Expected direction:

- rewrite only the `## Flow` and `## Integration` sections of `src/config/codemap.md`, dropping `legacy.ts`
- leave `src/agents/codemap.md` untouched
- refresh the `src/config/` row of the root atlas if its Responsibility summary changed
- run `update` afterwards so the state matches the new hashes
