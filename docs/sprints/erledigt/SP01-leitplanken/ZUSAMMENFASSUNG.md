# SP01 · Zusammenfassung

Regelbasiert aus der Sprint-README erzeugt (`plan_digest`, B-696); maßgeblich bleibt [README.md](README.md).

- **Titel:** Leitplanken + Go-Gerüst
- **Projekt:** –

## Ziel

Das Komplexitäts-Budget wird für TypeScript und Go automatisch geprüft, und das Repo hat ein Go-Modul, in das die Engine ab SP03 einzieht. Am Ende sichtbar: `npm run check` lokal und in der CI grün; die CI hat einen Go-Job mit Tests, `golangci-lint` und Cross-Build für Windows und Raspberry Pi.

## Ergebnis

**2026-09-30, SP01.4 (autonomer Agent, nur lokal).** Geprüft: 42 Dateien aus `git diff --stat 918f385..main` (lokales `main` 9cc82fd, 7 Commits vor `origin/main`; kein `git fetch`, 🧑: nur lokal). Jede Datei vollständig gelesen; die 11 JSON-Dateien in `data/` als reine Umbenennungen (`git diff -M`: 100 %, keine Wertänderung), `package-lock.json` über den Diff (nur `oxlint` 1.86.0 mit Plattform-Bindings, `@types/node` 26.6.3 → 22.20.4, `undici-types`). Lokal: Node 26.7, Go 1.27.0, golangci-lint 2.14.0. - **AC-01 geprüft:** Messung mit den Zielwerten ohne Ausnahmen: alle 9 Ausnahmen in `.oxlintrc …

## Kriterien

| Kriterium | Status | Inhalt |
|---|---|---|
| AC-01 | geprüft | `npm run lint` (Oxlint) scheitert bei Verstößen gegen die harten Budget-Grenzen; der Besta … |
| AC-02 | geprüft | `npm run check` bündelt Lint, Typecheck und Tests und läuft in der CI grün. |
| AC-03 | geprüft | Die Balancing-Daten liegen in `data/`; der Client importiert von dort, `data/embed.go` bet … |
| AC-04 | geprüft | CI-Job `go`: `go test ./...`, `golangci-lint` mit Budget- und Schichtregeln und Cross-Buil … |
| AC-05 | geprüft | `npm test` scheitert, wenn eine Datei ohne passende Ausnahme über 300 Zeilen liegt, eine A … |
| AC-06 | geprüft | `npm run typecheck` prüft `tests/` (B-033/AC-01). |
| AC-07 | geprüft | Der Regel-Test prüft auch Go-Dateien: über 300 Zeilen nur mit Ausnahme, über 400 nie, über … |
| AC-08 | geprüft | `requirements.md` im Root listet alle vorausgesetzten Installationen mit Version und Prüfb … |

## Tickets

| Ticket | Status | Kriterien | Titel |
|---|---|---|---|
| B-009 | erledigt | AC-01 geprüft, AC-04 geprüft | Komplexitäts-Budget wird automatisch geprüft |
| B-033 | erledigt | AC-06 geprüft | Tests werden typgeprüft |
| B-050 | erledigt | AC-07 geprüft | Die Dateilänge von Go-Code wird wie bei TypeScript geprüft |
| B-052 | erledigt | AC-08 geprüft | Alle vorausgesetzten Installationen stehen in requirements.md |
