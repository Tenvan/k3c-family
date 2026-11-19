//! Fehler des Protokolls.

use std::fmt;

/// ProtoError beschreibt, warum eine Zeile nicht kodiert oder dekodiert werden konnte.
#[derive(Debug)]
pub enum ProtoError {
    /// Die Zeile ist länger als `MAX_LINE`.
    TooLong(usize),
    /// Die Zeile ist kein gültiges JSON oder passt nicht zur Nachricht.
    Json(String),
    /// Die Gegenseite spricht eine andere Hauptversion.
    Version(u32),
}

impl fmt::Display for ProtoError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ProtoError::TooLong(n) => write!(f, "Zeile zu lang ({n} Bytes)"),
            ProtoError::Json(e) => write!(f, "ungültige Nachricht: {e}"),
            ProtoError::Version(v) => write!(f, "Protokollversion {v} wird nicht unterstützt"),
        }
    }
}

impl std::error::Error for ProtoError {}
