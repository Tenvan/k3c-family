# RU4 · CLI · Plattform-Schicht: Schleife mit und ohne Fenster, Eingabe, Ton, Text

- **Status:** aktiv
- **Projekt:** RSU
- **Domäne:** CLI
- **Reife:** bereit
- **Tickets:** –
- **Start-Commit:** – (wird beim Aktivieren gesetzt: `git rev-parse --short origin/develop`)
- **Spec:** freigegeben
- **Revision:** 2
- **Freigabe:** Ralf, 2026-11-17

## Ausgangslage

RU2 hat die Crate `rust/crates/k3c-platform` gebaut (Stand RU2.1/RU2.3 › Ergebnis): Fenster (winit 0.30), wgpu 30 (Vulkan, DX12), Schleife mit eigener Zeitsammlung (`Clock`: Takt 30 Hz = `STEP`, höchstens `MAX_STEPS` = 4 je Bild, alpha an `draw`, VSync Fifo), `trait App { step(dt), draw(canvas, alpha), done() }`, `trait Canvas` mit `size`, `layer(z)`, `texture(rgba, w, h)`, `sprite(tex, src, dst, tint)` (Instanzen, ein Draw-Call je Textur und Ebene), `rect`, Pixel-Schrift `text` (5×7, nur Großbuchstaben), `Frame` als Sammler, `NullCanvas` + `run_headless(app, steps)`, `Measure`/`Summary`, `Config { title, measure, max_seconds, fullscreen }`. Tests: `tests/alloc.rs` (0 Allokationen je Leerlauf-Bild), `tests/layering.rs` (winit/wgpu nur in `src/backend/`). Esc und F (Vollbild) wertet heute das Backend selbst aus.

Es fehlt für das Spiel: Eingabe an die App (Tastatur als Schnappschuss, Gamepads), Eingabe-Skript für headless, Beschneiden auf eine Bildhälfte, gedrehte/skalierte Sprites (GeoM), Dreiecksnetze (Rig, Licht), Formen (`vector.*`), Filter je Textur, Texturen freigeben, Schrift aus `public/schrift/k3c.ttf` mit Umlauten, Ton, Log mit Emoji-Regel. Dazu der Port der Eingabe-Auswertung `native/k3c-game/input` samt der Verträge, die sie aus `native/k3c-game/view` braucht (input, profile, settings, devkeys; alle ohne Sim-Typen bis auf `Command.Sim`).

## Ziel

`k3c-platform` deckt alles ab, was das Go-Spiel von Ebitengine nutzt (außer Shadern und Blend-Modi), und die Eingabe-Auswertung ist portiert. Am Ende sichtbar: `cargo run -p k3c-platform --example probe` zeigt zwei Bildhälften, je Spieler (Tastatur, Pad) ein bewegtes Kästchen, Text mit Umlauten und Ton auf Tastendruck; derselbe Lauf mit `--headless --steps 1000 --script <datei>` endet ohne Fenster und Tongerät mit Code 0.

## Beteiligte und Zielgruppen

Agenten setzen um, die Review-Session prüft. 🧑 gibt die Spec frei und entscheidet an den Gates (B-1014).

## Anforderungen

`B-1014 › Anforderungen` (Port ohne Regeländerung, Golden als Orakel, zwei Spieler).

Welle 3 von 26 (B-1015), überarbeitet nach RU2 (Revision 2). Für jede Session gilt der Port-Leitfaden `docs/standards/rust-port.md` (B-1018), § 8 Client. Modell: `sonnet`.

### Sessions

1. RU4.1 Schleife: Eingabe-Schnappschuss, Eingabe-Skript, Bildhälften (`clip`), Vollbild über die App (AC-01, AC-02)
2. RU4.2 Canvas Teil 2: Transform, Dreiecksnetz, Formen, Filter, Freigeben (AC-01)
3. RU4.3 Schrift: TTF mit Umlauten, Breite messen, Rückfall Pixel-Schrift (AC-05) – braucht Crate-Freigabe
4. RU4.4 Gamepads im Backend, Xbox-Belegung, Geräte kommen und gehen (AC-03) – braucht Crate-Freigabe
5. RU4.5 view-Verträge der Eingabe: `view/input`, `profile`, `settings`, `devkeys` (AC-03)
6. RU4.6 Eingabe-Port Teil 1: `input/bindings`, `input/tracker` (AC-03)
7. RU4.7 Eingabe-Port Teil 2: `input/source` an `InputState` (AC-03)
8. RU4.8 Ton: `AudioOut`, Gerät, `NullAudio` (AC-04) – braucht Crate-Freigabe
9. RU4.9 Log-Regel und Probe-Programm für beide Modi (AC-02, AC-05)
10. RU4.10 Review

