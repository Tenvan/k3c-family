# WD1.1 · Level: volle Baumdichte, Hub-Streifen und Lichtungen

- **Status:** fertig
- **Typ:** Umsetzung
- **Agent:** autonom
- **Domäne:** SIM
- **Lane:** erst
- **Modell:** opus
- **Umgebung:** offline
- **Branch:** wd1/1-level-dichte-lichtungen
- **Abhängig von:** –
- **Tickets:** B-1204
- **Kriterien:** AC-01

## Ziel

Auf der Oberfläche hat jeder Chunk-Typ die volle Baumdichte; Hub-Streifen und Ereignis-Lichtungen bleiben baumfrei (native Regel, Go-Golden unverändert).

## Kontext

B-1204 AC-01/AC-02, Annahmen im Ticket. Generator `rust/crates/k3c-level/src/level_entities.rs`, Biom `data/biomes/forest.json` (mit Go geteilt, darum Rust-only: `data/sizes.json`-Muster oder Flag der Level-Optionen). Hub-Layout `data/hub.json` und `hub_layout`.

## Erlaubte Dateien

- `rust/crates/k3c-level/**`
- `rust/crates/k3c-data/**`
- `rust/crates/k3c-sim/src/world/**`
- `data/sizes.json`
- `docs/rules/**`

## Nicht-Ziele

Was in dieser Session ausdrücklich **nicht** passiert.

## Schritte

1. Level-Erzeugung lesen (Chunk-Einträge, `on_free_ground`, Ereignis-Chunks).
2. Rust-only-Schalter (native Regeln) und Werte in `data/sizes.json` (Lichtungsradius, Hub-Abstand).
3. Volle Dichte für alle Oberflächen-Chunk-Typen; Hub-Streifen bis kurz vor dem ersten Mauer-Bauplatz; Lichtung je Truhe, Camp, Hafen, Portal, Ausgang.
4. Tests über 40 Seeds, Go-Golden unverändert.

## Fertig, wenn

- [ ] AC-01: Dichte, Hub-Streifen und Lichtungen laut B-1204/AC-01, AC-02 (Tests über 40 Seeds).
- [ ] `task check:diff` grün (außer vorher roten Tests).

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

AC-01 geprüft: `world/forest.rs` (`grow_forest`, aus `place_entities`, nur mit nativen Regeln auf Tiefe 0) füllt jeden Nicht-Hub-Chunk auf die Dichte des Chunk-Typs `forest` (anteilig, wenn Hub-Streifen oder Lichtung ihn teilweise freihält), überspringt den Hafen-Chunk (`plant_harbor_trees`, Gap jetzt gleich der Lichtung) und hält Hub-Streifen (bis 4 Units vor dem ersten Mauer-Bauplatz) und Lichtungen (Radius 6) um Truhe, Lager, Portal, Ausgang, Hafen frei. Würfel aus eigenem Strom `<seed>:wald`. Umsetzung in der Sim statt im Level-Generator (wie ECO1.8), damit Level-Seeds und Go-Golden unverändert bleiben. Tests `forest_tests` (40 Seeds: Dichte, Hub-Streifen, Lichtungen, Go-Parität), `cargo test -p k3c-sim` grün bis auf den schon roten Insel-Takt-Test, clippy sauber. Regeltext in materialien-gebaeude.md § 3.3.

Annahme/Abweichung: Lager-Wächter-Bäume (HB1.4) stehen in der Lager-Lichtung. Die Workbench-Level-Vorschau rechnet mit der Go-Engine und zeigt den nativen Wald nicht (WD1.3 klärt die Anzeige). Schon rot: `echte_grafik_plant_alle_assets` (fehlende gen:paper-Grafik aus FG1), `alloc_je_bild`, `e2e_flow`, `still_hud`.
