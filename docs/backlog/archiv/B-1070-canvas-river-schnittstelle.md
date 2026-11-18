# B-1070 · Canvas kennt einen Fluss-Aufruf mit Uniforms (Schnittstelle ohne Shader)

- **Domäne:** CLI
- **Typ:** Schuld
- **Prio:** niedrig
- **Umgebung:** ?
- **Status:** erledigt
- **Sprint:** RU13
- **Projekt:** RSU
- **Erstellt:** 2026-11-18
- **Spec:** freigegeben
- **Revision:** 1
- **Freigabe:** autonom, Claude, 2026-11-18
- **Duplikat von:** –
- **Ersetzt:** –

## Ausgangslage

`k3c_platform::Canvas` (`rust/crates/k3c-platform/src/lib.rs`) kennt Sprites, Meshes, Formen und Text, aber keinen Shader-Aufruf. Go zeichnet den Fluss mit `DrawRectShader` und `water/river.kage` (`native/k3c-game/world/river.go`, `drawRiver`). Rust rechnet die Uniforms schon (`world/water.rs`: `water_layout`, `RiverWater::set`), hat aber keine Stelle, sie abzugeben. Teil 1 von 4 aus B-1055.

## Ziel

Der Canvas bekommt einen festen Aufruf `river`, der ein Rechteck mit den Uniforms des Fluss-Shaders zeichnet. Dieses Ticket legt nur die Schnittstelle an: Typ, Trait-Methode mit Rückfall (`false` = nicht unterstützt), Aufzeichnung im `Frame`. Kein WGSL, keine Pixelrechnung.

## Beteiligte und Zielgruppen

Wer spielt, entwickelt, betreibt oder entscheidet (🧑)? Keine Verantwortlichen erfinden.

## Anforderungen

- Neuer Typ `RiverUniforms { origin: [f32; 2], rows: [f32; 24], phase: [f32; 3], base: [f32; 3], wave: [f32; 3], shade: [f32; 3], glint: [f32; 3] }` in `k3c-platform`, Layout wie die Uniforms von `river.kage` (`rows` = 6 Reihen mal vec4).
- `Canvas::river(&mut self, dst: Rect, u: &RiverUniforms) -> bool` mit Standard `false` (Aufrufer fällt auf Kacheln zurück); Clip, Ebene und Reihenfolge wie bei `rect`.
- `Frame` (batch.rs, frame_canvas.rs) zeichnet den Aufruf als eigenen Run `Kind::River` samt Uniforms auf und meldet `true`; Reihenfolge innerhalb der Ebene bleibt erhalten.
- Deterministisch, keine Allokation je Aufruf außer dem Push in einen Vec.

## Nicht-Ziele

WGSL und Pipeline (B-1072), Pixelrechnung und SoftCanvas (B-1071), Aufruf aus `world` (B-1073). Der GPU-Backend-Pfad ignoriert `Kind::River`, bis B-1072 ihn belegt.

## Regeln und Einschränkungen

Entscheidung 008 (Rust-Port), `docs/standards/rust-port.md`. Datei höchstens 400 Zeilen, Funktion höchstens 60 Zeilen.

Planungsangaben für die Arbeitswellen:

- **Bereich:** `rust/crates/k3c-platform/src/lib.rs`, `rust/crates/k3c-platform/src/batch.rs`, `rust/crates/k3c-platform/src/batch_tests.rs`, `rust/crates/k3c-platform/src/frame_canvas.rs`, `rust/crates/k3c-platform/src/headless.rs`, `rust/crates/k3c-platform/src/backend/gpu.rs`
- **Abhängig von:** –
- **Braucht 🧑:** –
- **Größe:** S
- **Sperre:** –

## Beispiele

`Frame.river(rect, &u)` ruft `true` zurück und legt einen Run `Kind::River` an. `NullCanvas.river(..)` und jede andere Canvas ohne Überschreibung liefern `false`.

## Ausnahme- und Fehlerfälle

Leeres Rechteck (Breite oder Höhe höchstens 0): kein Run, Rückgabe `true`. Rechteck außerhalb des Clips: Run bleibt, der Scissor schneidet (wie `rect`).

## Akzeptanzkriterien

- **AC-01** `RiverUniforms` und `Canvas::river` mit Standard `false` existieren; `NullCanvas` liefert `false` (Test in `headless` oder `batch_tests.rs`).
- **AC-02** `Frame::river` legt genau einen Run `Kind::River` mit den übergebenen Uniforms an und liefert `true`; Reihenfolge gegenüber vorherigen und folgenden `rect`-Aufrufen derselben Ebene bleibt (Test in `batch_tests.rs`).
- **AC-03** `cargo test -p k3c-platform` und `task rust:lint` sind grün; der GPU-Backend-Build kompiliert mit dem neuen `Kind`.

## Offene Fragen

keine

## Notizen

Aus B-1055 zerlegt. Reihenfolge: B-1070, dann B-1071 und B-1072 parallel, zuletzt B-1073.

### Ergebnis

Umgesetzt: `RiverUniforms` (`k3c-platform/src/river.rs`), `Canvas::river` mit Standard `false`, `Frame::river` mit eigenem Run `Kind::River` (nie gebündelt; `Frame::river_calls` liefert Rechteck und Uniforms in Zeichenreihenfolge), leeres Rechteck: kein Run, `true`. GPU-Backend und `SoftCanvas` überspringen `Kind::River` bis B-1071/B-1072. Tests in `batch_tests.rs` (AC-01 bis AC-03). `cargo test -p k3c-platform -p k3c-game` und `task rust:lint` grün.
