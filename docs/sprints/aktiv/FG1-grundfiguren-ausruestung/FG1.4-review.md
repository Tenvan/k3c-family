# FG1.4 · Review und Abnahme-Ticket FG1

- **Status:** fertig
- **Typ:** Review
- **Agent:** autonom
- **Domäne:** CLI
- **Lane:** zweit
- **Modell:** sonnet
- **Umgebung:** offline
- **Branch:** fg1/4-review
- **Abhängig von:** FG1.2, FG1.3
- **Tickets:** B-1199, B-1200, B-1201
- **Kriterien:** AC-01

## Ziel

FG1 ist geprüft; ein Abnahme-Ticket sammelt PC-Abnahme, Gen-, Sim- und Race-Läufe.

## Kontext

Review der Sessions FG1.1 bis FG1.3 gegen B-1199 bis B-1201. Sim, Balance, Gen und `check:race` nur im Abnahme-Ticket (B-742).

## Erlaubte Dateien

- `docs/sprints/aktiv/FG1-*/**`

## Nicht-Ziele

Was in dieser Session ausdrücklich **nicht** passiert.

## Schritte

1. Diff lesen, Fachagenten (k3c-welten, k3c-grafik, k3c-balancing).
2. `task check` und `task check:go` nacheinander.
3. Abnahme-Ticket anlegen.

## Fertig, wenn

- [ ] AC-01: Review ohne offene schwere Befunde, Abnahme-Ticket angelegt.
- [ ] `task check` und `task check:go` grün.

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

AC-04 geprüft: Review FG1.1 bis FG1.3 durch k3c-reviewer; `task check` und `task check:go` grün. Befund 1 (mittel, behoben): `dev/spawn.rs` legte Bauern und Landstreicher mit der Roh-Art an, `step_troops` ließ sie als Bogenschützen laufen (Dev-Cheats, Lastmessung `LOAD_TROOPS`); jetzt `serve()` bzw. `go_wild()`, Test `spawn_macht_buerger_aus_bauer_und_landstreicher`. Befund 2 (mittel, offen): `armor_convert` ohne Reichweite, Rüstung im Regal macht sofort jeden Kämpfer der Stufe zur Elite mit voller Heilung. Befund 3 (mittel, offen): Rüstungspreis, Regalgröße und Herstellzeiten als Code-Konstanten (`unit/hammer.rs`, `unit/warrior.rs`). Beide offenen Befunde stehen als Entscheidung bzw. AC-02 im Abnahme-Ticket B-1203.

Abweichung: Der Fix lag außerhalb der erlaubten Dateien (`k3c-sim/src/dev`). Bekannt rot: `alloc_je_bild` (B-1190 AC-04).

### Für den Hauptrechner

- Befund 2 und 3 entscheidet Ralf (B-1203); danach Ticket für den Umbau anlegen.
