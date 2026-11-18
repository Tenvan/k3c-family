# B-1071 · Fluss-Fragment als Referenz in Rust und Software-Nachbau im Standbild-Rasterer

- **Domäne:** CLI
- **Typ:** Schuld
- **Prio:** niedrig
- **Umgebung:** offline
- **Status:** offen
- **Sprint:** RU13
- **Projekt:** RSU
- **Erstellt:** 2026-11-18
- **Spec:** freigegeben
- **Revision:** 1
- **Freigabe:** autonom, Claude, 2026-11-18
- **Duplikat von:** –
- **Ersetzt:** –

## Ausgangslage

`water/river.kage` (Go) ist die Quelle des Flusses: sechs Wellen-Reihen, Verzerrung, Papier-Schatten, Glanzstriche. Der Standbild-Rasterer `SoftCanvas` (`rust/crates/k3c-game/src/still/raster.rs`) rechnet die GPU-Befehle für den Bildvergleich nach und kennt den Fluss nicht (`tests/still.rs` zeichnet Kacheln). Teil 2 von 4 aus B-1055.

## Ziel

Das Fragment des Shaders steht einmal als reine Rust-Funktion (`river_fragment`) in `k3c-platform`; `SoftCanvas` implementiert `Canvas::river` damit. So gibt es eine Referenz in Rust, gegen die B-1072 (WGSL) und der Bildvergleich prüfen.

## Beteiligte und Zielgruppen

Wer spielt, entwickelt, betreibt oder entscheidet (🧑)? Keine Verantwortlichen erfinden.

## Anforderungen

- `river_fragment(u: &RiverUniforms, p: [f32; 2]) -> [f32; 4]` nach `river.kage`: Reihenwahl, `v`, `u`, Verzerrung, Band mit Schatten, Mischung, Glanz; Alpha 0 oberhalb der ersten Reihe. Konstanten `tau`, `srcH = 64` wie im Original.
- `SoftCanvas::river` rastert das Rechteck mit Pixelmitte als `p` (wie `draw_quad`), beachtet Clip und die Blend-Stufe und liefert `true`.
- Deterministisch; `f32` wie in der GPU-Fassung, kein `f64` in der Pixelrechnung.

## Nicht-Ziele

WGSL (B-1072), Schnittstelle (B-1070, Voraussetzung), Anbindung in `world` und der Vergleich Go gegen Rust im Standbild (B-1073).

## Regeln und Einschränkungen

`docs/standards/rust-port.md`, Datei höchstens 400 Zeilen. Kein Go-Lauf, kein `gen:paper`.

Planungsangaben für die Arbeitswellen:

- **Bereich:** `rust/crates/k3c-platform/src/river.rs`, `rust/crates/k3c-platform/src/river_tests.rs`, `rust/crates/k3c-platform/src/lib.rs`, `rust/crates/k3c-game/src/still/raster.rs`, `rust/crates/k3c-game/src/still/raster_tests.rs`
- **Abhängig von:** B-1070
- **Braucht 🧑:** –
- **Größe:** S
- **Sperre:** –

## Beispiele

Punkt oberhalb der ersten Reihe: Alpha 0. Punkt in der Reihe 3 auf der Bandlinie: Farbe `wave`. Punkt 4 Quell-px unter der Bandlinie: `shade`.

## Ausnahme- und Fehlerfälle

Reihenhöhe 0: `v` wird auf 0 geklemmt, keine Division durch 0 im Ergebnis (NaN). Rechteck teils außerhalb des Bildes: nur sichtbare Pixel werden geschrieben.

## Akzeptanzkriterien

- **AC-01** `river_fragment` liefert für feste Uniforms und mindestens 8 Punkte (je Zweig: oberhalb, Grund, Band, Schatten, Glanz an und aus, erste und letzte Reihe) die erwarteten Farben; Erwartungswerte von Hand aus `river.kage` gerechnet (`river_tests.rs`).
- **AC-02** `SoftCanvas::river` färbt ein Rechteck mit denselben Werten wie `river_fragment` in der Pixelmitte und respektiert den Clip (Test in `raster_tests.rs`).
- **AC-03** `cargo test -p k3c-platform -p k3c-game` und `task rust:lint` sind grün.

## Offene Fragen

keine

## Notizen

Aus B-1055 zerlegt. Die Referenz-Funktion dient B-1072 als Vergleichswert (Testwerte teilen).
