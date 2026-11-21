# LA3.1 · Bauplatz-Abstände nach Gebäudebreite (B-1182)

- **Status:** offen
- **Typ:** Umsetzung
- **Agent:** autonom
- **Domäne:** SIM
- **Lane:** hauptrechner
- **Modell:** opus
- **Umgebung:** offline
- **Branch:** sprint/la3
- **Abhängig von:** –
- **Tickets:** B-1182
- **Kriterien:** AC-01

## Ziel

Feste Bauplätze richten ihren Abstand nach der Größe der Gebäude, sodass Gebäude sich nicht mehr überlappen.

## Kontext

Bauplätze liegen in `rust/crates/k3c-game/src/props/plan_sites.rs` und `props/plan.rs` (Reihenfolge: decor, back_walls, sites_pass(true), castle, entities, nodes, sites_pass(false)); Plätze kommen aus der Simulation/Daten (`k3c-sim`, `data/`). Gebäudebreiten aus den Papierschnitt-Maßen. Ticket B-1182. Berührt Spielstand-Layout und Golden: Sperre `golden|spielstand`.

## Erlaubte Dateien

- `rust/crates/k3c-sim/src/**`
- `rust/crates/k3c-game/src/props/**`
- `data/buildings.json`

## Nicht-Ziele

Keine Balance-Änderung, keine Simulationsläufe.

## Schritte

1. Gebäudebreiten je Typ bestimmen.
2. Abstandsregel für Bauplätze: Mitte zu Mitte mindestens halbe Breite beider Nachbarn plus Rand.
3. Alte Spielstände: keine Migration (🧑 2026-11-21: egal).
4. Tests, Golden bewusst neu schreiben mit Begründung.

## Fertig, wenn

- [ ] AC-01: Kein Paar benachbarter Bauplätze überlappt in Breite; Test über alle Welten grün.
- [ ] `task check:diff` grün.

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
