# rust/crates/k3c-platform/

## Responsibility

Plattform-Kern des Spiels (RU2.1): Fenster, Zeichnen, Eingabe, Ton und feste Schleife. Nur `backend/` kennt winit, wgpu, gilrs und rodio; alles andere ist reiner Rust-Code ohne Grafikabhängigkeit und ohne Fenster testbar. `#![forbid(unsafe_code)]`.

## Design

- `Cargo.toml`: `winit`, `wgpu`, `pollster`, `fontdue`, `gilrs`, `rodio`, `serde`, `serde_json`.
- Quellcode in `src/` (siehe [`src/codemap.md`](src/codemap.md)), Fenster und GPU in `src/backend/` ([`codemap.md`](src/backend/codemap.md)).
- Schnittstellen: Trait `App` (`step`, `draw`, `done`, `fullscreen`) und Trait `Canvas` (Sprites, Meshes, Formen, Text, `river`, `mirror`); logische Auflösung `LOGICAL` = 1920 × 1080.

## Flow

`run_window(app, Config)` oder `run_headless(app, steps, script)` → `Runner` → `Clock` liefert feste Takte → `App::step` mit `InputState` → `App::draw` auf einem `Canvas` (im Fenster `Frame` → `Gfx`, headless `NullCanvas`).

## Integration

- Konsumenten: `k3c-game` (implementiert `App`), `k3c-rig` (`Canvas`, `Affine`, `Vertex`).
- Keine internen Abhängigkeiten.
