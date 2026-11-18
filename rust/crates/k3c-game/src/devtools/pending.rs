//! Platzhalter für Teile, die spätere Sprints liefern (Leitfaden § 7). `dev_of` löscht der Sprint, der `Session` um die
//! Entwickler-Rolle erweitert (RU30.3); danach wird diese Datei gelöscht. RU22 prüft, dass keiner übrig ist.

use crate::view::devkeys::DevSession;
use crate::view::session::Session;

/// Go: Typ-Zusicherung `s.(view.DevSession)`. `view::session::Session` hat in Rust noch keine Entwickler-Rolle;
/// bis dahin `None`: das Cheat-Menü meldet „nur im lokalen Spiel“ statt eine Aktion auszulösen (kein Absturz).
pub fn dev_of(_s: &mut dyn Session) -> Option<&mut dyn DevSession> {
    None
}
