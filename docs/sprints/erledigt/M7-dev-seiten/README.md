# M7 · SRV · k3c-dev VII: Seiten Tasks, Planung und Git

- **Status:** erledigt
- **Projekt:** –
- **Domäne:** SRV
- **Reife:** bereit
- **Tickets:** B-171
- **Start-Commit:** 433dd07
- **Spec:** rückwirkend
- **Revision:** 2
- **Freigabe:** – (rückwirkend: aus der Umsetzung vom 2026-10-03 abgeleitet, ohne Freigabe)

## Ausgangslage

k3c-dev hat nach M4 und M5 die Seiten Dienste, Logs und MCP. Die Workbench der ErpApi hat zusätzlich Tasks, Planung und Git. Details: B-171 › Ausgangslage.

## Ziel

Tasks, Planung (mit Plan und Fragenkatalog) und Git sind in k3c-dev bedienbar, im Mock und im Fenster. Am Ende sichtbar: drei neue Reiter in `task k3c-dev`, im Browser-Pane mit `npx vite` im Frontend.

## Beteiligte und Zielgruppen

Entwickler und 🧑 (Abnahme im Fenster).

## Anforderungen

B-171 › Anforderungen.

## Nicht-Ziele

B-171 › Nicht-Ziele (MCP-Tools, KI-Commit, Release, Historie, Allow-Liste, Radar-Wächter).

## Regeln und Einschränkungen

Nur `tools/k3c-dev/`; Komplexitäts-Budget wie in `docs/arbeitsweise.md`; Staging bleibt bei der Person (kein MCP-Tool); keine neue Abhängigkeit.

## Beispiele

B-171 › Beispiele.

## Ausnahme- und Fehlerfälle

B-171 › Ausnahme- und Fehlerfälle.

## Akzeptanzkriterien

- **AC-01** Seite Tasks mit Katalog, Start, Stopp und Live-Ausgabe (B-171/AC-01).
- **AC-02** Seite Planung mit Sprints, Backlog, Plan und Fragenkatalog, die sich bei Dateiänderungen selbst aktualisiert (B-171/AC-02).
- **AC-03** Seite Git mit Staging und Commit (B-171/AC-03).
- **AC-04** Standard-Theme Dark/Light und Mock für alle Seiten (B-171/AC-04, B-171/AC-05).
- **AC-05** `task check:dev` grün (B-171/AC-06).
- **AC-06** 🧑 hat die Seiten im echten Fenster abgenommen (B-171/AC-07).

## Offene Fragen

keine

## Sessions

| Nr. | Datei | Typ | Agent | Status |
|---|---|---|---|---|
| M7.1 | `M7.1-seiten.md` | Umsetzung | autonom | fertig |
| M7.2 | `M7.2-abnahme-fenster.md` | Workshop | Mensch | fertig |
| M7.3 | `M7.3-review.md` | Review | autonom | fertig |

## Abnahme

2026-10-03: AC-01 bis AC-04 geprüft in M7.1 (Tests und Mocks), AC-05 in M7.3 (`task check:dev` und `task check` grün), AC-06 durch 🧑 in M7.2 (echtes Fenster, bestanden).
Review ohne schweren Sicherheitsbefund; behoben: `TasksPage` auf unter 60 Zeilen, `TestFindRoot` mit Temp-Ordner im Repo. Neue Tickets: keine.
