//! Fehler des Raum-Crates (Leitfaden § 3): ein `enum` je Crate, Meldungstexte wie in Go (`engine/store`).
use std::fmt;

/// Fehler von Spielständen, Berichten und Raum. `errors.Is` aus Go wird ein `match` auf die Variante.
#[derive(Debug, Clone, PartialEq)]
pub enum Fehler {
    /// `ErrSlot`: der Name des Spielstands ist ungültig.
    Slot,
    /// `ErrNotFound`: nicht gefunden.
    NotFound,
    /// `ErrInvalid`: ungültiger Inhalt.
    Invalid,
    /// `ErrTooLarge`: zu groß.
    TooLarge,
    /// Raum-Fehler mit dem Protokoll-Code aus `docs/protocol.md` › Fehler-Codes als Text (`room_full` …).
    RoomFull,
    TooManySlots,
    TooManyRooms,
    RoomNotFound,
    SaveExists,
    SaveNotFound,
    BadRequest,
    /// Go `SkillError`: von der Sim abgelehnter Skill-Befehl mit ihrem Grund; Text `bad_request: <reason>`. Zählt als
    /// `BadRequest` (`is`); der Code im Protokoll bleibt `bad_request`.
    BadRequestReason {
        reason: String,
    },
    /// Go `CandidateError`: die Kandidaten-hub.json der Vorschau ist kein JSON oder hat Befunde (Text wie in Go).
    Candidate(String),
    /// Fehler der Vorschau (Go `fmt.Errorf("vorschau: …")`), der ganze Text.
    Preview(String),
    /// `room_closed`: der Server fährt herunter.
    Closed,
    /// `forbidden`: dev ohne Dev-Mode (B-178).
    Forbidden,
    /// `save_in_use`: Spielstand in einem Raum mit verbundenem Gerät offen (B-424).
    SaveInUse,
    /// `world_unknown`: Welt unbekannt, Welt-ID oder Welt-Datei ungültig (welten.md § 5/6).
    WorldUnknown,
    /// `game_over`: Die Insel ist komplett verloren, der Raum speichert nicht mehr (B-344); kein Protokoll-Code.
    GameOver,
    /// Dateisystem-Fehler (Text des Betriebssystems, Code in `os`).
    Io(String),
    /// Gos `fmt.Errorf("%s (%w)", text, code)`: Meldung mit dem Fehlerwert `code` in Klammern (`is` ersetzt `errors.Is`).
    Meldung {
        code: Box<Fehler>,
        text: String,
    },
}

impl Fehler {
    /// Meldung `text` mit dem Fehlerwert `code` (Welt-Fehler, welten.md § 5/6).
    pub fn meldung(code: Fehler, text: impl Into<String>) -> Fehler {
        Fehler::Meldung {
            code: Box::new(code),
            text: text.into(),
        }
    }

    /// Gos `errors.Is`: der Fehler ist `code` oder trägt ihn in einer Meldung.
    pub fn is(&self, code: &Fehler) -> bool {
        match self {
            Fehler::Meldung { code: inner, .. } => inner.is(code),
            Fehler::BadRequestReason { .. } => *code == Fehler::BadRequest || self == code,
            other => other == code,
        }
    }

    /// Gos `fmt.Errorf("prefix: %w", err)`: stellt `prefix` vor den Text und behält den Fehlerwert.
    pub fn mit_vorsatz(self, prefix: &str) -> Fehler {
        match self {
            Fehler::Meldung { code, text } => Fehler::Meldung {
                code,
                text: format!("{prefix}{text}"),
            },
            other => Fehler::Io(format!("{prefix}{other}")),
        }
    }
}

impl fmt::Display for Fehler {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Fehler::Slot => f.write_str("ungültiger Slot"),
            Fehler::NotFound => f.write_str("nicht gefunden"),
            Fehler::Invalid => f.write_str("ungültiger Inhalt"),
            Fehler::TooLarge => f.write_str("zu groß"),
            Fehler::RoomFull => f.write_str("room_full"),
            Fehler::TooManySlots => f.write_str("too_many_slots"),
            Fehler::TooManyRooms => f.write_str("too_many_rooms"),
            Fehler::RoomNotFound => f.write_str("room_not_found"),
            Fehler::SaveExists => f.write_str("save_exists"),
            Fehler::SaveNotFound => f.write_str("save_not_found"),
            Fehler::BadRequest => f.write_str("bad_request"),
            Fehler::BadRequestReason { reason } => write!(f, "bad_request: {reason}"),
            Fehler::Candidate(text) | Fehler::Preview(text) => f.write_str(text),
            Fehler::Closed => f.write_str("room_closed"),
            Fehler::Forbidden => f.write_str("forbidden"),
            Fehler::SaveInUse => f.write_str("save_in_use"),
            Fehler::WorldUnknown => f.write_str("world_unknown"),
            Fehler::GameOver => f.write_str("game_over"),
            Fehler::Io(s) => f.write_str(s),
            Fehler::Meldung { code, text } => write!(f, "{text} ({code})"),
        }
    }
}

impl std::error::Error for Fehler {}

impl From<std::io::Error> for Fehler {
    fn from(e: std::io::Error) -> Self {
        Fehler::Io(e.to_string())
    }
}
