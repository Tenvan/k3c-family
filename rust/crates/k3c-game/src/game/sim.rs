//! Port von `Sim` aus `native/k3c-game/game/game.go`: die Simulation aus Sicht der Schleife.

use crate::view::devkeys::DevSession;
use crate::view::frame::Frame;
use crate::view::input::Command;
use crate::view::session::{Fehler, Session};

/// Die Session der Menüs plus Takt und Befehle (`local`, RU27, erfüllt sie).
pub trait Sim: Session {
    /// Befehl des Spielers `player` für den nächsten Takt.
    fn command(&mut self, player: i64, c: Command) -> Result<(), Fehler>;

    /// Rechnet einen Takt und liefert den Frame.
    fn step(&mut self) -> Frame;

    /// Go: Typ-Zusicherung `g.sim.(view.DevSession)`; `None`, wenn die Sitzung keine Entwickler-Rolle hat (`local`
    /// überschreibt es).
    fn dev_session(&mut self) -> Option<&mut dyn DevSession> {
        None
    }
}
