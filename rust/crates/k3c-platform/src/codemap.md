# rust/crates/k3c-platform/src/

## Responsibility

Der fensterunabhängige Kern: Takt, Eingabe-Zustand, Sprite-Sammlung, Schrift, Formen, Messung und Headless-Lauf.

## Design

- `lib.rs`: Reexporte, `LOGICAL`, `Rect`, `Filter`, `Blend`, `TexId`, Traits `App` und `Canvas`.
- Takt und Lauf: `clock.rs` (`Clock::advance` → Anzahl Takte und `alpha`; `STEP` = 30 Hz, `MAX_STEPS`), `runner.rs` (`Runner`: Uhr, Eingabe, Takte, Zeichnen; im Test mit simulierter Uhr), `headless.rs` (`run_headless`, `NullCanvas` zählt Zeichenaufrufe), `measure.rs` (`Measure`: Ringpuffer mit Bildabstand und CPU-Zeit, `Summary` als JSON).
- Eingabe: `input.rs` (`Key`, `PadButton`, `PadAxis`, `KeyState`, `PadState`, `InputState`, `InputScript` für Läufe ohne Fenster).
- Zeichnen: `batch.rs` (`Frame`: Rechteck-Instanzen und Dreiecke, geordnet nach Ebene, Ausschnitt und Aufruf; gleiche Textur, Art und Mischung bilden einen Draw-Call; `Inst`, `Vertex`, `Kind`, `RiverCall`, `MirrorCall`), `frame_canvas.rs` (`impl Canvas for Frame`), `affine.rs` (`Affine` wie `ebiten.GeoM`), `shape.rs` (`Tris`: Kreis, Ring, Polygon per Ohren-Schnitt, Puffer wiederverwendet).
- Text: `font.rs` (5×7-Pixel-Schrift), `ttf.rs` (`Fonts` über fontdue, Glyph-Atlas 2048 px, `AtlasUpdate`).
- Shader-Referenzen in Rust: `river.rs` (`RiverUniforms`, `river_fragment`), `mirror.rs` (`MirrorUniforms`, `mirror_fragment`); die WGSL-Fassungen liegen in `backend/`.
- `audio.rs`: Trait `AudioOut`, `SoundId`, `VoiceId`, `NullAudio`, `open_audio`. `log.rs`: Log nach stderr, jede Meldung beginnt mit einem Emoji.

## Flow

Eingabe-Schnappschuss + Uhr → `Runner` → Takte → `App::step`; danach `App::draw` in einen `Canvas`; `Frame` sammelt Instanzen und Dreiecke, das Backend zeichnet sie.

## Integration

- `backend/` konsumiert `Frame`, `InputState`, `AudioOut`, `Fonts`.
- Konsumenten außerhalb: `k3c-game`, `k3c-rig`.
