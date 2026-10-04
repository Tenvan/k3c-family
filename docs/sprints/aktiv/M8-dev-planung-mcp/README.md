# M8 · SRV · k3c-dev VIII: Planung über MCP, React-Planungsseite, GitHub-Status

- **Status:** aktiv
- **Domäne:** SRV
- **Reife:** bereit
- **Einschiebbar:** ja
- **Tickets:** B-210, B-211, B-212
- **Start-Commit:** c00cbd6
- **Spec:** freigegeben
- **Revision:** 2
- **Freigabe:** 2026-10-04, Chat (Ralf), Revision 2 (Ergänzung Glossar), durch 🧑; umfasst B-210, B-211, B-212

## Ausgangslage

k3c-dev liest die Planung (`internal/planning/`) und zeigt „Sprints & Backlog“ als von Go erzeugtes HTML im iframe (`page.html`). Planungsänderungen machen Agenten von Hand, GitHub-Stand (PR, CI, Konflikt) sieht man nur auf GitHub. Details: B-210, B-211, B-212 › Ausgangslage.

## Ziel

Agenten ändern die Planung nur noch über MCP-Tools, die Planungsseite ist React aus denselben Daten, und je Sprint stehen PR, CI und Merge-Status daneben. Am Ende sichtbar: `plan_create`/`plan_set` im MCP-Katalog, Planungsseite ohne iframe mit PR-Badge im Mock (`npx vite` in `tools/k3c-dev/frontend`).

## Beteiligte und Zielgruppen

Agenten (Planung, autonome Sessions), 🧑 (Überblick, Freigaben, Merge).

## Anforderungen

B-210 › Anforderungen, B-211 › Anforderungen, B-212 › Anforderungen.

## Nicht-Ziele

B-210 › Nicht-Ziele, B-211 › Nicht-Ziele, B-212 › Nicht-Ziele. Keine Abnahme im echten Fenster in diesem Sprint (Mock und Tests reichen; Fenster bei Bedarf als Ticket).

## Regeln und Einschränkungen

- Domäne SRV, Code nur in `tools/k3c-dev/`. Ausnahme mit dieser Freigabe: M8.1 ändert in `docs/arbeitsweise.md` und `CLAUDE.md` nur die Absätze zum Weg für Planungsänderungen (INF-Dateien, wie LT1 beim `Taskfile.yml`).
- Einschiebbar: LT1 belegt die Domäne SRV; M8 berührt nur das Entwickler-Werkzeug und keine Server-Dateien.
- Keine neue Abhängigkeit (Go-SDK, React, Radix reichen; `gh` ist ein externes Programm von 🧑). Komplexitäts-Budget aus `docs/arbeitsweise.md`.
- Schreiben nur unter `docs/sprints/` und `docs/backlog/`, Pfade nur aus geprüften IDs (B-210 › Regeln).

## Beispiele

B-210 › Beispiele, B-211 › Beispiele, B-212 › Beispiele.

## Ausnahme- und Fehlerfälle

B-210, B-211, B-212 › Ausnahme- und Fehlerfälle.

## Akzeptanzkriterien

- **AC-01** Planungs-CRUD im Go-Paket mit Folgeänderungen, getestet im Temp-Repo (B-210/AC-01).
- **AC-02** MCP-Tools `plan_list`, `plan_get`, `plan_create`, `plan_set`, `plan_section`, `plan_delete` registriert, mit Instructions und Roundtrip-Test (B-210/AC-02).
- **AC-03** Pfad- und Werteprüfung ändert bei ungültiger Eingabe keine Datei (B-210/AC-03).
- **AC-04** Arbeitsweise, `CLAUDE.md` und MCP-Instructions nennen die Tools als Weg, Handarbeit nur als Rückfall (B-210/AC-04).
- **AC-05** `page.html`, `page.go` und Binding `PlanningPage` entfallen (B-211/AC-01).
- **AC-06** React-Planungsseite mit Filter, Karten, Backlog, Detail-Panel und Neuladen ohne Zustandsverlust, im Mock und mit Vitest; Ansicht „Glossar“, wenn `docs/glossar.md` existiert (B-211/AC-02, B-211/AC-03, B-211/AC-04).
- **AC-07** GitHub-Stand je Sprint aus `gh` mit Tests auf aufgezeichneten Ausgaben und MCP-Tool `gh_status` (B-212/AC-01, B-212/AC-03).
- **AC-08** Planungsseite zeigt PR, CI, Merge-Status und letzten `develop`-Lauf, im Mock mit allen Zuständen (B-212/AC-02).
- **AC-09** `task check:dev` und `task check` grün.

## Offene Fragen

keine

## Sessions

| Nr. | Datei | Typ | Agent | Status |
|---|---|---|---|---|
| M8.1 | `M8.1-planung-mcp.md` | Umsetzung | autonom | fertig |
| M8.2 | `M8.2-planungsseite-react.md` | Umsetzung | autonom | offen |
| M8.3 | `M8.3-github-status.md` | Umsetzung | autonom | offen |
| M8.4 | `M8.4-review.md` | Review | autonom | offen |

## Abnahme

–
