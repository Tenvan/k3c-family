# RU2.1 · Plattform-Kern: Fenster, Schleife, Sprite-Bündelung, headless

- **Status:** fertig
- **Typ:** Umsetzung
- **Agent:** autonom
- **Domäne:** CLI
- **Lane:** –
- **Modell:** sonnet
- **Umgebung:** live
- **Branch:** sprint/ru2
- **Abhängig von:** RU1.1
- **Tickets:** B-1016
- **Kriterien:** AC-01

## Ziel

Crate `rust/crates/k3c-platform`: Fenster, wgpu, Schleife mit eigener Zeitsammlung, `App`/`Canvas` mit gebündelten Sprites, Pixel-Schrift, headless. Ein Probe-Programm zeigt 20 000 bewegte Sprites ruckelfrei.

## Kontext

Stack aus RU2 › Anforderungen. Vorlage (lokal, nicht eingecheckt): `.work/vergleich/src/rust-last/main.rs` (winit 0.30 `ApplicationHandler`, wgpu 23, Instanzen 48 Byte, ein Draw-Call, Textur mit vormultipliziertem Alpha, `ControlFlow::Poll` + `request_redraw`) und `font.rs` (5×7-Pixel-Schrift). Fehlt der Ordner: aus RU0.2 › Ergebnis und dieser Datei nachbauen.

Schnittstelle (fest, wie RU4 › Vorentscheidungen, hier nur der Teil für Grafik):
- `trait App { fn step(&mut self, dt: f64); fn draw(&mut self, c: &mut dyn Canvas, alpha: f64); fn done(&self) -> bool; }`
- `trait Canvas { fn size(&self) -> (f32, f32); fn texture(&mut self, rgba: &[u8], w: u32, h: u32) -> TexId; fn sprite(&mut self, t: TexId, src: Rect, dst: Rect, tint: [f32; 4]); fn rect(&mut self, dst: Rect, color: [f32; 4]); fn text(&mut self, s: &str, x: f32, y: f32, px: f32, color: [f32; 4]); }`
- Schleife: Takt 1/30 s, Zeit sammeln, je Bild so viele `step` wie die Zeit hergibt (höchstens 4), alpha = Rest / Takt an `draw`. Logische Auflösung 1920×1080, auf das Fenster skaliert.
- Sprites werden je Bild gesammelt, nach Textur sortiert (Reihenfolge innerhalb der Textur bleibt) und mit einem Instanz-Puffer gezeichnet.

## Erlaubte Dateien

- `rust/crates/k3c-platform/**`
- `rust/Cargo.toml` (nur Mitglied und Abhängigkeiten eintragen)

## Nicht-Ziele

Eingabe, Ton, Bildhälften, Meshes (RU4). Grafik-Dateien laden (RU2.2).

## Schritte

1. Crate anlegen, Abhängigkeiten `winit`, `wgpu`, `pollster`.
2. `backend/` (einziger Ort mit winit/wgpu): Fenster, Surface Fifo, Pipeline für Instanzen (Textur oder Vollfarbe), Puffer wächst bei Bedarf.
3. `run_window(app, cfg)` mit Zeitsammlung; Fenstertitel je Sekunde: Backend, Bilder/s, späte Bilder (> 25 ms).
4. `NullCanvas`, `run_headless(app, steps)`.
5. Messung je Bild (dt, CPU-Zeit bis `present`) als Ringpuffer; `--measure` schreibt JSON (Median, p99, längstes, späte je s).
6. Probe `examples/probe.rs`: gelbes Quadrat + N Sprites (Argument), wie `.work/vergleich/src/rust-last`.
7. Tests: Zeitsammlung (60 Hz → abwechselnd 0/1 Takte, alpha in [0,1)), Sortierung, keine Allokation je Bild im Leerlauf (zählender Allocator im Test).

## Fertig, wenn

