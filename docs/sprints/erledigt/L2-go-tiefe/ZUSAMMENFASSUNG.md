# L2 · Zusammenfassung

Regelbasiert aus der Sprint-README erzeugt (`plan_digest`, B-696); maßgeblich bleibt [README.md](README.md).

- **Titel:** Go-Tiefe wie TypeScript zählen
- **Projekt:** –

## Ziel

Die harte Grenze Verschachtelung 4 wird in Go genauso gezählt wie in TypeScript, bevor in SP03 der erste echte Go-Code entsteht. Am Ende sichtbar: `npm run check:go` scheitert bei `range` → 4× `if` und bleibt bei einer flachen `else if`-Kette grün.

## Ergebnis

**2026-09-30, L2.2 (autonomer Agent, nur lokal).** Geprüft: 9 Dateien aus `git diff --stat 27cf540..main` (lokales `main` 9b4e1ac; kein `git fetch`, 🧑: nur lokal), jede vollständig gelesen: `.golangci.yml`, `docs/arbeitsweise.md`, `docs/sprints/README.md`, die drei Dateien dieses Sprints, `requirements.md`, `tests/nesting_test.go`, `tests/projectRules.test.ts`. Lokal: Go 1.27.0, golangci-lint 2.14.0. Gegenproben in `engine/probe/` (Go, `go test ./tests/`) und `src/` (TypeScript, `npx oxlint`), danach gelöscht. - **AC-01 geprüft:** `for _, x := range` → 4× `if` → rot (Tiefe 5), `for range` → 3× …

## Kriterien

| Kriterium | Status | Inhalt |
|---|---|---|
| AC-01 | geprüft | `range` → 4× `if` scheitert, `range` → 3× `if` nicht (B-057/AC-01). |
| AC-02 | geprüft | Eine flache `else if`-Kette aus 6 Zweigen bleibt grün (B-057/AC-02). |
| AC-03 | geprüft | `for`, `switch` und `select` zählen je eine Ebene, ein Funktionsliteral beginnt bei 0 (B-0 … |
| AC-04 | geprüft | `revive` ist entfernt, `.golangci.yml` und `docs/arbeitsweise.md` › Werkzeuge nennen die n … |
| AC-05 | geprüft | Die Dateilänge von `.go`-Dateien in `tests/` wird wie unter `data/`, `engine/`, `cmd/` gep … |

## Tickets

| Ticket | Status | Kriterien | Titel |
|---|---|---|---|
| B-057 | erledigt | AC-01 geprüft, AC-02 geprüft, AC-03 geprüft, AC-04 geprüft | Die Go-Verschachtelung zählt `for range` und `else if` wie TypeScript |
