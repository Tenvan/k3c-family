# codemap (Skill)

## Zweck

Kartiert ein Repository hierarchisch: pro relevantem Ordner eine `codemap.md` (Responsibility, Design, Flow, Integration) plus ein Root-Atlas als Einstiegspunkt für Agenten und Menschen. Ein gebündeltes Node-Skript wählt die Kern-Code-Dateien aus, speichert ihre Hashes in `.codemap/state.json` und meldet bei späteren Läufen nur die Ordner, deren Dateien sich geändert haben. Teure Operation, läuft nur auf ausdrückliche Anweisung.

## Voraussetzungen

- `node` (Node.js 18+) auf dem `PATH`, keine weiteren Abhängigkeiten
- Schreibbares Repository; `.codemap/state.json` und alle `codemap.md` werden committed
- `SKILL_FOLDER`: je Agent/Subagent Runner-Pfad des geladenen `SKILL.md` zuerst, sonst die fünf workspace-root-Kandidaten für `codemap`; lesbares Frontmatter und `scripts/codemap.mjs` prüfen, bei null/mehreren Treffern abbrechen, Pfad direkt als `"<absolute-SKILL_FOLDER>/..."` einsetzen
- Optional: Subagenten (Agent-Tool). Mit installiertem `orga-core` schreibt `fixer` je einen Ordner; ohne Subagenten schreibt der Haupt-Agent sequenziell

## Ablauf

1. `changes` ausführen: kein State → `init`, keine Änderungen → fertig, sonst Liste der betroffenen Ordner
2. Bei `init` Include-/Exclude-Globs für Kern-Code ableiten (Tests, Docs, Build, Dependencies bleiben draußen), bei mehrdeutigem Stack per `interviewe` klären, dann `init` ausführen
3. Pro betroffenem Ordner `codemap.md` schreiben oder abschnittsweise aktualisieren
4. `update` ausführen, Root-Atlas `codemap.md` pflegen
5. Abschnitt `## Repository Map` in `CLAUDE.md` (Claude Code) bzw. `AGENTS.md` (Codex, OpenCode) eintragen, idempotent

```bash
node "<absolute-SKILL_FOLDER>/scripts/codemap.mjs" init --root . --include "src/**/*.ts" --exclude "**/*.test.ts"
node "<absolute-SKILL_FOLDER>/scripts/codemap.mjs" changes --root .
node "<absolute-SKILL_FOLDER>/scripts/codemap.mjs" update --root .
```

## Ausgabe

- `.codemap/state.json`: Datei- und Ordner-Hashes plus die verwendeten Muster
- `codemap.md` in jedem kartierten Ordner, Root-`codemap.md` als Atlas mit Verzeichnistabelle
- `## Repository Map`-Abschnitt in der Agenten-Instruktionsdatei
- Deutscher Abschlussbericht: Modus, kartierte Ordner, State-Datei, Registrierung

## Struktur

- `SKILL.md`: Trigger, Execution Contract, Workflow mit State Table und Gates
- `scripts/codemap.mjs`: CLI (`init`, `changes`, `update`), nur Node-Stdlib
- `scripts/codemap.test.mjs`: Selbsttest mit `node:test`
- `references/examples.md`: gefüllte Beispiel-Codemap, Root-Atlas, Muster je Stack
- `references/eval-notes.md`: Review-Kriterien
- `evals/evals.json`: realistische Testprompts

## Test

```bash
node --test "<absolute-SKILL_FOLDER>/scripts/codemap.test.mjs"
```

## Abgrenzung

- Für Komplexitätsmetriken `complexity` verwenden, für „wo liegt X"-Fragen `explorer` oder Grep.

## Quelle

Übernommen aus [oh-my-opencode-slim](https://github.com/alvinunreal/oh-my-opencode-slim) (`src/skills/codemap`, Commit `3013dc25`). Anpassungen für diesen Marketplace:

- fester Pfad `~/.config/opencode/skills/codemap` durch die logische `SKILL_FOLDER`-Auflösung ersetzt
- Registrierung in `CLAUDE.md` statt nur `AGENTS.md`
- Delegation an `Fixer` ist optional (Agent-Tool oder `orga-core:fixer`)
- Test von `bun:test` auf `node:test` umgestellt, CLI-Roundtrip als Prüfung ergänzt; `init` listet die angelegten Ordner
- State-Datei nach `.codemap/state.json` verlegt
