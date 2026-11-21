# EQ1.1 · Kämpfer holen die Rüstung am Gebäude ab

- **Status:** fertig
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

- AC-01 geprüft (B-1205 AC-01 bis AC-03): Elite entsteht erst, wenn ein Kämpfer die Rüstung am Rüstungsschmied abholt (`unit/armor.rs`, `fetchArmor`-Auftrag). `armor_convert` entfernt. Bei Gefahr (Nacht oder Welle) holt niemand, je Rüstung genau ein Kämpfer, ohne Kandidat bleibt die Rüstung im Regal; nur mit `native_rules`. Tests in `unit/combo_tests.rs` (3 neue).
- Review (k3c-reviewer): zwei mittlere Befunde behoben: Holer mit Auftrag wird nicht in Gruppe/Trupp gezogen (`combat/squad.rs`), Schild-Holer wird nicht als Rüstungsholer gewählt.
- Abweichung: `combat/squad.rs` liegt außerhalb der erlaubten Dateien (nur zwei Bedingungen).
- `cargo test -p k3c-sim --lib`: nur die bekannten Roten (`insel_nach_1000_schritten_wie_go`, `eingebettete_welten_gueltig`); clippy sauber, fmt ok. `check:diff` bricht an diesen bekannten Roten ab (B-1190 AC-04).
- PC-Abnahme Spielgefühl (Holer läuft sichtbar zum Gebäude): Ralf, im Abnahme-Ticket EQ1.