Abhängigkeiten in den Session-Dateien; RU4.2, RU4.4, RU4.5 und RU4.8 setzen nur RU4.1 voraus.

### Vorentscheidungen

Die Schnittstellen stehen fest; die Sessions bauen sie, sie entwerfen sie nicht neu. Bestehendes aus RU2 bleibt, wo nichts anderes steht.

- `trait App { fn step(&mut self, input: &InputState, dt: f64); fn draw(&mut self, c: &mut dyn Canvas, alpha: f64); fn done(&self) -> bool; fn fullscreen(&self) -> Option<bool> { None } }`. `fullscreen()` ist der gewünschte Zustand; die Schleife setzt ihn nach jedem Takt, wenn er abweicht. Das Backend wertet keine Taste mehr selbst aus (Esc und F wandern in die Demo-App).
- `InputState` (neu, `src/input.rs`, ohne winit): `keys` mit gehaltenen Tasten und Flanken seit dem letzten Takt (`held(Key)`, `just_pressed(Key)`; ein Tipp kürzer als ein Takt geht nicht verloren), `pads: Vec<PadState>` mit `id` (neu je Verbindung, wie `ebiten.GamepadID`), `held(PadButton)`, `axis(PadAxis) -> f64`. `Key` ist ein eigenes `enum` mit allen Tasten, die `native/k3c-game` nutzt (`grep -rhoE "ebiten\.Key[A-Za-z0-9]+" native/k3c-game`). `PadButton` nur mit Xbox-Namen: `A, B, X, Y, LB, RB, LT, RT, View, Menu, Home, LS, RS, Up, Down, Left, Right`; `PadAxis`: `LX, LY, RX, RY, LT, RT`. Der Schnappschuss wird je Takt wiederverwendet (keine Allokation).
- `InputScript` (JSON, `serde`): Liste aus `{ "step": n, "keys": ["D"], "pads": [{ "id": 0, "buttons": ["A"], "axes": { "LX": 1.0 } }] }`; ein Eintrag gilt bis zum nächsten. `run_headless(app, steps, script: Option<&InputScript>) -> NullCanvas`.
- Ein gemeinsamer Schleifen-Kern (`src/runner.rs`: Uhr, Eingabe, `step`, `draw`) treibt Fenster und headless; im Test läuft er mit einer simulierten Uhr. Dadurch ist „1000 Schritte gleich“ ohne Fenster prüfbar.
- Bildhälften: `Canvas::clip(&mut self, area: Option<Rect>)` beschneidet die folgenden Aufrufe auf `area` (Scissor), die Koordinaten bleiben Fensterkoordinaten wie bei Ebitengine-`SubImage` (`native/k3c-game/view/layer.go` › Drawer). Gebündelt wird nach (Ebene, Ausschnitt, Textur); kein Offscreen-Ziel.
- Canvas Teil 2: `sprite_tf(tex, src, tf: Affine, tint)` mit `Affine { a, b, c, d, tx, ty }` wie `ebiten.GeoM`; `mesh(tex, verts: &[Vertex], idx: &[u16])` wie `DrawTriangles`; `fill_rect`/`stroke_rect`, `fill_circle`/`stroke_circle`, `stroke_line`, `fill_path(points)` (Polygon aus Geraden, Ohren-Schnitt im eigenen Code) wie `vector.*`; `texture_with(rgba, w, h, Filter::{Linear, Nearest})`; `free_texture(TexId)`. Das bestehende `sprite(tex, src, dst, tint)` und `rect` bleiben.
- Schrift: `Canvas::font(ttf: &[u8]) -> Result<FontId, String>`, `text_font(font, s, x, y, px, color)`, `text_width(font, s, px) -> f32`; Glyphen-Atlas mit Cache, `NullCanvas` misst gleich. Die Pixel-Schrift `text` bleibt als Rückfall.
- Ton: `trait AudioOut` (`src/audio.rs`): `load(bytes) -> Result<SoundId, String>` (OGG, MP3, WAV, 44,1 kHz), `play(sound, volume) -> VoiceId`, `play_loop`, `set_volume(voice, v)`, `stop(voice)`, `playing(voice)`, `failed() -> Option<String>`. `open_audio() -> Box<dyn AudioOut>` liefert das Gerät oder bei Fehler `NullAudio` mit Warnung; headless immer `NullAudio`. Der Ton hängt nicht an `run_window`.
- Log: `k3c_platform::log::{debug, info, warn, error}(msg, &[("k", &v)])` nach stderr; `EMOJIS` ist die Liste aus `CLAUDE.md` › Logging; `debug_assert!` auf das erste Zeichen.
- Fremde Crates (Freigabe 🧑 nötig, Vorschlag): `fontdue` (Schrift), `gilrs` (Gamepads, XInput/WGI), `rodio` ohne Standard-Features, nur Wiedergabe plus Vorbis, MP3, WAV (Ton). Versionen fest mit `=` in `rust/Cargo.toml`. Ohne Freigabe bleiben RU4.3, RU4.4 und RU4.8 blockiert; die übrigen Sessions laufen.
- Shader (Fluss, `world/water`) und Blend-Modi (Licht) sind nicht Teil von RU4; braucht RU13 sie, entsteht dort ein Ticket.

