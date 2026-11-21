# LA1.3 · Der Kompass im Hub zeigt die Biome der Insel

- **Status:** fertig
- **Typ:** Umsetzung
- **Agent:** autonom
- **Domäne:** CLI
- **Lane:** zweit
- **Modell:** sonnet
- **Umgebung:** offline
- **Branch:** sprint/la1
- **Abhängig von:** –
- **Tickets:** B-1167
- **Kriterien:** AC-03

## Ziel

Der Kompass im Hub zeigt die Biome der Insel an.

## Kontext

Ticket B-1167 (Ralf 2026-11-20). Offen im Ticket: Lage des Kompasses im Hub, Anzeige als Papierschnitt-Icon oder Text, Biome der Insel oder je Richtung. Die Session entscheidet nach Go-Verhalten (falls der Kompass dort existiert) und `docs/rules/`; bleibt etwas offen, mit Default entscheiden und im Ergebnis melden. Sichtprüfung 🧑 am PC.

## Erlaubte Dateien

- `rust/crates/k3c-game/src/hud/**`, `props/**`, `world/**` (Hub, Kompass)
- `data/hub.json`, `data/paper.json`

## Nicht-Ziele

Neue Biome, Regeln, Seekarte.

## Schritte

1. Kompass im Hub finden, Datenquelle der Biome der Insel klären.
2. Anzeige umsetzen mit Test.
3. `cargo test -p k3c-game`, `task rust:lint`, einmal `task check:diff`.

## Fertig, wenn

- [ ] AC-03: Kompass zeigt die Biome der Insel (Test).
- [ ] Nachprüfung 🧑: Anzeige im Spiel.
- [ ] Tests und `task rust:lint` grün.

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

### Umgesetzt
- Der „Kompass“ ist das Radar der Zelle (`hud/radar.rs`, `drawworld.rs`). Im **Hub** (Stufe Tiefe 0) steht jetzt über der Leiste eine Zeile mit den Biomen der Insel in Reihenfolge der Stufen, z. B. „Oberwelt (Wald) · Tiefe 1 (Höhle)“ (Namen aus `texts_de/en`, Rückfall `biome.name`).
- `island_biomes` und `biomes_key` (`radar.rs`), `SeatState.biomes_s` (`state.rs`, nur bei Änderung neu gebaut, ohne Allokation je Bild), neuer `TextSlot::BIOMES` (`paint.rs`, `MENU_ROW` rückt auf 18), `draw_radar` nimmt die Schrift (`draw.rs`, `drawworld.rs`).
- Eine einzelne Stufe zeigt keine Zeile; unter der Oberfläche bleibt sie leer.

### Kriterien
- **AC-03** erfüllt: Test `kompass_zeigt_die_biome_der_insel` (`hud/radar_tests.rs`, deutsch und englisch, Hub gegen Tiefe).
- `cargo test -p k3c-game` grün, `task rust:lint` grün, `cargo fmt --check` grün. `task check:diff` bleibt in Stufe `regeln` rot an den bekannten Planungstests (RU30, Fahrplan, `planningProjects`), nicht durch diese Session.

### Entscheidungen (Default, für Ralf)
- Textzeile statt Biom-Symbole auf der Leiste; nur im Hub; Reihenfolge der Stufen, nicht nach Tiefe sortiert.

### Nachprüfung (Ralf, PC)
- Hub im Spiel ansehen: Zeile über dem Kompass lesbar, kollidiert nicht mit Geldsack und Vorrat, auch im Split-Screen.

### Für den Hauptrechner
- Nichts.
