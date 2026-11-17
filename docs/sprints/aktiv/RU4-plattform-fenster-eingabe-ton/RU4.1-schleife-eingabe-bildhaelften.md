# RU4.1 · Schleife: Eingabe-Schnappschuss, Skript, Bildhälften, Vollbild

- **Status:** fertig
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

- **AC-01 geprüft:** `cargo test -p k3c-platform` grün (28 Unit-Tests); `tests/alloc.rs` läuft jetzt über `Runner` mit Tastenflanken, Pad-Taste/Achse und zwei `clip`-Hälften: 0 Allokationen in 120 Leerlauf-Bildern, 3 Bündel (zwei Hälften + Vollfarbe). `Frame` bündelt nach (Ebene, Ausschnitt, Textur), gleicher Ausschnitt = gleicher Index; `backend/gpu.rs` setzt den Scissor je Bündel (logisch → Surface-Pixel, begrenzt, leere Fläche übersprungen; Test `ausschnitt_wird_auf_surface_gestreckt_und_begrenzt`).
- **AC-01:** `App::step(&InputState, dt)`, `App::fullscreen() -> Option<bool>`; das Backend wertet keine Taste mehr aus (`backend/keys.rs` winit → `Key`, physisch; Wiederholungen zählen nicht; Fokusverlust lässt alle Tasten los) und setzt Vollbild nach den Takten, wenn die App anderes will. Demo: Esc → `done`, F schaltet `fullscreen()` (Test `esc_beendet_und_f_schaltet_vollbild`). **Ralf prüft:** `cargo run --release -p k3c-game -- demo --driver rust/demo/reiten.json` zeigt die Demo wie vorher, F schaltet Vollbild, Esc beendet, Fenster lässt sich ziehen.
- **AC-02 geprüft:** `runner_tests.rs` › `tausend_takte_mit_simulierter_uhr_gleich_headless`: 1000 Takte mit unregelmäßiger Bildzeit (2–250 ms, 0 bis 4 Takte je Bild) über `Runner` gegen `run_headless(app, 5000, Some(&script))` mit demselben 50-Einträge-Skript (Tasten, Tipps, Pads kommen und gehen) – Zustand samt Prüfsumme gleich.
- **Lauf:** `demo --headless --steps 600` Code 0; mit `--script` (F in Takt 5, Esc in Takt 100) endet sie nach Takt 100 mit Code 0. `cargo clippy --workspace --all-targets -D warnings` grün, `task check:diff` grün.
- **API:** `InputState { keys: KeyState, pads }` mit `held`, `just_pressed` (auch `just_released`, `release_all`), `pads()`, `pad(id)`, `pad_mut(id)` (verbindet), `disconnect`, `end_step`; `PadState { id, held, just_pressed, axis, set_button, set_axis }`; `Key` (48 Tasten aus `ebiten.Key*`, `Key::ALL`, Bitfeld u64), `PadButton` (17 Xbox-Namen), `PadAxis` (6); `InputScript::from_json`/`new`/`apply(step, &mut InputState)`; `Runner::new(app, script)`, `advance(elapsed, canvas) -> Takte`, `input_mut`, `app`, `steps`; `run_headless(app, steps, script)`; `NullCanvas.clips`; Demo-Option `--script <datei>`.
- **Abweichungen:** (1) Pads haben zusätzlich `just_pressed` (Flanke wie bei Tasten), damit ein kurzer Tipp auch am Pad nicht verloren geht. (2) Das Skript setzt den Zustand nur in dem Takt, in dem ein Eintrag beginnt; dazwischen bleibt er (gleich „gilt bis zum nächsten“). (3) `Runner` löscht die Flanken nach jedem Takt (nach dem ersten Takt eines Bildes kommen ohnehin keine neuen). Die Grafik liegt im Worktree nicht vor (B-737); der Lauf nutzte `--grafik` der Repo-Wurzel.
