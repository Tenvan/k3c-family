# SP00 · Zusammenfassung

Regelbasiert aus der Sprint-README erzeugt (`plan_digest`, B-696); maßgeblich bleibt [README.md](README.md).

- **Titel:** Arbeitsweise einführen
- **Projekt:** –

## Ziel

Das Projekt hat eine feste Arbeitsweise (Tickets → Sprints → Sessions, Review-Pflicht, Komplexitäts-Budget), eine Architektur-Entscheidung für die Go-Engine und einen Planer, den ein Cloud-Agent ohne Planungs-Werkzeuge abarbeiten kann. Am Ende sichtbar: `docs/arbeitsweise.md`, `docs/sprints/`, `docs/backlog/`, `tests/planning.test.ts` grün.

## Ergebnis

**2026-09-30, SP00.5 (autonomer Agent).** Geprüft: 85 Dateien aus `git diff --stat df6e1de..origin/main -- docs tests/planning.test.ts CLAUDE.md README.md .github` (86 Einträge; `docs/game-design.md` nicht, dort nur Reittiere aus PR #15; `docs/sessions.md` als Löschung geprüft). Jede Datei vollständig gelesen, migrierte Tickets und Sprints gegen ihren Stand vor SP00.4 (`55d15cb`) verglichen. - **AC-01 geprüft:** `docs/arbeitsweise.md` beschreibt Ticket → Sprint → Session, Review-Session mit Checkliste und Komplexitäts-Budget; `docs/backlog/README.md` und `.github/pull_request_template.md` lieg …

## Kriterien

| Kriterium | Status | Inhalt |
|---|---|---|
| AC-01 | geprüft | `docs/arbeitsweise.md` beschreibt Tickets → Sprints → Sessions, Review-Pflicht und Komplex … |
| AC-02 | geprüft | Entscheidung 001 (Go-Engine) ist beschlossen (B-001/AC-01, B-003/AC-01). |
| AC-03 | geprüft | Sprints und Tickets liegen als Dateien nach Pflicht-Vorlagen, `tests/planning.test.ts` prü … |
| AC-04 | geprüft | Tickets und Sprints sind Specs nach SDD, `tests/planning.test.ts` prüft Spec-Felder, Krite … |

## Tickets

| Ticket | Status | Kriterien | Titel |
|---|---|---|---|
| B-001 | erledigt | AC-02 geprüft | Server bekommt ein tragfähiges Framework, falls mehr Leistung nötig wird |
| B-003 | erledigt | AC-02 geprüft | Server-Sprache ist entschieden |
| B-044 | erledigt | AC-03 geprüft | Sprints und Tickets liegen als Dateien nach Pflicht-Vorlagen |
| B-045 | erledigt | AC-04 geprüft | Tickets und Sprints sind Specs nach Spec-Driven Development |
