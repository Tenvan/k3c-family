//! Vertrag zwischen Spielzustand und Darstellung des nativen Spiels (Port von `native/k3c-game/view`, B-792).
//!
//! Rollen:
//!   - game (Schleife) besitzt Fenster, Zellen, Kameras, Menü-Stapel, Geräte-Zuordnung, Grafik der Level und die
//!     Simulation (local); nur game kennt alle Pakete.
//!   - local erfüllt Session (Spiel, Spielstände, Skills, Geräte der Spieler, Einstellungen) und liefert je Update
//!     einen Frame.
//!   - Fachpakete (world, actors, props, input, hud, fx, menu, audio) liefern Drawer, Listener, Source oder Menus.
//!
//! RU4.5 portiert die Eingabe-Verträge (input, profile, settings, devkeys); die übrigen Dateien und den vollen
//! Doc-Kommentar aus `view/doc.go` bringt RU7.

pub mod devkeys;
pub mod input;
pub mod profile;
pub mod settings;

/// MaxPlayers ist die Höchstzahl lokaler Spieler (Zellen) im Split-Screen.
pub const MAX_PLAYERS: usize = 2;
