//! Port von `native/k3c-game/input/`: liest Tastatur und Gamepads und liefert `view::input::Input` (Sprint NG5, B-798).
//! Die Tastatur ist ein Gerät (keys1) für einen Spieler; ihre Belegung folgt dem Tastatur-Profil der Einstellungen
//! (B-893, view/profile, `set_keyboard`). Belegung je Profil:
//!
//! ```text
//! Befehl (gehalten)  wasd          pfeile                     Gamepad (Xbox)
//! laufen             A / D         ← / →                      linker Stick oder Steuerkreuz ←/→
//! Galopp (Sprint)    Shift links   Shift rechts               RT
//! zahlen             Leertaste     Enter, Num-Enter, Leertaste  A   (B-917: die Leertaste löst in jedem Profil aus)
//! Schlag             E             Strg rechts, Num 0         X
//! Skill 1 … 4        Q, R, T, Z    Einfg, Entf, Ende, Bild ↓  LB, RB, LT, Steuerkreuz ↑ (der niedrigste gewinnt)
//!                                  oder Num 1 … Num 4
//!
//! Aktion (Flanke)    wasd          pfeile                     Gamepad
//! ↑ ↓ ← →            W S A D       Pfeile                     Steuerkreuz oder Stick (Wiederholung beim Halten)
//! Bestätigen         Leertaste, Enter, Num-Enter  Enter, Num-Enter           A
//! Zurück             Esc           Esc, Rücktaste             B (im Spiel unbelegt, game wertet sie dort nicht aus)
//! Menü               Esc           Esc, Rücktaste             Menu kurz (beim Loslassen, < 600 ms, ohne View)
//! View               Tab           Tab                        View (beim Loslassen, ohne Menu)
//! Seite zurück/vor   Q / E         Bild ↑ / Bild ↓            LB / RB
//! Home               Pos1          Pos1                       View + Menu gemeinsam (statt Menu und View)
//! Skill-Menü         K             ↓, Num 5                   Steuerkreuz ↓ (zusätzlich zu ↓, nicht der Stick)
//! Vollbild, Cheats   F11, F9       F11, F9                    rechter Stick gedrückt, View + Y
//! ```
//!
//! input unterscheidet Spiel und Menü nicht: Steuerkreuz ↓ (und ↓ im Profil pfeile) liefert `Action::Down` und
//! `Action::SkillMenu` zugleich; das Skill-Menü (Overlay) wertet `Action::SkillMenu` nur im Spiel aus, Menüs nehmen
//! `Action::Down`. In jedem Menü gelten zusätzlich `MENU_KEYS` (Pfeile, Enter, Leertaste, Esc) unabhängig vom Profil
//! (`set_menu`, B-945). Die übrigen Entwickler-Tasten F1 … F12 (view/devkeys) gelten unabhängig vom Profil.
//!
//! Pads ohne Standard-Belegung werden mit einer Meldung im Log ignoriert (das Plattform-Pad hat immer Standard-Belegung,
//! der Zweig greift in Rust nie). Die Auswertung ist rein (Rohzustand → `DeviceInput`, tracker), nur source liest den
//! Plattform-Schnappschuss.

pub mod bindings;
#[cfg(test)]
#[path = "profile_tests.rs"]
mod profile_tests;
pub mod source;
#[cfg(test)]
mod test_util;
pub mod tracker;

pub use bindings::{
    ACTION_COUNT, LAYOUT_ARROWS, LAYOUT_WASD, LAYOUTS, MENU_KEYS, USED_KEYS, action_keys, command_keys, layout,
};
pub use source::{InputSource, new};
pub use tracker::{PadState, home_chord};
