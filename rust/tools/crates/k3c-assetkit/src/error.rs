//! Fehlertyp des Crates (V4).

use std::fmt;
use std::io;

/// Fehler der Asset-Logik; die Texte folgen Go.
#[derive(Debug)]
pub enum Error {
    /// Speichern auf veraltetem Stand (Go `ErrStale`); der Text geht so an die Oberfläche (B-459).
    Stale,
    /// Fehler des Dateisystems; die Art (z. B. `NotFound`) bleibt unterscheidbar.
    Io(io::Error),
    /// Meldung mit dem Go-Text.
    Msg(String),
}

/// Ergebnis mit [`Error`].
pub type Result<T> = std::result::Result<T, Error>;

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::Stale => f.write_str("Register geändert, bitte erneut speichern"),
            Error::Io(e) => e.fmt(f),
            Error::Msg(s) => f.write_str(s),
        }
    }
}

impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Error::Io(e) => Some(e),
            _ => None,
        }
    }
}

impl From<io::Error> for Error {
    fn from(e: io::Error) -> Self {
        Error::Io(e)
    }
}
