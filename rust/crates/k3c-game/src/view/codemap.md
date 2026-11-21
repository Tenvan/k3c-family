# rust/crates/k3c-game/src/view/

## Responsibility

Der eingefrorene Vertrag zwischen Spielzustand und Darstellung (Port von `native/k3c-game/view`): Frame, Zellen und Kameras, Ebenen, Eingabe-Typen, Einstellungen, Rollen-Traits und die Session-Schnittstelle der Menüs. Braucht ein Fachpaket mehr, gibt es ein Ticket statt eines Eingriffs.

## Design

- Zustand: `frame.rs` (`Frame`, `Stage`, `Seat`, `Outcome`, `Event`, `Cue`, `RouteIsland`), `story.rs` (`Campaign`, `EndHolder`), `voyage.rs` (`Voyage`, `VoyageDrawer`), `crown.rs`, `seal.rs` (Siegel je Schwierigkeitsgrad, gemeinsam für Menü und HUD).
- Geometrie und Ebenen: `cell.rs` (`Point`, `Rectangle`, `Camera`, `screen_x`, `parallax_x`), `layer.rs` (`Layer`: FAR, MID, NEAR, GROUND, BUILDINGS, ACTORS, RIVER, EFFECTS, HUD, OVERLAY; Drawer).
- Eingabe: `input.rs` (`DeviceKind`, `DeviceID`, `Command`, `Action`, `Input`, Traits `Source`, `KeyboardSource`), `keys.rs`, `profile.rs` (Tastatur-Profile, `default_controls`).
- Rollen: `roles.rs` (`CameraRule`, `AssetSource`, `O
