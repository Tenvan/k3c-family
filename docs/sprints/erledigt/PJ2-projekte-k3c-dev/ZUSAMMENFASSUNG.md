# PJ2 · Zusammenfassung

Regelbasiert aus der Sprint-README erzeugt (`plan_digest`, B-696); maßgeblich bleibt [README.md](README.md).

- **Titel:** k3c-dev plant mit Projekten: plan-Tools und Planungsseite
- **Projekt:** PRZ

## Ziel

Projekte werden über MCP angelegt, zugeordnet und nach Rang sortiert, und die Planungsseite zeigt die Arbeit nach Projekten mit verschiebbarem Rang. **Am Ende sichtbar:** Workbench, Reiter Planung: Projekte nach Rang mit Sprints, Fortschritt und nächster Session; `plan_list kind=projekt` antwortet.

## Ergebnis

2026-10-08: AC-01 bis AC-03 geprüft in PJ2.1, AC-04/AC-05 in PJ2.2 (AC-04 mit Übergangsregel zur Prio, Rest B-361), AC-06 bis AC-09 in PJ2.3. Behobener Befund: Panic bei Sprint-README ohne Überschrift in `syncSprintDomain` (PJ2.4). CI-Fix außerhalb SRV: Root-Job nutzt `task install`, damit die Komponententests von k3c-dev (React, Radix) laufen (`.github/workflows/ci.yml`, INF). Neue Tickets: B-361 (Sprint-Felder Prio und Einschiebbar entfallen). Browser-Abnahme der Planungsseite durch 🧑 offen. Version: v0.15.0 vorgeschlagen (Minor: neue Wirkung im Werkzeug k3c-dev).

## Kriterien

| Kriterium | Status | Inhalt |
|---|---|---|
| AC-01 | geprüft | B-357/AC-01 (Projekte in `plan_create`, `plan_get`, `plan_list`, `plan_section`). |
| AC-02 | geprüft | B-357/AC-02 (Ränge lückenlos). |
| AC-03 | geprüft | B-357/AC-03 (Zuordnung pflegt Sprint-Tabelle und Übersicht, Planungstest grün). |
| AC-04 | – | B-357/AC-04 (Domäne je Session, Sprint-Domänen abgeleitet, keine Sprint-Prio). |
| AC-05 | – | B-338/AC-02 (`plan_set` setzt `verworfen`). |
| AC-06 | – | B-358/AC-01 (Seite nach Rang mit Sprints und nächster Session). |
| AC-07 | – | B-358/AC-02 (Rang hoch/runter). |
| AC-08 | – | B-358/AC-03 (ruhend/erledigt eingeklappt, ABN, Ohne Projekt). |
| AC-09 | – | B-358/AC-04 (Prompt-Vorschläge nach Rang). |

## Tickets

| Ticket | Status | Kriterien | Titel |
|---|---|---|---|
| B-357 | erledigt | AC-01 geprüft, AC-02 geprüft, AC-03 geprüft, AC-04 – | Die plan-Tools von k3c-dev legen Projekte an, ordnen Sprints und Tickets zu und … |
| B-358 | erledigt | AC-06 –, AC-07 –, AC-08 –, AC-09 – | Die Planungsseite der Workbench zeigt Projekte nach Rang mit ihren Sprints |
