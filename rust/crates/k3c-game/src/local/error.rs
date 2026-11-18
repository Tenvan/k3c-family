//! Fehler der Session (Go: `userError`, `userErr`, `errNoGame`, `describe` aus `session.go`).

use std::fmt::{self, Display};

use k3c_room::fehler::Fehler as RoomFehler;

use crate::view::session::Fehler;

/// Ein anzeigbarer Fehler (deutsch); `source` ist der Fehler des Raums oder Stores (Go `Unwrap`).
#[derive(Debug)]
pub struct UserError {
    pub msg: String,
    pub cause: Option<Box<dyn std::error::Error>>,
}

impl Display for UserError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.msg)
    }
}

impl std::error::Error for UserError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        self.cause.as_deref()
    }
}

impl From<UserError> for Fehler {
    fn from(e: UserError) -> Fehler {
        Fehler::Session(e.msg)
    }
}

/// Go `userErr`.
pub fn user_err(msg: impl Into<String>, cause: Option<Box<dyn std::error::Error>>) -> UserError {
    UserError { msg: msg.into(), cause }
}

/// Aufruf ohne laufendes Spiel (Go `errNoGame`).
pub fn err_no_game() -> UserError {
    user_err("Es läuft kein Spiel", None)
}

/// Übersetzt Fehler von Raum und Store in eine Meldung für das Menü (Go `describe`).
pub fn describe(err: &RoomFehler, name: &str) -> UserError {
    let msg = if err.is(&RoomFehler::SaveExists) {
        format!("Spielstand „{name}“ gibt es schon")
    } else if err.is(&RoomFehler::SaveNotFound) || err.is(&RoomFehler::NotFound) {
        format!("Spielstand „{name}“ gibt es nicht")
    } else if err.is(&RoomFehler::SaveInUse) {
        format!("Spielstand „{name}“ ist gerade offen")
    } else if err.is(&RoomFehler::BadRequest) || err.is(&RoomFehler::Slot) {
        "Ungültiger Name oder ungültige Optionen (Name: a–z, 0–9, -, höchstens 32 Zeichen)".to_string()
    } else if err.is(&RoomFehler::RoomFull) {
        "Kein Monarch mehr frei".to_string()
    } else if err.is(&RoomFehler::GameOver) {
        "Insel verloren, der Spielstand bleibt wie zuletzt gesichert".to_string()
    } else {
        format!("Spielstand „{name}“: {err}")
    };
    user_err(msg, Some(Box::new(err.clone())))
}
