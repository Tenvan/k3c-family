# rust/crates/k3c-game/src/input/

## Responsibility

Liest Tastatur und Gamepads und liefert `view::input::Input` je Takt (Port von `native/k3c-game/input/`): Tastatur ist ein Gerät (keys1) für einen Spieler, Belegung nach Tastatur-Profil der Einstellungen.

## Design

- `bindings.rs`: Belegung: `KeyLayout`, `LAYOUT_WASD`/`LAYOUT_ARROWS`, `layout`, Pad-Tasten-Konstanten (`PAD_A` … `PAD_MENU`), `action_keys`, `command_keys`, `ALL_ACTIONS`.
- `source.rs`: `InputSource` (implementiert die `view`-Quelle) mit `PadSlot` je Gamepad und `poll_raw`.
- `tracker.rs`: `Tracker` und `PadState`: Stick zu Bewegung und Navigation mit Schwellen (`MOVE_ON/OFF`, `NAV_ON/OFF`), Wiederholung (`REPEAT_DELAY`, `REPEAT_EVERY`), kurzer Menü-Druck (`MENU_SHORT`), `home_chord`, `key_command`, `pad_command`.

## Flow

`InputSource::poll_raw` fragt `k3c_platform` (Tasten, Pads) ab, `Tracker::step` leitet daraus gehaltene und neu gedrückte Befehle und Menü-Aktionen ab (je Gerät), das Ergebnis ist `Input` mit `DeviceInput` je `DeviceID`.

## Integration

- Abhängigkeiten: `view` (Input, Command, Action, Profile), `k3c_platform`.
- Konsumenten: `game` (Schleife, `devices`, `keytable`), `menu` (`padtest_read`: `InputPads`).
