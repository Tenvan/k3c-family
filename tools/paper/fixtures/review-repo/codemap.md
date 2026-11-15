# tools/paper/fixtures/review-repo/

## Responsibility
Erzeugt ein kleines Test-Git-Repo mit allen Review-Fällen der Papierschnitt-Grafik (RV1, `docs/standards/paper-mcp.md` › Review-Test-Repo), damit Review-Code der Workbench gegen feste Commits und einen festen Arbeitsstand getestet werden kann.

## Design
- `erzeuge.ts`, einzige Funktion `erzeugeReviewRepo(ziel)` → `{head, basis}` (kurze Commit-Hashes):
  - Commit 1 (Basis): `public/grafik/stil.paper.json` (Kopie aus dem Repo), `figuren/monarch`, `gebaeude/mauer` (aus `../specs/`), `requisiten/fass` und `requisiten/banner` (kleine `requisite`-Specs).
  - Commit 2: `reittiere/pferd` neu, `fass` gelöscht, Material `stein` im Stil heller.
  - Arbeitsstand (nicht committet): Umhang des Monarchen `purpur`, `banner` mit fremdem Material `scharlach` (ungültig).
  - Basis HEAD → Fälle geändert und ungültig; Basis HEAD~1 → alle fünf Fälle (neu, geändert, gelöscht, ungültig, Stil).
- Deterministisch: feste Autor- und Commit-Daten über `GIT_*`-Umgebung, `core.autocrlf=false`, Branch `develop`; Specs ohne `$schema` (im Test-Repo gibt es das Schema nicht).
- Direktaufruf (`import.meta.main`): `bun tools/paper/fixtures/review-repo/erzeuge.ts <leerer Zielordner>` gibt ein JSON-Objekt aus, ohne Ziel Exit 2.

## Flow
`erzeugeReviewRepo(ziel)` → `git init` → Basis-Dateien schreiben → Commit → Änderungen → Commit → Arbeitsstand schreiben → Hashes zurück.

## Integration
- Aufrufer: `tools/paper/fixtures/beispiele.ts` (Beispielantworten `ui/*.json`), `tools/paper/fixtures/stub.test.ts`; die Go-Tests `tools/k3c-workbench/internal/review/helpers_test.go` und `internal/assetsui/helpers_test.go` starten das Skript.
- Liest `tools/paper/fixtures/specs/*.paper.json` und `public/grafik/stil.paper.json`; braucht `git` im PATH.
