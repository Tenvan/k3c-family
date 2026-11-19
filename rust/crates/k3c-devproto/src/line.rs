//! Zeilen-JSON: eine Nachricht je Zeile.

use crate::error::ProtoError;
use crate::msg::{Msg, PROTO_VERSION};

/// MAX_LINE ist die größte erlaubte Zeilenlänge in Bytes (ohne Zeilenende).
pub const MAX_LINE: usize = 64 * 1024;

/// encode_line kodiert die Nachricht als eine Zeile mit abschließendem `\n`.
pub fn encode_line(msg: &Msg) -> Result<String, ProtoError> {
    let mut s = serde_json::to_string(msg).map_err(|e| ProtoError::Json(e.to_string()))?;
    if s.len() > MAX_LINE {
        return Err(ProtoError::TooLong(s.len()));
    }
    s.push('\n');
    Ok(s)
}

/// decode_line liest eine Zeile (mit oder ohne `\n`/`\r\n`). Eine unbekannte Art `t` ergibt `Msg::Unknown`,
/// ein `hello` mit fremder Hauptversion `ProtoError::Version`.
pub fn decode_line(line: &str) -> Result<Msg, ProtoError> {
    let line = line.trim_end_matches(['\n', '\r']);
    if line.len() > MAX_LINE {
        return Err(ProtoError::TooLong(line.len()));
    }
    let msg: Msg = serde_json::from_str(line).map_err(|e| ProtoError::Json(e.to_string()))?;
    if let Msg::Hello { v, .. } = &msg
        && *v != PROTO_VERSION
    {
        return Err(ProtoError::Version(*v));
    }
    Ok(msg)
}
