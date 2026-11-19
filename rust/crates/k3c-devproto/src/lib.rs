//! Protokoll zwischen dem Rust-Spiel und dem Cheat-Fenster `k3c-devwin` (RU56, B-1131).
//!
//! Transport ist eine Named Pipe als Byte-Strom, eine JSON-Nachricht je Zeile (UTF-8, `\n`, höchstens
//! [`MAX_LINE`] Bytes). Das Crate kennt nur Zeilen (`encode_line`, `decode_line`), nicht die Pipe. Es hat weder
//! `windows` noch Netz noch `unsafe`.

mod error;
mod line;
mod msg;

pub use error::ProtoError;
pub use line::{MAX_LINE, decode_line, encode_line};
pub use msg::{Catalog, DevCommand, Entry, GameInfo, IslandInfo, Msg, PROTO_VERSION, PlayerState, State, WorldInfo};
