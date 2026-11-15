# WE5.4 · Overrides für hub › sites: Level-Generator liest den Wertesatz der Welt (B-983)

- **Status:** offen
- **Typ:** Umsetzung
- **Agent:** autonom
- **Domäne:** SIM
- **Lane:** –
- **Modell:** opus
- **Umgebung:** offline
- **Branch:** sprint/we5
- **Abhängig von:** WE5.3
- **Tickets:** B-983
- **Kriterien:** AC-04, AC-08

## Ziel

`hub.sites` und `hub.islandSites` sind übersteuerbar; Level-Generator und Simulation rechnen mit den Plätzen der Welt (B-983).

## Kontext

Eigener Worktree mit Dev-Workbench (B-925, Agent `k3c-sim-dev`). Heute gesperrt: `engine/sim/world_values_locked.go` (Liste nach `docs/rules/welten.md` § 1.3). Leser in der Basis: `engine/level/lines.go` (Hub- und Insel-Plätze), `ruins.go` (Ruinen-Lage), `sites.go`, `guards.go`, `planned.go` (`sitePlacement`, bleibt gesperrt), dazu `engine/sim/world_lines.go`. Planungsprüfung: `level.ValidatePlanFor`, aufgerufen in `engine/sim/island_data.go`. `engine/level` importiert nichts aus `engine/sim`: Plätze werden als Wert übergeben. Entscheidung B-692 (sperren statt durchreichen) wird für diese zwei Felder aufgehoben. Golden dürfen sich nicht ändern; ändern sie sich, stoppen und Ticket anlegen.

## Erlaubte Dateien

- `engine/level/**`
- `engine/sim/**`
- `docs/rules/welten.md`

## Nicht-Ziele

`sitePlacement`, `payRangeUnits`, `stripUnits` entsperren; neue Platz-Arten; Workbench; `task go:golden`; Simulation.

## Schritte

1. Alle Leser von `hub.sites`/`islandSites` in `engine/level` finden und auf übergebene Werte umstellen (Standard = Basis).
2. Tests zuerst: Welt mit geändertem Platz → Level und Simulation zeigen denselben Platz; Fehlerfälle (außerhalb, Überlappung).
3. Sperre für die zwei Felder aufheben, Prüfung ergänzen.
4. `welten.md` § 1.3 nachziehen; Review `k3c-welten` und `k3c-reviewer` vor dem Commit.

## Fertig, wenn

- [ ] AC-04: B-983/AC-01, AC-02 als Tests grün.
- [ ] AC-04: B-983/AC-04 Regel nachgezogen.
- [ ] AC-08: `testdata/golden/` unverändert (B-983/AC-03).

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

Wird am Ende der Session ausgefüllt: Nachweis je Kriterium (`AC-01 geprüft: task check grün`,
`AC-02 verschoben: Grund, B-0NN`), wer manuell geprüft hat, Abweichungen vom Plan, neue Tickets. Bis dahin `–`.
