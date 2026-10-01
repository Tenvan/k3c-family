# B-073 · Alle Aufrufer nutzen Go Task statt npm-Skripte

- **Domäne:** INF
- **Typ:** Schuld
- **Prio:** hoch
- **Status:** offen
- **Sprint:** –
- **Erstellt:** 2026-10-01
- **Spec:** Entwurf
- **Revision:** 1
- **Freigabe:** –

## Ausgangslage

`Taskfile.yml` (Go Task) ist der Einstieg für alle Befehle; `CLAUDE.md` und `requirements.md` sind umgestellt.
Noch auf `npm run …` verweisen: `package.json` (Skripte), die Workflows in `.github/workflows/` (ci, deploy-pages,
release), `Dockerfile`, `tools/k3c-dev` (Prüfkatalog `check_run` mit Zielen `npm:check`, `npm:test` …,
`services.json`, `internal/mcpsrv/instructions.md`, `tools.go`), `README.md`, `docs/arbeitsweise.md`,
`docs/vorlagen/session.md`, `.github/pull_request_template.md`, Meldungen in `server/server.mjs` und `cmd/k3c-server`.

## Ziel

Genau ein Weg, Aufgaben auszuführen: `task <name>`. Keine doppelte Pflege in `package.json`.

## Beteiligte und Zielgruppen

Entwickler und Coding-Agenten; die CI. Entscheidung über den CI-Umbau: 🧑.

## Anforderungen

- Skripte in `package.json` entfallen (nur Abhängigkeiten bleiben), alle Aufrufer rufen `task`.
- k3c-dev führt Prüfungen über `task` aus (Katalog und Tests in `internal/mcpsrv` anpassen).
- CI installiert Task (`go-task/setup-task`) und ruft dieselben Tasks wie lokal.

## Nicht-Ziele

Neue Aufgaben oder inhaltliche Änderungen an Prüfungen.

## Regeln und Einschränkungen

`tests/planning.test.ts` prüft Vorlagen; Vorlagen-Text (`session.md`) bleibt strukturgleich.

## Beispiele

`task check` lokal und in der CI liefert dasselbe Ergebnis wie bisher `npm run check`.

## Ausnahme- und Fehlerfälle

Fehlt Task auf dem Rechner, nennen Fehlermeldungen `requirements.md`.

## Akzeptanzkriterien

- **AC-01** `grep -rn "npm run" .` außerhalb von `docs/sprints/erledigt/` und `node_modules/` liefert keinen Treffer.
- **AC-02** CI läuft grün mit Tasks.
- **AC-03** `check_run` in k3c-dev startet die Prüfungen über `task`; `task check:dev` grün.

## Offene Fragen

keine

## Notizen

Task unter Windows: `winget install --id Task.Task -e --source winget` (neues Terminal öffnen, sonst fehlt der PATH).
