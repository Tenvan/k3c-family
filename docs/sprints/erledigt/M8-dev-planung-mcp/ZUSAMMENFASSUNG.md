# M8 · Zusammenfassung

Regelbasiert aus der Sprint-README erzeugt (`plan_digest`, B-696); maßgeblich bleibt [README.md](README.md).

- **Titel:** k3c-dev VIII: Planung über MCP, React-Planungsseite, GitHub-Status
- **Projekt:** –

## Ziel

Agenten ändern die Planung nur noch über MCP-Tools, die Planungsseite ist React aus denselben Daten, und je Sprint stehen PR, CI und Merge-Status daneben. Am Ende sichtbar: `plan_create`/`plan_set` im MCP-Katalog, Planungsseite ohne iframe mit PR-Badge im Mock (`npx vite` in `tools/k3c-dev/frontend`).

## Ergebnis

2026-10-04 · AC-01 bis AC-03 und AC-04: M8.1 (PR #104), AC-05 und AC-06: M8.2, AC-07 und AC-08: M8.3, AC-09: M8.4 (`task check`, `task check:dev`, `golangci-lint` grün). Befunde: keine schweren (IDs per Regex, Slugs geprüft, Schreiben nur unter `docs/`, `gh` ohne Shell mit Timeout, Löschen nur für Entwürfe in `geplant/`). Neue Tickets: B-213 (entstand in M8.2). Version: v0.6.0 vorgeschlagen (Minor, Wirkung im Werkzeug: plan_*-Tools, React-Planungsseite, GitHub-Status)

## Kriterien

| Kriterium | Status | Inhalt |
|---|---|---|
| AC-01 | geprüft | Planungs-CRUD im Go-Paket mit Folgeänderungen, getestet im Temp-Repo (B-210/AC-01). |
| AC-02 | geprüft | MCP-Tools `plan_list`, `plan_get`, `plan_create`, `plan_set`, `plan_section`, `plan_delete … |
| AC-03 | geprüft | Pfad- und Werteprüfung ändert bei ungültiger Eingabe keine Datei (B-210/AC-03). |
| AC-04 | – | Arbeitsweise, `CLAUDE.md` und MCP-Instructions nennen die Tools als Weg, Handarbeit nur al … |
| AC-05 | – | `page.html`, `page.go` und Binding `PlanningPage` entfallen (B-211/AC-01). |
| AC-06 | geprüft | React-Planungsseite mit Filter, Karten, Backlog, Detail-Panel und Neuladen ohne Zustandsve … |
| AC-07 | geprüft | GitHub-Stand je Sprint aus `gh` mit Tests auf aufgezeichneten Ausgaben und MCP-Tool `gh_st … |
| AC-08 | geprüft | Planungsseite zeigt PR, CI, Merge-Status und letzten `develop`-Lauf, im Mock mit allen Zus … |
| AC-09 | geprüft | `task check:dev` und `task check` grün. |

## Tickets

| Ticket | Status | Kriterien | Titel |
|---|---|---|---|
| B-210 | erledigt | AC-01 geprüft, AC-02 geprüft, AC-03 geprüft, AC-04 – | Agenten pflegen Tickets, Sprints und Sessions über MCP-Tools von k3c-dev |
| B-211 | erledigt | AC-05 –, AC-06 geprüft | Die Planungsseite von k3c-dev ist eine React-Ansicht aus denselben Daten wie die … |
| B-212 | erledigt | AC-07 geprüft, AC-08 geprüft | k3c-dev zeigt PR, CI und Merge-Konflikte je Sprint aus GitHub |
