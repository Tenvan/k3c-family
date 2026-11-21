# rust/crates/k3c-game/src/demo/

## Responsibility

Demo-Client (`k3c-game demo`, RU2.3): zeichnet eine Papierschnitt-Welt, durch die die Kamera reitet; Figuren laufen nach
einem JSON-Skript, nicht nach der Sim. Dient Messung und Sichtprüfung der Grafik; zeichnet nur über `k3c-platform`.

## Design

- `cli.rs`: `Options`, `parse_args` (`--driver`, `--sprites N`, `--measure S`, `--headless --steps N [--script S]`, `--out`, `--fullscreen`, `--grafik`, `--data`).
- `driver.rs`: `Driver` aus JSON (`Ebene`, `Gebaeude`, `Figur`, `Kamera`), `read`, `assets`. `assets.rs`: `paper_assets` sammelt alle Zuordnungen aus `data/paper.json`.
- `world.rs`: `World` je Takt (30 Hz, `TICKS`): Kamera, Reiter und Figuren laufen hin und her. `scene.rs`: Frame wählen, Sprite mit Fußpunkt und Spiegelung setzen (`sprite_at`).
- `mod.rs`: `Demo::new`, `Demo::run`, `run(&Options)`.

## Flow

`parse_args`, Driver lesen, Assets über `art` laden, je Takt `World::step`, dann zeichnet `scene` Ebenen, Gebäude und
Figuren. Mit `--measure` Messlauf, mit `--headless --steps N` ohne Fenster.

## Integration

- Abhängigkeiten: `art` (Library, Sheets), `k3c-platform`, `k3c-data`; Daten `data/paper.json`.
- Konsument: `main.rs` (Unterbefehl `demo`).
