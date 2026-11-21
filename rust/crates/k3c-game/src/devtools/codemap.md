# rust/crates/k3c-game/src/devtools/

## Responsibility

Entwickler-Anzeigen des nativen Spiels (B-888): Tastenübersicht (F1), Cheat-Menü je Spieler (F9), Debug-Overlay (F10),
dazu Chronik und Bericht. Nichts davon erscheint im Aufnahme-Modus.

## Design

- `mod.rs`: `DevOverlay` hinter `Rc<RefCell<..>>`, getrennte Hüllen für `Drawer` (`Layer::OVERLAY`, `&self`) und `Overlay`
  (`update`, `&mut self`); Farbkonstanten (`PANEL_COLOR`, `TEXT_COLOR`, `KEY_COLOR`, ...).
- Cheats: `cheatlist.rs` (Gruppen `CheatGroup`/`CheatEntry`: Ressourcen, Phasen, Grade, Ereignisse, Bosse, Ausbau),
  `cheatmenu.rs` (`CheatState`, `step`, `CheatResult`), `cheats.rs` (`update_with`, `actions_of`, `keyboard_seat`,
  `ERR_NO_DEV`), `drawcheats.rs` (Fenster). Hilfe: `help.rs` (`help_rows` aus `view::devkeys::DEV_KEYS`, `draw_help`).
- Debug: `debuggeo.rs` (Geometrie-Formen für Monarch, Truppe, Gegner, Münze, Platz, Chunk, Kamera), `drawdebug.rs`,
  `values.rs` (Wertezeilen, `select_at`).
- Chronik und Bericht: `chronicle.rs` (`Chronicle`, `LogSink`, `PlatformSink`, `NativeValues`), `chronicle_stage.rs`
  (Wechsel und Nachtzusammenfassungen je Stufe, Spieler, Takt), `report.rs` (`Report`, `write_report`, `Zeit`).

## Flow

`game` setzt F1/F10 im Frame (`Frame::dev`); das Cheat-Menü führt sein Overlay selbst (`Overlay::update` mit
`Input`/`Session`) und löst Dev-Aktionen über `DevSession` aus; `draw` malt je Zelle in deren Rechteck (`Frame::cells`)
über allen Ebenen. Die Chronik schreibt Ereignisse und Zusammenfassungen ins Log.

## Integration

- Abhängigkeiten: `view` (`devkeys`, `frame`, `layer`, `roles`, `session`, `input`), `art` (Schrift), `k3c-platform`.
- Konsumenten: `game` (u. a. `devshot`, `devvalues`, `devwinhost`, `stress`, `stressreport`, `packages`), `devwin`, `local`.
