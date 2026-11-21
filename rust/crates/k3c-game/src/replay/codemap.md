# rust/crates/k3c-game/src/replay/

## Responsibility

Wiedergabe einer Go-Aufzeichnung (`.jsonl.gz`, RU38): Unterbefehl `k3c-game replay <datei>`, ohne Fenster als Prüfwert oder im Fenster mit Messung, gezeichnet mit den Zeichnern des Spiels.

## Design

- `cli.rs`: `parse_args`, `Options`, `Mode` (`--headless`, `--window`, `--measure`, `--out`, `--measure-frames`, `--size`, `--novsync`).
- `trace.rs`: Leser (`Trace`, `Head`, `Row`, `TraceError`): Kopfzeile mit Level, danach je Takt Zustand und Ereignisse.
- `player.rs`: `Playback`: ein Takt je 1/30 s, Takt 0 ist der Startzustand.
- Bild: `app.rs` (`ReplayApp` als `App` der Plattform: Frame je Takt, zwei Zellen übereinander), `scene.rs` (`Scene`: Zeichner world, props, actors, fx, hud in der Anmeldereihenfolge des Spiels), `layout.rs` (Zellen, `Cameras::follow`), `blend.rs` (Zwischenbilder), `neutral.rs` (`NeutralSimView` ohne Absturz).
- Läufe und Messung: `run.rs` (`run_headless` mit FNV-Prüfwert, `run`), `report.rs` (`Metered`, derselbe `Report` wie in `game::measure`), `frames.rs`.

## Flow

`cli` parst, `trace::load` entpackt und liest die Aufzeichnung, `Playback::advance` schaltet je Takt den Sim-Zustand fort, `ReplayApp` baut Frame, Kameras und Listener und zeichnet über `Scene`; headless entsteht ein Prüfwert, im Fenster ein Messbericht (JSON).

## Integration

- Abhängigkeiten: `k3c_sim`, `k3c_level`, `k3c_data`, `k3c_platform`, `view`, `art`, `actors`, `props`, `game` (Messbericht).
- Konsumenten: Unterbefehl `replay` des Programms.
