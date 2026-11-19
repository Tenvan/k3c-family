# WE4.3 · Welten-Editor: Querschnitt und Seekarte (B-963)

- **Status:** fertig
- **Typ:** Umsetzung
- **Agent:** autonom
- **Domäne:** DEV
- **Lane:** –
- **Modell:** –
- **Umgebung:** offline
- **Branch:** sprint/we4
- **Abhängig von:** WE4.1
- **Tickets:** B-963
- **Kriterien:** AC-03

## Ziel

Welten-Seite zeigt Querschnitt nach Etappen mit Seekarte; Etappe per Editor und MCP setzbar (B-963).

## Kontext

Seite `tools/k3c-workbench/frontend/src/world/` (WorldPage, useWorld, levelView), MCP `internal/mcpsrv/tools_world.go`, Schreiben `internal/world/edit.go`. Vorbild: Mock `welten-querschnitt.html`.

## Erlaubte Dateien

- `tools/k3c-workbench/**`
- `tools/webkit/**`

## Nicht-Ziele

Drag & Drop, echte Marker-Positionen.

## Schritte

1. MCP und Schreiben um `etappe`.
2. Querschnitt-Komponente.
3. Seekarte mit Sprung.
4. Tests und Browser-Prüfung.

## Fertig, wenn

- [ ] AC-01, AC-02, AC-03 aus B-963

## Prüfen

```bash
task check:diff
```

Einmal vor dem Commit; während der Arbeit nur gezielte Tests (`docs/arbeitsweise.md` › Tempo im Lauf).
Review-Session statt dessen: `task check`, `task check:go` (nacheinander) und das Abnahme-Ticket des Sprints anlegen.
Sim-, Balance- und Gen-Läufe (`sim_test`, Balancing, `gen:paper` samt Golden, `check:race`) nur, wenn diese Session das
betroffene Feature entwickelt; sonst laufen sie im Abnahme-Ticket (`docs/arbeitsweise.md` › Sim, Balance und Gen).
Manuelle Prüfungen (Browser, Xbox, TV) nur, wenn diese Datei sie nennt und 🧑 sie für diesen Lauf freigegeben hat.

## Ergebnis

AC-01–AC-03 (B-963) geprüft: Vitest `Overview.test.tsx`, `CrossLoad.test.tsx`, Go `internal/world/etappe_test.go`; Browser-Pane mit Sengoku (Dev-Instanz im Worktree): Seekarte springt, Etappe-Änderung mit Hafen-Fehler abgelehnt und zurückgesetzt. Review k3c-workbench: Befunde behoben.
