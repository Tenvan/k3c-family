# rust/crates/k3c-platform/src/backend/

## Responsibility

Die einzige Stelle im Crate mit winit, wgpu, gilrs und rodio: Fenster und Ereignisschleife, GPU-Zeichnen eines `Frame`, Gamepads und Tongerät.

## Design

- `mod.rs`: `Config`, `run_window(app, cfg)` (winit-Fenster, Schleife, Messring).
- `gpu.rs`: `Gfx` (Gerät, Surface mit Fifo, Texturen, `render` zeichnet Instanzen und Dreiecke im Wechsel nach Draw-Call-Reihenfolge); `setup.rs`: Bind-Layout, Pipelines (Rechteck-Instanzen, Dreiecke, Fluss), Puffer (`Grow`), Textur-Upload.
- Spiegel (RU55.3): `gpu_mirror.rs` (Uniforms, Bind-Group, Kopie des Streifens), `mirror_setup.rs` (Layout, Pipeline, Zwischentextur).
- Shader: `shader.wgsl`, `river.wgsl`, `mirror.wgsl`; die Referenzrechnung steht in `river.rs` und `mirror.rs` des Elternordners.
- `keys.rs`: `map` (winit-`KeyCode` → `Key`, physische Taste); `pads.rs`: `Pads` über gilrs (`poll` füllt `InputState`, `PadEv`); `sound.rs`: `decode` in `Pcm`, `Sound` mit je einem Player je Stimme, `open`.

## Flow

winit-Ereignisse und `Pads::poll` → `InputState` → `Runner` → `App` → `Frame` → `Gfx::render` → `present`.

## Integration

- Genutzt über `run_window` und `open_audio` aus `lib.rs`; Konsument ist `k3c-game`.
- Abhängigkeiten: `winit`, `wgpu`, `pollster`, `gilrs`, `rodio`.