### Hinweise für die Umsetzung

- Nur `src/backend/` importiert winit, wgpu, gilrs und das Ton-Gerät; `tests/layering.rs` wird um die neuen Crate-Namen erweitert.
- Headless heißt: kein Fenster, kein Grafikkontext, kein Tongerät, keine Gamepad-Abfrage.
- Im Schritt und im Zeichnen keine Allokation je Bild; `tests/alloc.rs` deckt jede neue Canvas-Funktion mit ab.
- `k3c-game/src/demo` implementiert `App`; jede Änderung an `App` zieht die Demo mit (`--headless --steps 600` bleibt grün).
- Ablage des Ports: `native/k3c-game/input/` → `rust/crates/k3c-game/src/input/`, die vier view-Dateien → `rust/crates/k3c-game/src/view/` (Leitfaden § 2). `Command.Sim()` aus `view/input.go` braucht `sim::PlayerCommand` (RU8) und kommt in RU7.5; Tests dazu bekommen `#[ignore = "wartet auf RU8"]`.
- `native/k3c-game/audio` (Mischen, Bank, Atlas, Ducking, Wiedergabe) portiert RU17 auf `AudioOut`; RU4 liefert nur die Plattform.

## Nicht-Ziele

Regel-, Wert- oder Formatänderungen. Änderungen am Go-Code. Optimierungen über den Go-Stand hinaus.

## Regeln und Einschränkungen

Entscheidung 008 gilt, bis 🧑 Entscheidung 011 annimmt (B-1014). Der Go-Code bleibt unverändert lauffähig (Rückfall). Golden werden nie zugunsten von Rust neu geschrieben. Datei ≤ 400 Zeilen, Funktion ≤ 60 Zeilen. Neue Crates nur mit Freigabe 🧑. Die Sim-Sperre vom 2026-11-11 gilt; Golden-Vergleiche laufen nur als Tests und nur mit der Ausnahme aus B-1014.

- **Bereich:** `rust/crates/k3c-platform/**`, `rust/crates/k3c-game/src/input/**`, `rust/crates/k3c-game/src/view/mod.rs`, `rust/crates/k3c-game/src/view/input*.rs`, `rust/crates/k3c-game/src/view/profile*.rs`, `rust/crates/k3c-game/src/view/settings*.rs`, `rust/crates/k3c-game/src/view/devkeys*.rs`, `rust/crates/k3c-game/src/demo/**`, `rust/crates/k3c-game/src/lib.rs`, `rust/Cargo.toml`, `rust/Cargo.lock`
- **Abhängig von:** RU2
- **Braucht 🧑:** Freigabe der Crates `fontdue`, `gilrs`, `rodio` (RU4.3, RU4.4, RU4.8)
- **Sperre:** –

## Beispiele

nicht relevant: Port ohne neues Verhalten; die Beispiele sind die Go-Tests des Bereichs.

## Ausnahme- und Fehlerfälle

Weicht Rust von Go ab, gilt Go: Die Abweichung wird behoben oder als Ticket erfasst, nie durch ein geändertes Golden verdeckt. Fehlt eine Voraussetzung aus einem anderen Sprint, entsteht ein Ticket statt einer Änderung außerhalb des Bereichs.

## Akzeptanzkriterien

