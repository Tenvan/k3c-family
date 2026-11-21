# HB1.3 · Alle Biome dicht bewachsen, Biome am Radar

- **Status:** offen
- **Typ:** Umsetzung
- **Agent:** autonom
- **Domäne:** SIM
- **Lane:** zweit
- **Modell:** opus
- **Umgebung:** offline
- **Branch:** sprint/hb1
- **Abhängig von:** HB1.2
- **Tickets:** B-1187
- **Kriterien:** AC-01, AC-02

## Ziel

Alle Biome starten dicht bewachsen (kleine Lichtungen und Lücken per Seed) und die Biome sind am Radar sichtbar.

## Kontext

Ticket B-1187. RU36.7 hat den Wald verdichtet: `data/biomes/forest.json` (chunkWeights), Test `rust/crates/k3c-level/src/forest_density_tests.rs`. Gleiches Prinzip für alle Biome in `data/biomes/*.json` (Baumdichte, Lichtungen einstellbar). Die Go-Engine ist eingefroren (Entscheidung 013): Goldens der Go-Engine (`task go:golden`, `testdata/golden`) müssen mit neu geschrieben werden, wenn `data/` sich ändert (Begründung in den Commit; Ralf hat das bei RU36.7 so entschieden). Radar: `rust/crates/k3c-game/src/hud/radar.rs` (`island_biomes`, LA1.3 zeigt Biome nur bei mehreren Stufen im Hub); neu immer am Radar.

## Erlaubte Dateien

- `data/biomes/**`
- `rust/crates/k3c-level/**`
- `rust/crates/k3c-game/src/hud/**`
- `engine/level/**`
- `testdata/**`

## Nicht-Ziele

Landstreicher-Lager (HB1.4), Hafen (HB1.5), Balancing.

## Schritte

1. Je Biom Dichte und Lichtungen im Daten-Wert, Test je Biom (Lichtungen ≤ 10 %, Baum-Mindestdichte).
2. Goldens und Datenbaum neu schreiben (`go test ./internal/detvec/ -write`, `task go:golden`).
3. Biome am Radar (Text im oder über der Leiste, auch ohne Hub und bei einer Stufe).
4. Tests.

## Fertig, wenn

- [ ] AC-01: Alle Biome dicht bewachsen, Lichtungen per Seed, Test je Biom.
- [ ] AC-02: Biome am Radar sichtbar.
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
