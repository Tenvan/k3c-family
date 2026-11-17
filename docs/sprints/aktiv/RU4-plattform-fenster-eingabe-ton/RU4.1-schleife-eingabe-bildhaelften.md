# RU4.1 · Schleife: Eingabe-Schnappschuss, Skript, Bildhälften, Vollbild

- **Status:** offen
- **Typ:** Umsetzung
- **Agent:** autonom
- **Domäne:** CLI
- **Lane:** –
- **Modell:** sonnet
- **Umgebung:** offline
- **Branch:** sprint/ru4
- **Abhängig von:** –
- **Tickets:** B-1014
- **Kriterien:** AC-01, AC-02

## Ziel

`k3c-platform` übergibt der App je Takt einen Eingabe-Schnappschuss, läuft headless mit Eingabe-Skript über denselben Schleifen-Kern wie im Fenster und beschneidet Zeichenaufrufe auf eine Bildhälfte.

## Kontext

Stand aus RU2 (`docs/sprints/erledigt/RU2-spike-schlanker-rust-client/RU2.1-plattform-kern.md` und `RU2.3-demo-szene-und-driver.md` › Ergebnis): `rust/crates/k3c-platform/src/` mit `lib.rs` (`App`, `Canvas`, `Rect`, `TexId`, `LOGICAL`), `clock.rs` (`Clock`, `STEP` = 1/30 s, `MAX_STEPS` = 4), `batch.rs` (`Frame` sammelt Instanzen, Sortierschlüssel Ebene/Textur/Reihenfolge), `headless.rs` (`NullCanvas`, `run_headless(app, steps)`), `backend/mod.rs` (winit-Schleife; wertet heute Esc und F selbst aus), `backend/gpu.rs` (wgpu). Tests: `tests/alloc.rs`, `tests/layering.rs`. Die Demo `rust/crates/k3c-game/src/demo/mod.rs` implementiert `App`.

Vorgaben: RU4-README › Vorentscheidungen (`App` mit `input` und `fullscreen()`, `InputState`, `Key`, `PadButton`, `PadAxis`, `InputScript`, `runner.rs`, `Canvas::clip`). Go-Vorbild für die Bildhälfte: `native/k3c-game/view/layer.go` (Drawer bekommt `SubImage`, Koordinaten bleiben Fensterkoordinaten) und `view/cell.go`. Die Tastenliste: `grep -rhoE "ebiten\.Key[A-Za-z0-9]+" native/k3c-game | sort -u`. Gamepads füllt erst RU4.4; hier nur die Typen und das Skript.

## Erlaubte Dateien

- `rust/crates/k3c-platform/**`
- `rust/crates/k3c-game/src/demo/**`

## Nicht-Ziele

Gamepad-Abfrage (RU4.4), neue Zeichenformen (RU4.2), Schrift (RU4.3), Ton (RU4.8). Neue Crates.

## Schritte

1. `src/input.rs`: `Key` (alle Tasten aus der grep-Liste, Namen ohne `Key`-Präfix), `PadButton`, `PadAxis`, `KeyState` (Bitfeld gehalten + Flanken seit dem letzten Takt), `PadState { id, held, axes }`, `InputState { keys, pads }` mit `held`, `just_pressed`, `axis`; `serde` für `InputScript` (Abhängigkeit `serde` aus dem Workspace in `k3c-platform/Cargo.toml`). Kein winit-Typ in dieser Datei.
2. `src/runner.rs`: Schleifen-Kern aus `Clock`, `InputState` und App (`advance(elapsed, &mut dyn Canvas)`); `backend/mod.rs` und `headless.rs` nutzen ihn. Flanken werden nach dem ersten Takt eines Bildes gelöscht.
3. `App::step(&mut self, input: &InputState, dt)` und `App::fullscreen() -> Option<bool>` (Standard `None`); die Schleife setzt Vollbild nach jedem Takt, wenn es abweicht. `backend/mod.rs`: winit-Tasten → `Key` (Zuordnung in `backend/keys.rs`), kein Esc/F mehr im Backend; Fenster bleibt in der Größe veränderbar.
4. `run_headless(app, steps, script: Option<&InputScript>)`; `InputScript::from_json(&str)`.
5. `Canvas::clip(&mut self, area: Option<Rect>)` (Standard leer); `Frame` bündelt nach (Ebene, Ausschnitt, Textur); `backend/gpu.rs` setzt den Scissor je Bündel (logisch → Pixel wie die bestehende Skalierung). `NullCanvas` zählt `clips`.
6. Demo: `step` nimmt `input`; Esc setzt `done`, F schaltet `fullscreen()` um.
7. Tests: `src/input_tests.rs` (Flanken, Skript lesen und abspielen), `src/runner_tests.rs` (1000 Takte mit simulierter Uhr gegen `run_headless` mit demselben Skript, Zustand gleich), `tests/alloc.rs` um `clip` und Eingabe erweitern.

## Fertig, wenn

- [ ] AC-01: `cargo test -p k3c-platform` grün; `tests/alloc.rs` zählt 0 Allokationen je Leerlauf-Bild mit `clip` und Eingabe.
- [ ] AC-01: `cargo run --release -p k3c-game -- demo --driver rust/demo/reiten.json` zeigt die Demo wie vorher; Esc beendet, F schaltet Vollbild (über `App::fullscreen`).
- [ ] AC-02: Test in `runner_tests.rs` rechnet 1000 Takte im Schleifen-Kern und headless mit demselben `InputScript`, der Zustand ist gleich.
- [ ] `demo --headless --steps 600` endet mit Code 0; `task rust:lint` grün.

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
