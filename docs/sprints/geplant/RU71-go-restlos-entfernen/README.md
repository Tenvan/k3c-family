# RU71 · DEV · Go restlos entfernen: go.mod, Taskfile, CI, Doku, Entscheidung 014

- **Status:** geplant
- **Projekt:** RSU
- **Domäne:** DEV
- **Reife:** Entwurf
- **Tickets:** –
- **Start-Commit:** – (wird beim Aktivieren gesetzt: `git rev-parse --short origin/develop`)
- **Spec:** freigegeben
- **Revision:** 1
- **Freigabe:** autonom, Claude, 2026-11-25

## Ausgangslage

Entwurf. Abhängig von RU69 und RU70, beide von Ralf abgenommen. Danach hängt kein Programm mehr an Go.

## Ziel

Go ist aus dem Repo entfernt: `go.mod`/`go.sum`, Go-Tasks, CI-Schritte, golangci-lint, `engine/`, `cmd/`, `tools/*` Go-Teile; `CLAUDE.md`, `codemap.md`, Arbeitsweise und `requirements.md` beschreiben den reinen Rust-Stand; Entscheidung 014 ist umgesetzt. Löschen nur nach Ja von Ralf.

## Beteiligte und Zielgruppen

Agenten räumen auf; 🧑 gibt das Löschen frei und nimmt die Doku ab.

## Anforderungen

Nach der Abnahme von RU69 und RU70: Go-Code, `go.mod`, `go.sum`, `golangci-lint`-Konfiguration, Go-Tasks, CI-Schritte und Go-Voraussetzungen in `requirements.md` entfernen; `CLAUDE.md`, `codemap.md`, `docs/arbeitsweise.md` und die Agenten beschreiben den reinen Rust-Stand; Entscheidung 014 steht als umgesetzt.

## Nicht-Ziele

Neue Funktionen. Änderungen an Frontends.

## Regeln und Einschränkungen

Löschen nur nach Ja von 🧑 (Haltepunkt vor dem Löschen). Golden und `testdata/` bleiben. Datei ≤ 400, Funktion ≤ 60 Zeilen.

Planungsangaben für die Arbeitswellen (`docs/arbeitsweise.md` › Parallelität und Wellen), je eine Zeile, nur IDs und Pfade:

- **Bereich:** `engine/**`, `cmd/**`, `tools/**`, `go.mod`, `go.sum`, `Taskfile.yml`, `.github/workflows/ci.yml`, `requirements.md`, `CLAUDE.md`, `codemap.md`, `docs/arbeitsweise.md`, `docs/decisions/**`, `.claude/agents/**`
- **Abhängig von:** RU69, RU70, RU41
- **Braucht 🧑:** Entscheidung
- **Sperre:** deps, taskfile, exe, golden

## Beispiele

`git ls-files '*.go'` ist leer; `task check:all` ist grün.

## Ausnahme- und Fehlerfälle

Hängt noch ein Programm an Go, bricht die Session ab und legt ein Ticket an.

## Akzeptanzkriterien

- **AC-01** Es gibt keine Go-Dateien und keinen Go-Schritt in `Taskfile.yml` und CI; `task check:all` ist grün.
- **AC-02** `CLAUDE.md`, `codemap.md`, `docs/arbeitsweise.md`, `requirements.md` und die Agenten beschreiben den Rust-Stand.

## Offene Fragen

Ja von 🧑 vor dem Löschen.

## Sessions

| Nr. | Datei | Typ | Agent | Status |
|---|---|---|---|---|

Stichpunkte (Reife Entwurf): 1. Haltepunkt 🧑, dann Go entfernen (AC-01). 2. Doku (AC-02). 3. Review.

## Abnahme

Wird von der Review-Session (Doku-Sprint: letzte Session) ausgefüllt, höchstens fünf Zeilen: Datum, Kriterien
(Verweis auf Session-Ergebnisse), behobene Befunde, neue Tickets und das Abnahme-Ticket für Sim, Balance und Gen
(genau eins, `docs/arbeitsweise.md` › Sim, Balance und Gen). Bis dahin `–`.
