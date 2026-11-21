# EQ1.1 · Kämpfer holen die Rüstung am Gebäude ab

- **Status:** offen
- **Typ:** Umsetzung
- **Agent:** autonom
- **Domäne:** SIM
- **Lane:** zweit
- **Modell:** opus
- **Umgebung:** ?
- **Branch:** eq1/1-ruestung-abholen
- **Abhängig von:** –
- **Tickets:** B-1205
- **Kriterien:** AC-01

## Ziel

Der Kämpfer holt die Rüstung am Rüstungsschmied ab und wird dabei Elite; keine Fernwirkung mehr.

## Kontext

Ticket B-1205. `armor_convert` in `rust/crates/k3c-sim/src/building/upgrades.rs` wandelt heute aus der Ferne um (FG1.2). Vorbild: Bogen und Schwert holen (`weapon_to_fetch`, `unit/warrior.rs`). Nur mit nativen Regeln.

## Erlaubte Dateien

- `rust/crates/k3c-sim/src/building/**`
- `rust/crates/k3c-sim/src/unit/**`
- `rust/crates/k3c-sim/src/state/**`

## Nicht-Ziele

Was in dieser Session ausdrücklich **nicht** passiert.

## Schritte

1. `armor_convert` und Regal lesen.
2. Auftrag "Rüstung holen" für freie Kämpfer, nur ohne Gefahr, je Rüstung einer.
3. Elite beim Abholen, ohne Kandidat bleibt die Rüstung im Regal.
4. Tests AC-01 bis AC-03 des Tickets.

## Fertig, wenn

- [ ] AC-01: Elite entsteht erst nach dem Abholen am Gebäude, nur ohne Gefahr, je Rüstung ein Kämpfer (B-1205 AC-01 bis AC-03).
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
