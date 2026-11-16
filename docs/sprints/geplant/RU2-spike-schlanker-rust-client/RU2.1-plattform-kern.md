# RU2.1 · Plattform-Kern: Fenster, Schleife, Sprite-Bündelung, headless

- **Status:** offen
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

Wird am Ende der Session ausgefüllt: Nachweis je Kriterium (`AC-01 geprüft: task check grün`,
`AC-02 verschoben: Grund, B-0NN`), wer manuell geprüft hat, Abweichungen vom Plan, neue Tickets. Bis dahin `–`.