- [ ] AC-01: `cargo run --release -p k3c-platform --example probe -- n=20000` läuft mit 60 Bildern/s, Titel zeigt späte Bilder.
- [ ] AC-01: `cargo test -p k3c-platform` grün, inklusive Allokations-Test.
- [ ] AC-01: kein `winit`/`wgpu` außerhalb von `backend/` (Test liest die Dateien).

## Prüfen

```bash
task check:diff
```

Einmal vor dem Commit; während der Arbeit nur gezielte Tests (`docs/arbeitsweise.md` › Tempo im Lauf).
Review-Session statt dessen: `task check`, `task check:go` (nacheinander) und das Abnahme-Ticket des Sprints anlegen.
Sim-, Balance- und Gen-Läufe (`sim_test`, Balancing, `gen:paper` samt Golden, `check:race`) nur, wenn diese Session das
betroffene Feature entwickelt; sonst laufen sie im Abnahme-Ticket (`docs/arbeitsweise.md` › Sim, Balance und Gen).
Manuelle Prüfungen (Browser, Xbox, TV) nur, wenn diese Datei sie nennt und 🧑 sie für diesen Lauf freigegeben hat.

## Ergebnis

- **AC-01 geprüft:** `cargo run --release -p k3c-platform --example probe -- n=20000 secs=15` (Vulkan und DX12, Windows): Median Bildabstand 16,68 ms (60 Bilder/s), p99 17,5–17,7 ms, längstes Bild 33–35 ms; 1–3 späte Bilder (> 25 ms) je Lauf, alle in den ersten Sekunden (Fensterstart, parallel liefen andere Builds), danach 0 je Sekunde. Titel zeigt Backend, Bilder/s und späte Bilder. CPU-Zeit bis present: Vulkan Median 1,6 ms, DX12 16,3 ms (DX12 blockiert in submit/present, vermutet wegen Fifo-Kopplung; für RU2.4 beobachten).
- **AC-01 geprüft:** `cargo test -p k3c-platform` grün (16 Unit-Tests, `tests/alloc.rs` zählender Allocator: 0 Allokationen in 120 Leerlauf-Bildern mit 2000 Sprites, Rechteck und Text; `tests/layering.rs`: kein `winit::`/`wgpu::` außerhalb `src/backend/`). `cargo clippy --all-targets -D warnings` und `cargo fmt --check` grün; `task check:diff` grün.
- **Abweichungen:** (1) Crate-`Cargo.toml` führt eigene Lints (`unsafe_code = "deny"`, `too_many_lines = "deny"`) statt `workspace = true`, weil der Allokations-Test `unsafe` braucht; `lib.rs` hat `#![forbid(unsafe_code)]`. (2) wgpu 30 statt 23 der Vorlage: `request_device` ohne Trace, `Queue::present`, `Option`-Bind-Group-Layouts, `immediate_size`, `CurrentSurfaceTexture`. (3) Vollfarbflächen und Schrift tragen den Sortierschlüssel `SOLID` und werden nach allen Sprites gezeichnet (also obenauf); Zeichenreihenfolge zwischen Sprites und Rechtecken ist damit nicht frei. (4) Schrift: Großbuchstaben, Ziffern, Satzzeichen; Klein und Umlaute werden Groß, Unbekanntes ein Kästchen. (5) Instanzen laufen über einen wiederverwendeten Byte-Puffer und `write_buffer` (`write_buffer_with` liefert in wgpu 30 keinen Slice).
- **API:** `App`, `Canvas`, `Rect`, `TexId`, `Frame` (Sammler, implementiert `Canvas`), `Clock` (30 Hz, höchstens 4 Schritte), `run_window(app, Config)`, `run_headless(app, steps) -> NullCanvas`, `Measure`/`Summary::to_json`, `Config { title, measure, max_seconds }`.
- **Manuell:** nicht verlangt, nicht geprüft (kein Blick auf das Fenster durch Ralf).
