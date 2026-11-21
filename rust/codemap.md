# rust/

## Responsibility
Cargo-Workspace des nativen Rust-Spiels (Projekt RSU, Port aus Go; Leitfaden `docs/standards/rust-port.md`). Er enthält das Spiel `k3c-game`, das Cheat-Fenster `k3c-devwin`, die Simulation samt Hilfs-Crates und das Paket-Beiwerk. Der Namensraum `rust` in `Taskfile.yml` baut, prüft, startet und paketiert das Spiel. Die Crates unter `crates/` sind hier nur namentlich aufgeführt.

## Design
- `Cargo.toml`: `members = ["crates/*"]`, Resolver 2, Edition 2024, `publish = false`; fremde Crates nur mit festen Versionen unter `[workspace.dependencies]` (`winit`, `wgpu`, `pollster`, `png`, `serde`, `serde_json`, `fontdue`, `gilrs`, `rodio`, `windows` mit Win32-Features nur für das Cheat-Fenster). Lints: `unsafe_code = "forbid"`, Clippy verbietet `unwrap`, `expect`, `panic`, `todo`, `unimplemented` und zu lange Funktionen (`too_many_lines`). Dazu `rust-toolchain.toml` (feste Toolchain mit rustfmt und clippy), `clippy.toml`, `rustfmt.toml`.
- `crates/` (nach den Namen): `k3c-game` (Spiel), `k3c-platform` (Fenster, Zeichnen, Schleife über winit/wgpu), `k3c-rig` (Rig aus Teilen), `k3c-sim` (Simulation), `k3c-room` (Raum mit Stufen und Spielstand), `k3c-level` (Level-Generator), `k3c-balance` (Balancing und Testläufe), `k3c-data` (typisierte `data/*.json`), `k3c-rng` und `k3c-det` (deterministischer Zufall und bitgleiche Gleitkomma-Hilfen), `k3c-devwin` (Cheat-Fenster, F4 im Spiel) mit `k3c-devproto` (Protokoll Spiel ↔ Fenster, Named Pipe), `k3c-alloc-probe` (Allokationszähler für Tests).
- Weitere Ordner: `paket/README.txt` (Beilage des Spielpakets), `demo/reiten.json`, `target/` (Build-Ausgabe).
- `Taskfile.yml` (Namensraum `rust`, Variablen `PAKET` = `dist/k3c-game`, `VERSION` aus `git describe` über `v*`-Tags):
  - Workspace: `build`, `test` (Filter über `CLI_ARGS`), `lint` (clippy `-D warnings`), `fmt`, `fmt:check`.
  - `run`: Spiel im Release aus der Repo-Wurzel starten, vorher `gen:paper`.
  - `game:exe` (Release-EXE, `K3C_VERSION` für `option_env!`), `devwin:exe` (Cheat-Fenster-EXE), `game:build` (Abhängigkeiten `game:exe`, `devwin:exe`, `gen:paper`, `tmp`; dann `game:dist`), `game:dist` (intern: `tools/sync` kopiert EXE, `data/`, `public/grafik`, `public/rig`, `public/audio`, Lizenzen, `README.txt`, optional `public/schrift` ins Paket; Go-Quellen und Doku aus `data/` entfernt; Cheat-Fenster daneben; `rustpaket lizenzen` schreibt `LIZENZEN.txt`; `rustpaket pruefen -lokal` prüft den Ordner), `game:dist:check` (strenge Paketprüfung ohne Neubau), `game:run`, `game:dev` (Debug-Bau, Start aus der Repo-Wurzel), `game:measure` (`tools/measure/measure.ps1 -Engine rustgame`).

## Flow
1. Entwicklung: `task rust:build|test|lint|fmt:check` arbeiten im Ordner `rust/`; `task rust:game:dev` baut Debug und startet aus der Repo-Wurzel.
2. Paket: `task build` bzw. `rust:game:build` → Release-EXEs → `tools/sync` → `rustpaket lizenzen` → `rustpaket pruefen` → `dist/k3c-game/`.
3. Vor Commits wählt `tools/checkdiff` je geänderter Crate `cargo fmt --check`, `clippy` und `cargo test`.

## Integration
- Wurzel-`Taskfile.yml` bindet das Taskfile als Namensraum `rust` ein; `task build` entspricht `rust:game:build`.
- Werkzeuge: [`tools/sync`](../tools/sync/codemap.md), [`tools/rustpaket`](../tools/rustpaket/codemap.md), [`tools/checkdiff`](../tools/checkdiff/codemap.md) (Bahn `rust`), [`tools/portcheck`](../tools/portcheck/codemap.md) (Testabgleich gegen den Go-Stand).
- Daten und Grafik kommen von außen: `data/` (Balancing-JSON), `public/grafik` (per `gen:paper`), `public/rig`, `public/audio`, `public/schrift`.

## Crate-Karten

Überblick und Abhängigkeitsrichtung: [`crates/codemap.md`](crates/codemap.md). Je Crate: [`k3c-alloc-probe`](crates/k3c-alloc-probe/codemap.md), [`k3c-balance`](crates/k3c-balance/codemap.md), [`k3c-data`](crates/k3c-data/codemap.md), [`k3c-det`](crates/k3c-det/codemap.md), [`k3c-devproto`](crates/k3c-devproto/codemap.md), [`k3c-devwin`](crates/k3c-devwin/codemap.md), [`k3c-level`](crates/k3c-level/codemap.md), [`k3c-platform`](crates/k3c-platform/codemap.md), [`k3c-rig`](crates/k3c-rig/codemap.md), [`k3c-rng`](crates/k3c-rng/codemap.md), [`k3c-room`](crates/k3c-room/codemap.md).
