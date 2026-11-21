# rust/crates/k3c-game/src/game/

## Responsibility

Die Spielschleife des nativen Spiels (Port von `native/k3c-game/game/`): `Game` ist eine `k3c_platform::App` und verbindet Sim-Sitzung, Eingabe, Menü-Stapel und die Fachmodule (world, props, actors, fx, hud, Audio) zu Takt und Bild. Dazu Start, Abnahme- und Messmodi, Entwickler-Werkzeuge.

## Design

- Kern: `mod.rs` (`Game`, `new`, `frame`), `advance.rs` (ein Takt), `draw.rs` (ein Bild), `play.rs` (Takt ohne offenen Screen), `sim.rs` (Trait `Sim`: die Simulation aus Sicht der Schleife; `local::Session` implementiert ihn). Die Zeitsammlung entfällt: `k3c_platform::Clock` liefert 30 Hz, höchstens 4 Takte je Bild und `alpha`.
- Zusammenbau: `parts.rs` (`Parts::assemble`), `packages.rs` (`packages`, `Libs`: ein Eintrag je Fachmodul), `shared.rs` (`Shared`, `HudOverlay`, `HudVoyage`, `HudEnd`: ein Zustand hinter mehreren `view`-Rollen), `layout.rs` (`cells`, `Cameras::follow`, Split-Screen-Zellen), `blend.rs` (Zwischenbilder), `devices.rs` (verbundene Geräte, Neubelegung), `stack.rs` (`Mode`, `Stack`: Menü-Stapel, reine Logik), `keytable.rs`.
- Laden und Anzeige: `assets.rs`/`levels.rs` (Grafik-Ordner, `Loader`, `LibLoader`), `loading.rs`/`loadscreen.rs` (Ladeanzeige beim Stufenwechsel), `voyage.rs` (Schiffsfahrt beim Inselwechsel), `fault.rs` (Fehlerzustand), `notice.rs`, `overlay.rs`, `window.rs`, `workdir.rs` (Arbeitsordner = Paket).
- Start: `launch.rs` (`Launch`, `build_game`, `open_session`), `run.rs`, `flags.rs` (Ersatz für Go-`flag`), `scene.rs` (`Args`, `Scene`).
- Abnahme und Messung: `shots*.rs` (Aufnahme-Modus `--shots`), `script.rs` (feste Eingabe `--input`), `menu.rs`, `measure*.rs` (`--measure`, je Bild, Last-Szene), `stress.rs`/`stressreport.rs` (Stresstest aus dem Testing-Menü), `modes.rs`.
- Entwickler: `devkeys.rs` (F-Tasten), `devshot.rs` (F12), `devvalues.rs`, `devwinhost.rs` (Cheat-Fenster, `Game` als Host der Befehle), `clock.rs`.

## Flow

`run::run` liest Schalter, `launch` öffnet über `local` die Session und baut `Game` samt `Parts`. Je Takt: Eingabe lesen, Geräte beobachten, dann Menü-Stapel oder `play` (Befehle an `Sim`, `Frame` holen, Kameras und Listener fortschreiben, Ladeanzeige und Schiffsfahrt prüfen). Je Bild: Zellen und Kameras, Zwischenbilder, Drawer in Ebenen-Reihenfolge, Overlay und Meldungen.

## Integration

- Abhängigkeiten: `view` (Vertrag, am stärksten), `k3c_platform`, `k3c_sim`, `k3c_data`, `k3c_room`, `k3c_devproto`; crate-intern `art`, `actors`, `audio`, `devtools`, `devwin`, `hud`, `input`, `local`, `menu`, `world`, `still` (Aufnahme-Modus).
- Konsumenten: Einstieg des Programms `k3c-game` (Unterbefehl `play` bzw. ohne Unterbefehl); `replay` nutzt `game::measure`-Bericht.
- `imports_tests.rs` prüft die erlaubten Importe.
