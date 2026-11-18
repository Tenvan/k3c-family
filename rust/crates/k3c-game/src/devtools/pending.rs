//! Platzhalter für Teile, die spätere Sprints liefern (Leitfaden § 7). RU29.3 liefert `drawcheats.go`, `drawdebug.go`
//! und `help.go` und löscht `draw_help`, `draw_debug` und `draw_cheats`; `dev_of` löscht der Sprint, der `Session` um
//! die Entwickler-Rolle erweitert (RU30.3). Ist alles geliefert, wird diese Datei gelöscht. RU22 prüft, dass keiner
//! übrig ist.
#![allow(clippy::unimplemented)] // Platzhalter nach Leitfaden § 7: wartet auf RU29.3

use super::cheatmenu::CheatState;
use crate::art::Library;
use crate::view::cell::Cell;
use crate::view::devkeys::DevSession;
use crate::view::frame::Frame;
use crate::view::session::Session;
use k3c_platform::Canvas;

/// Go: `drawHelp` (`help.go`, RU29.3): Tastenübersicht (F1).
pub fn draw_help(_dst: &mut dyn Canvas, _lib: Option<&Library>, _c: &Cell) {
    unimplemented!("RU29: draw_help")
}

/// Go: `(*overlay).drawDebug` (`drawdebug.go`, RU29.3): Debug-Overlay (F10) einer Zelle.
pub fn draw_debug(_dst: &mut dyn Canvas, _lib: Option<&Library>, _f: &Frame, _cell: &Cell) {
    unimplemented!("RU29: draw_debug")
}

/// Go: `(*overlay).drawCheats` (`drawcheats.go`, RU29.3): Cheat-Menü eines Spielers in seiner Zelle.
pub fn draw_cheats(_dst: &mut dyn Canvas, _lib: Option<&Library>, _cell: &Cell, _m: &CheatState) {
    unimplemented!("RU29: draw_cheats")
}

/// Go: Typ-Zusicherung `s.(view.DevSession)`. `view::session::Session` hat in Rust noch keine Entwickler-Rolle;
/// bis dahin `None`: das Cheat-Menü meldet „nur im lokalen Spiel“ statt eine Aktion auszulösen (kein Absturz).
pub fn dev_of(_s: &mut dyn Session) -> Option<&mut dyn DevSession> {
    None
}
