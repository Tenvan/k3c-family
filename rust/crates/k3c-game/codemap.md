# rust/crates/k3c-game/

## Responsibility

Das native Spiel K3C am PC (Port aus Go, Projekt RSU; Leitfaden `docs/standards/rust-port.md`): Client, Schleife und alle
Darstellung. Das Crate zeichnet nur über `k3c-platform` und rechnet keine Regeln; Regeln, Zustand und Takt liegen in
`k3c-sim`, `k3c-room`, `k3c-level`. Ein Lib-Crate (`src/lib.rs`) plus ein Binary (`src/main.rs`, EXE `k3c-game`).

## Design

- `Cargo.toml`: Abhängigkeiten `k3c-rng`, `k3c-det`, `k3c-data`, `k3c-level`, `k3c-sim`, `k3c-room`, `k3c-balance`,
  `k3c-rig`, `k3c-platform`, `k3c-devproto`, dazu `png`, `flate2` (`=1.1.11`, `.jsonl.gz` für Replays), `serde`,
  `serde_json`. Dev-Abhängigkeiten: `k3c-devwin` (nur Windows), `k3c-alloc-probe`, `k3c-data` mit Feature `go-default`
  (Tests laufen im Look-Profil `go`). Lints aus dem Workspace.
- Module (alle `pub mod` in `src/lib.rs`):

| Modul | Rolle |
|---|---|
| `actors/` | Figuren (Monarch, Truppen, Gegner, Bosse) als Rig oder Sheet zeichnen, Pose und Animation |
| `art/` | Grafik-Bibliothek: Papier-Sheets je Level, Rigs, Schrift, Texturbudget, Daten aus `data/paper.json` |
| `audio/` | Ton: Planer (Mix, Musik, Ducking) und Ausgabe über `k3c_platform::AudioOut` |
| `demo/` | Demo-Client (`k3c-game demo`): Papierschnitt-Welt nach Skript, ohne Sim |
| `devtools/` | Entwickler-Anzeigen: Tastenübersicht F1, Cheat-Menü F9, Debug-Overlay F10, Chronik, Bericht |
| `devwin/` | Spielseite des Cheat-Fensters `k3c-devwin` (Pipe-Client, Befehle) |
| `fx/` | Effekte der Welt: Treffer, Münzen, Status, Warnkreise, Geschosse, Fest-Himmel |
| `game/`, `hud/`, `input/`, `local/`, `menu/`, `props/`, `replay/`, `still/`, `view/`, `world/` | eigene `codemap.md` je Ordner |

- Einstieg `main.rs`: Unterbefehle `demo`, `still`, `replay`, `licenses`, `play` (Standard, auch ohne Argument oder mit
  Schaltern); `K3C_VERSION` (Build-Zeit) geht an `menu::version::set_version`.

## Flow

`main` wählt den Unterbefehl; `play` ruft `game::workdir::anchor_to_package` (Ordner der EXE) und `game::run::play`.
Die Schleife baut je Frame ein `view::frame::Frame`; Listener (`actors`, `fx`, `audio::Sound`) lesen es einmal je Frame,
Drawer (`actors`, `fx`, `devtools`) zeichnen auf Ebenen (`view::layer::Layer`). `art::Library` liefert Sheets und Rigs an
alle Zeichner.

## Integration

- Abhängig von den Crates oben; Daten aus `data/` (`k3c_data::Source`: `K3C_DATA_DIR`, `data/` neben der EXE, sonst eingebettet).
- Konsumenten: das Paket `dist/k3c-game/` (`task build`); das Cheat-Fenster `k3c-devwin` spricht über `k3c-devproto` mit `devwin/`.
- Referenz für das Verhalten ist das Go-Spiel (Tag `go-referenz-final`); Doc-Kommentare nennen die Go-Vorlage je Datei.
