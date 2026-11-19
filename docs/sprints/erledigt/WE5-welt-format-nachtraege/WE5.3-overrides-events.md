# WE5.3 · Overrides für events im Wertesatz der Welt (B-982)

- **Status:** fertig
- **Typ:** Umsetzung
- **Agent:** autonom
- **Domäne:** SIM
- **Lane:** –
- **Modell:** sonnet
- **Umgebung:** offline
- **Branch:** sprint/we5
- **Abhängig von:** WE5.2
- **Tickets:** B-982
- **Kriterien:** AC-03, AC-08

## Ziel

`events` ist eine erlaubte Override-Datei; der Raum rechnet mit den Events des Wertesatzes der Welt (B-982).

## Kontext

Eigener Worktree mit Dev-Workbench (B-925, Agent `k3c-sim-dev`). Wertesatz der Welt: `engine/sim/world_values.go` (Tabelle der erlaubten Dateien mit Typ), `world_values_merge.go` (Merge-Regeln, Listen je `id`), `world_values_locked.go` (gesperrte Felder), `world_values_room.go` (Raum nutzt den Satz). Events: `data/events.json`, Laden und Logik in `engine/sim/events.go`, `events_moon.go`, `events_merchant.go`; heute aus der eingebetteten Basis. Regeln: `docs/rules/welten.md` § 1.1, § 1.2, `bosse.md` § 2. Golden dürfen sich nicht ändern.

## Erlaubte Dateien

- `engine/sim/**`
- `engine/room/**`
- `docs/rules/welten.md`
- `docs/rules/bosse.md`

## Nicht-Ziele

Neue Events (WE8), Workbench (WE6.3), `data/events.json` ändern, Simulation.

## Schritte

1. Tests zuerst: Welt mit Override lädt und ändert den Abstand; Abstand 0 = aus; Fehlerfälle.
2. `events` in die Tabelle der Override-Dateien aufnehmen, Prüfung (Abstände ≥ 0, Faktoren > 0, bekannte Gegnerarten).
3. Event-Logik liest aus dem Wertesatz des Raums statt aus der Basis.
4. Regeln nachziehen; Review `k3c-reviewer` vor dem Commit.

## Fertig, wenn

- [ ] AC-03: B-982/AC-01, AC-02 als Tests grün.
- [ ] AC-03: B-982/AC-04 Regeln nachgezogen.
- [ ] AC-08: `testdata/golden/` unverändert (B-982/AC-03).

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

AC-03 geprüft: `TestEventsOverrideAendertAbstand`, `TestEventsAbstandNullIstAus`, `TestEventsKindErbtOverride`, `TestEventsOverrideAbgelehnt`; Regeln `welten.md`, `bosse.md` § 2 nachgezogen. AC-08 geprüft: Golden unverändert. `go vet`, `go test ./engine/...` (bis auf bekanntes `TestHTMLEigenstaendig`), Lint, `task check:diff` grün. Commit 8796fb90a auf `sprint/we5`. Abweichung: Abstand 0 ist jetzt gültig (aus).
