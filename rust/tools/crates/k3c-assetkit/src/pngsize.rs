//! Maße eines PNG aus dem Kopf (Ersatz für Go `image.DecodeConfig`, V9): Signatur und IHDR.

use std::fs::File;
use std::io::Read;
use std::path::Path;

use crate::error::{Error, Result};

const SIGNATURE: &[u8; 8] = b"\x89PNG\r\n\x1a\n";

/// Breite und Höhe in Pixeln.
pub(crate) fn size(path: &Path) -> Result<(i64, i64)> {
    // Signatur (8), Länge (4), Typ (4), Breite (4), Höhe (4)
    let mut head = [0u8; 24];
    File::open(path)?.read_exact(&mut head)?;
    if &head[..8] != SIGNATURE {
        return Err(Error::Msg("image: unknown format".to_string()));
    }
    let number = |at: usize| u32::from_be_bytes([head[at], head[at + 1], head[at + 2], head[at + 3]]);
    let (width, height) = (number(16), number(20));
    let valid = |v: u32| v > 0 && v <= i32::MAX as u32;
    if number(8) != 13 || &head[12..16] != b"IHDR" || !valid(width) || !valid(height) {
        return Err(Error::Msg("png: invalid format: IHDR".to_string()));
    }
    Ok((i64::from(width), i64::from(height)))
}
