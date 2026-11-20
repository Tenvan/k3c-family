# WZ61 · DEV, INF · Workbench-Nachträge: schlanke Planungsdaten, alte EXE in bin/ aufräumen

- **Status:** aktiv
- **Projekt:** WZG
- **Domäne:** DEV, INF
- **Reife:** bereit
- **Tickets:** B-909, B-1007
- **Start-Commit:** – (wird beim Aktivieren gesetzt: `git rev-parse --short origin/develop`)
- **Spec:** freigegeben
- **Revision:** 1
- **Freigabe:** 🧑 2026-11-15

## Ausgangslage

Zwei Nachträge an den Werkzeugen (Stand 2026-11-15): Die Planungsseite der Workbench lädt bei jeder Änderung die ganzen `PlanningData` (5,5 MiB, 1,9 s; gemessen in WZ60.1, B-909). In `bin/` liegen rund 110 MiB EXE entfernter Programme, `task ops:aufraeumen` kennt `bin/` nicht (B-1007). WZ60 hat alle Sessions fertig und wartet nur auf die Abnahme.

## Ziel

Die Planungsseite bleibt bei laufenden Agenten aktuell, ohne je Änderung mehrere MiB zu laden, und `bin/` lässt sich mit einem Befehl von alten EXE befreien. Am Ende sichtbar: Messwerte unter 500 KiB je Nachladen im Test, `task ops:aufraeumen -- --dry` listet die fünf alten Programme.

## Beteiligte und Zielgruppen

🧑 und Agenten an der Workbench; Agenten DEV und INF; Prüfer `k3c-workbench`; 🧑 gibt B-1007 frei und startet das Löschen.

## Anforderungen

`B-909 › Anforderungen`; `B-1007 › Anforderungen`.

## Nicht-Ziele

Neue Funktionen der Planungsseite; MCP-Tools `plan_*`; `dist/` und Worktrees aufräumen.

## Regeln und Einschränkungen

Oberfläche nach Atomic Design über `@webkit`. Die laufende Workbench-EXE (Port 5180) wird im Sprint nicht neu gebaut oder neu gestartet; entwickelt wird gegen `task k3c-workbench` (Dev). Löschen ohne `--dry` nur durch 🧑 (WZ61.5). WZ61.3 ändert zusätzlich `scripts/Taskfile.yml` (Ressource `taskfile`, nur die Beschreibung eines Tasks).

- **Bereich:** `tools/k3c-workbench/internal/planning/**`, `tools/k3c-workbench/app_planning.go`, `tools/k3c-workbench/app.go`, `tools/k3c-workbench/uiapi.go`, `tools/k3c-workbench/frontend/wailsjs/**`, `tools/k3c-workbench/frontend/src/planning/**`, `tools/k3c-workbench/frontend/src/api/**`, `tools/k3c-workbench/frontend/src/lib/useLiveReload.ts`, `tools/cleanup/**`, `scripts/Taskfile.yml`
- **Abhängig von:** WZ60.1
- **Braucht 🧑:** Freigabe
- **Sperre:** exe

## Beispiele

Ein Agent ändert per `plan_set` einen Status → die Seite lädt wenige KiB nach. `task ops:aufraeumen -- --dry` → Liste der alten EXE mit Größe, nichts gelöscht.

## Ausnahme- und Fehlerfälle

Viele Änderungen auf einmal (Merge) → die Seite lädt einmal alles. Alte EXE läuft noch → Meldung mit Grund, Rest wird aufgeräumt.

## Akzeptanzkriterien

- **AC-01** Nachladen einer Ticket-Änderung unter 500 KiB, Listen-Daten unter 1,5 MiB (`B-909/AC-01`).
- **AC-02** Die Seite lädt nach `planning:changed` mit einer ID nur diese ID (`B-909/AC-02`).
- **AC-03** Die Aufschlüsselung der 5,5 MiB steht im Ergebnis (`B-909/AC-03`).
- **AC-04** `ops:aufraeumen` löscht in `bin/` genau die EXE entfernter Programme, `--dry` löscht nichts (`B-1007/AC-01`, `B-1007/AC-02`, `B-1007/AC-03`).
- **AC-05** Nach Ralfs Ja sind die fünf alten EXE aus `bin/` entfernt (`B-1007/AC-04`).

## Offene Fragen

- B-909: erst Fließtexte aus den Listen-Daten, dann nur geänderte IDs nachladen? → Default: ja (vorläufig, 🧑 bestätigt).
- B-1007: feste Liste der Programme statt „alles Unbekannte“? → Default: feste Liste (vorläufig, 🧑 bestätigt).
- B-1007: Agent startet nur `--dry`, das Löschen startet 🧑? → Default: ja (vorläufig, 🧑 bestätigt).

## Sessions

| Nr. | Datei | Typ | Agent | Status |
|---|---|---|---|---|
| WZ61.1 | `WZ61.1-planungsdaten-backend.md` | Umsetzung | autonom | fertig |
| WZ61.2 | `WZ61.2-planungsseite-laedt-nur-geaendertes.md` | Umsetzung | autonom | fertig |
| WZ61.3 | `WZ61.3-bin-alte-exe-aufraeumen.md` | Umsetzung | autonom | fertig |
| WZ61.4 | `WZ61.4-review.md` | Review | autonom | fertig |
| WZ61.5 | `WZ61.5-bin-aufraeumen-am-pc.md` | Workshop | Mensch | fertig |

Bei `Reife: Entwurf` genügen Stichpunkte mit den Kriterien in Klammern statt Session-Dateien. Vor dem Aktivieren
wird jede Session als eigene Datei nach `docs/vorlagen/session.md` geschrieben. Keine eigenen Sim- oder
Balance-Sessions vor der Abnahme: Sim, Balance, Gen und `check:race` laufen im Abnahme-Ticket (B-742).

## Abnahme

2026-11-16: AC-01 bis AC-04 erfüllt (WZ61.4 › Ergebnis), AC-05 offen bei Ralf (WZ61.5).
Behoben: Nachladefehler ersetzte die Seite, Lint complexity. Keine neuen Tickets. Abnahme-Ticket für `check:race`: B-1022.
