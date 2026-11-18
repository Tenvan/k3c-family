# B-1072 · river.kage als WGSL-Pipeline im GPU-Backend

- **Domäne:** CLI
- **Typ:** Schuld
- **Prio:** niedrig
- **Umgebung:** offline
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

Das GPU-Backend (`rust/crates/k3c-platform/src/backend/`: `setup.rs`, `gpu.rs`, `shader.wgsl`) hat eine Pipeline für Quads und Dreiecke. `water/river.kage` (Go, Kage) hat dort kein Gegenstück. Teil 3 von 4 aus B-1055.

## Ziel

`river.kage` als `river.wgsl` mit eigener Pipeline und Uniform-Puffer; `Kind::River`-Runs aus B-1070 werden im Backend gezeichnet.

## Beteiligte und Zielgruppen

Wer spielt, entwickelt, betreibt oder entscheidet (🧑)? Keine Verantwortlichen erfinden.

## Anforderungen

- `backend/river.wgsl`: Fragment 1:1 nach `river.kage` (Uniforms `origin`, `rows[6]`, `phase`, vier Farben), Ausgabe in der Farbraum-Konvention von `shader.wgsl`.
- Eigene Render-Pipeline in `setup.rs`, Uniform-Puffer je River-Run (Dynamic Offset oder ein Puffer pro Frame), `gpu.rs` zeichnet `Kind::River` mit Scissor, Ebene und Blend wie die anderen Runs.
- Der Shader-Text wird ohne GPU per `naga` validiert (Test), damit CI ohne Adapter prüft.

## Nicht-Ziele

Anbindung in `world` (B-1073), Software-Fassung (B-1071), Sichtvergleich auf einer GPU (Ralf prüft im Spiel, siehe B-1073).

## Regeln und Einschränkungen

`docs/standards/rust-port.md`. Keine neue Abhängigkeit: `naga` ist über `wgpu` schon im Baum (sonst Sperre `deps` und Ticket anhalten).

Planungsangaben für die Arbeitswellen:

- **Bereich:** `rust/crates/k3c-platform/src/backend/river.wgsl`, `rust/crates/k3c-platform/src/backend/setup.rs`, `rust/crates/k3c-platform/src/backend/gpu.rs`, `rust/crates/k3c-platform/src/backend/mod.rs`
- **Abhängig von:** B-1070
- **Braucht 🧑:** –
- **Größe:** S
- **Sperre:** –

## Beispiele

Ein Frame mit einem River-Run und einem Sprite danach: beide werden in Aufrufreihenfolge gezeichnet.

## Ausnahme- und Fehlerfälle

Shader lässt sich nicht übersetzen: Backend meldet den Fehler im Log und überspringt River-Runs; die Aufrufer fallen über die Rückgabe von `Canvas::river` auf Kacheln zurück.

## Akzeptanzkriterien

- **AC-01** `river.wgsl` besteht die `naga`-Validierung (Test ohne GPU).
- **AC-02** Die Pipeline wird in `setup.rs` angelegt; `gpu.rs` behandelt `Kind::River` ohne `unreachable`/`todo`; `cargo build -p k3c-platform` mit GPU-Backend ist grün.
- **AC-03** Ein Test mit der Referenz-Funktion aus B-1071 und `naga`-Auswertung der Konstanten (`tau`, `srcH`) zeigt dieselben Werte; `cargo test -p k3c-platform` und `task rust:lint` sind grün.

## Offene Fragen

keine

## Notizen

Aus B-1055 zerlegt. Kann parallel zu B-1071 laufen (andere Dateien); AC-03 nutzt B-1071 nur als Wertquelle, notfalls Zahlen aus `river.kage` direkt.

### Ergebnis

`backend/river.wgsl` (Fragment 1:1 nach `river.kage`, Uniform-Struct aus vec4-Feldern, 208 Byte), Pipelines je Mischung und Bind-Layout mit dynamischem Versatz in `setup.rs` (`river_layout`, `river_stride`, `river_pipeline`, `river_pipelines`), Packung `pack_river` in `river.rs`, `gpu.rs` zeichnet `Kind::River` (ein Uniform-Puffer je Bild, Versatz je Aufruf, Scissor und Blend wie die anderen Runs); übersetzt der Shader nicht, Log-Zeile und Fluss-Aufrufe werden übersprungen. Keine neue Abhängigkeit: `wgpu::naga` (Re-Export, naga 30.0.2 schon im Lockfile) validiert ohne GPU. Tests: naga-Validierung beider Shader, Konstanten `tau`/`srcH` gleich `river.rs` (dafür im WGSL als `const …: f32`), Packungs-Layout, Pipeline-Aufbau auf einem Adapter (ohne Adapter übersprungen; lief hier). `cargo test -p k3c-platform -p k3c-game` und `task rust:lint` grün. Nicht geprüft: Bild der GPU-Fassung im Fenster (Ralf, B-1059).