- **AC-01** Die Schleife rechnet mit festem Takt (30 Hz, höchstens 4 Takte je Bild, alpha an `draw`) und übergibt je Takt einen `InputState`; `Canvas::clip` beschneidet auf eine Bildhälfte; `Canvas` kann zusätzlich Transform (`Affine` wie GeoM), Dreiecksnetz, Linie, Kreis, gefülltes Polygon, Filter je Textur und Textur freigeben; Vollbild schaltet die App über `App::fullscreen`, das Fenster ist in der Größe veränderbar. Im Leerlauf keine Allokation je Bild.
- **AC-02** `run_headless(app, steps, script)` läuft ohne Fenster, Grafikkontext, Tongerät und Gamepad-Abfrage; ein Test treibt dieselbe App mit demselben `InputScript` 1000 Takte über den Schleifen-Kern mit simulierter Uhr und headless und vergleicht den Zustand gleich.
- **AC-03** Tastatur und Gamepads (Xbox-Belegung, nur Xbox-Namen) sind je ein eigenes Gerät, Geräte kommen und gehen zur Laufzeit; der Port von `native/k3c-game/input` liefert je Gerät Befehle und Menü-Aktionen wie Go (Tests portiert); B ist `ActionBack` in Menüs und im Spiel unbelegt.
- **AC-04** `AudioOut` spielt OGG, MP3 und WAV mit Lautstärke je Stimme und Schleifen; ohne Gerät oder bei Gerätefehler läuft alles stumm weiter (Warnung, kein Absturz); headless `NullAudio` ohne Gerät und ohne Fehler.
- **AC-05** Text mit Umlauten in `public/schrift/k3c.ttf` (Größe, Farbe, Breite messen); fehlt die Datei, Pixel-Schrift mit Warnung. Jede Log-Meldung der Rust-Crates beginnt mit einem Emoji aus der Liste in `CLAUDE.md`; ein Test prüft die Quelltexte.

## Offene Fragen

- Freigabe der Crates `fontdue`, `gilrs`, `rodio` (Vorschlag in Vorentscheidungen). Default ohne Antwort: RU4.3, RU4.4, RU4.8 bleiben blockiert, der Rest läuft.
- `rust/Cargo.lock` ändert sich auch, wenn RU3 Test-Abhängigkeiten ergänzt. Bei parallelem Lauf entsteht nur dort ein Merge-Konflikt; Lösung: Lock aus `main` nehmen und `cargo build` neu schreiben lassen.

## Sessions

| Nr. | Datei | Typ | Agent | Status |
|---|---|---|---|---|
| RU4.1 | `RU4.1-schleife-eingabe-bildhaelften.md` | Umsetzung | autonom | fertig |
| RU4.2 | `RU4.2-canvas-transform-netz-formen.md` | Umsetzung | autonom | fertig |
| RU4.3 | `RU4.3-schrift-ttf-umlaute.md` | Umsetzung | autonom | fertig |
| RU4.4 | `RU4.4-gamepads-xbox.md` | Umsetzung | autonom | fertig |
| RU4.5 | `RU4.5-view-vertraege-eingabe.md` | Umsetzung | autonom | fertig |
| RU4.6 | `RU4.6-eingabe-port-bindings-tracker.md` | Umsetzung | autonom | fertig |
| RU4.7 | `RU4.7-eingabe-port-source.md` | Umsetzung | autonom | fertig |
| RU4.8 | `RU4.8-ton-audioout.md` | Umsetzung | autonom | fertig |
| RU4.9 | `RU4.9-log-und-probe.md` | Umsetzung | autonom | fertig |
| RU4.10 | `RU4.10-review.md` | Review | autonom | offen |

Bei `Reife: Entwurf` genügen Stichpunkte mit den Kriterien in Klammern statt Session-Dateien. Vor dem Aktivieren
wird jede Session als eigene Datei nach `docs/vorlagen/session.md` geschrieben. Keine eigenen Sim- oder
Balance-Sessions vor der Abnahme: Sim, Balance, Gen und `check:race` laufen im Abnahme-Ticket (B-742).

## Abnahme

Wird von der Review-Session (Doku-Sprint: letzte Session) ausgefüllt, höchstens fünf Zeilen: Datum, Kriterien
(Verweis auf Session-Ergebnisse), behobene Befunde, neue Tickets und das Abnahme-Ticket für Sim, Balance und Gen
(genau eins, `docs/arbeitsweise.md` › Sim, Balance und Gen). Bis dahin `–`.
