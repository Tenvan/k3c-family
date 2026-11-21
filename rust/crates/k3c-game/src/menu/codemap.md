# rust/crates/k3c-game/src/menu/

## Responsibility

Die Menü-Bildschirme des nativen Spiels (Port von `native/k3c-game/menu/`): Hauptmenü, Weltauswahl, Lobby für 1 bis 2 Spieler, Spielstände, Optionen mit Steuerung, Lizenzen, Pause, Tastenbelegung, Tasten-Test und Testing (Stresstest) auf Papier-Hintergrund.

## Design

- Jeder Bildschirm ist ein `view::screen::Screen` (`MainScreen`, `WorldPickScreen`, `LobbyScreen`, `SavesScreen`, `OptionsScreen`, `ControlsScreen`, `PauseScreen`, `TableScreen`, `PadTestScreen`, `TestingScreen`). Untermenüs über `StepKind::Push`, zurück über `Pop`; Stapeltiefe höchstens drei, Rückfragen liegen im Bildschirm.
- Gemeinsames: `keys.rs` (`Key`, `Base`, `Note`), `draw.rs` (`Page`, `Row`, `Hero`, Zeichner für Karten, Papier, Rückfrage), `layout.rs` (`Geom`, `row_rect`), `backdrop.rs` (Papier-Hintergrund), `hero.rs`, `art.rs` (Karten-Illustrationen, `world_asset`), `table.rs`.
- Texte: `texts.rs` (`TextKey`, `tr`, `tr1`) mit Daten in `texts_data.rs` (Deutsch, Englisch), `testing_text.rs`.
- Daten: `stress_presets.rs` (aus `data/`, `load_stress_presets`), `licenses.rs`, `version.rs`, `padtest_read.rs` (`PadSource`, `InputPads`), `placeholder.rs` (`PlaceholderSession` für Tests und Vorschau).

## Flow

`game::stack` hält den Menü-Stapel und ruft je Takt `update` des obersten `Screen` mit der `view::input::Input`. Der Bildschirm liest und ändert Spielstand, Einstellungen und Welten über `view::session::Session` und gibt einen `Step` (Push, Pop, Spiel starten, Beenden) zurück; gezeichnet wird auf `Layer::OVERLAY`-Ebene der Zelle.

## Integration

- Abhängigkeiten: `view` (Vertrag, dominant), `k3c_platform`, `k3c_data`, `art`, `still` (Bild-Tests).
- Konsumenten: `game` (`menu.rs`, `stack`, `parts`, `stress`) über `view::screen::Menus`/`MenuPages`; die Sitzung kommt aus `local`.
