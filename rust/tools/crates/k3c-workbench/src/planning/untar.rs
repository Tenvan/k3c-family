//! Entpackt das tar aus `git archive` (Go nimmt `archive/tar`; hier ohne neue Abhängigkeit): ustar mit den
//! pax-Köpfen, die git für lange Pfade und den Commit schreibt. Nur Dateien und Ordner, nie aus dem Ziel hinaus.

use std::fs;
use std::path::{Path, PathBuf};

const BLOCK: usize = 512;

/// Feld eines Kopfs bis zum ersten NUL.
fn field(header: &[u8], from: usize, len: usize) -> &[u8] {
    let bytes = header.get(from..from + len).unwrap_or_default();
    bytes.split(|&b| b == 0).next().unwrap_or_default()
}

fn text(header: &[u8], from: usize, len: usize) -> String {
    String::from_utf8_lossy(field(header, from, len)).into_owned()
}

fn octal(header: &[u8], from: usize, len: usize) -> Result<usize, String> {
    let digits = text(header, from, len);
    usize::from_str_radix(digits.trim(), 8).map_err(|_| "ungültiger Kopf im Archiv".to_owned())
}

/// `path=` aus den Datensätzen `<Länge> <Name>=<Wert>\n` eines pax-Kopfs.
fn pax_path(mut records: &[u8]) -> Option<String> {
    while !records.is_empty() {
        let space = records.iter().position(|&b| b == b' ')?;
        let len: usize = std::str::from_utf8(&records[..space]).ok()?.parse().ok()?;
        let record = records.get(space + 1..len.checked_sub(1)?)?;
        if let Some(path) = record.strip_prefix(b"path=") {
            return Some(String::from_utf8_lossy(path).into_owned());
        }
        records = records.get(len..)?;
    }
    None
}

/// Ziel unter `dir`; Teile, die hinausführen könnten, sind ein Fehler.
fn target(dir: &Path, name: &str) -> Result<PathBuf, String> {
    let mut path = dir.to_owned();
    for part in name.split(['/', '\\']).filter(|p| !p.is_empty() && *p != ".") {
        if part == ".." || part.contains(':') {
            return Err(format!("unzulässiger Pfad im Archiv: {name}"));
        }
        path.push(part);
    }
    Ok(path)
}

fn write_entry(path: &Path, is_dir: bool, data: &[u8]) -> Result<(), String> {
    if is_dir {
        return fs::create_dir_all(path).map_err(|e| e.to_string());
    }
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent).map_err(|e| e.to_string())?;
    }
    fs::write(path, data).map_err(|e| e.to_string())
}

pub(super) fn untar(mut archive: &[u8], dir: &Path) -> Result<(), String> {
    let truncated = || "Archiv abgeschnitten".to_owned();
    let mut long_name = None;
    while !archive.is_empty() {
        let header = archive.get(..BLOCK).ok_or_else(truncated)?;
        if header.iter().all(|&b| b == 0) {
            break;
        }
        let size = octal(header, 124, 12)?;
        let end = size.checked_add(BLOCK).ok_or_else(truncated)?;
        let data = archive.get(BLOCK..end).ok_or_else(truncated)?;
        let kind = header.get(156).copied().unwrap_or_default();
        match kind {
            b'x' => long_name = pax_path(data),
            b'g' => {}
            _ => {
                let name = long_name.take().unwrap_or_else(|| {
                    let (prefix, name) = (text(header, 345, 155), text(header, 0, 100));
                    if prefix.is_empty() {
                        name
                    } else {
                        format!("{prefix}/{name}")
                    }
                });
                if matches!(kind, 0 | b'0' | b'5') {
                    write_entry(&target(dir, &name)?, kind == b'5', data)?;
                }
            }
        }
        let next = end.div_ceil(BLOCK) * BLOCK;
        archive = archive.get(next..).unwrap_or_default();
    }
    Ok(())
}
