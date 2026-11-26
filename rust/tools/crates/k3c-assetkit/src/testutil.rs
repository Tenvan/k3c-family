//! Hilfen für Tests (V3). Temp-Ordner liegen im Repo (Regel „Schreiben nur im Repo“), nie unter
//! `std::env::temp_dir()`.

use std::fs;
use std::path::{Path, PathBuf};

use crate::register::REGISTER_PATH;

/// Wurzel des Checkouts, vier Ebenen über dem Crate.
pub(crate) fn checkout_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .ancestors()
        .nth(4)
        .expect("Crate liegt vier Ebenen unter der Wurzel")
        .to_path_buf()
}

/// Frischer Temp-Ordner unter `K3C_TMP`, sonst `<Repo>/.work/tmp`; er wird beim Fallenlassen gelöscht.
pub(crate) fn tempdir() -> tempfile::TempDir {
    let base = std::env::var_os("K3C_TMP")
        .filter(|v| !v.is_empty())
        .map_or_else(|| checkout_root().join(".work").join("tmp"), PathBuf::from);
    fs::create_dir_all(&base).expect("Temp-Basis anlegen");
    tempfile::Builder::new()
        .prefix("k3c-assetkit-")
        .tempdir_in(base)
        .expect("Temp-Ordner anlegen")
}

/// Mini-Repo: `data/sounds.json` mit einem Grafik- und einem Sound-Slot, zwei Bilder, ein Sound.
pub(crate) fn repo() -> tempfile::TempDir {
    let dir = tempdir();
    let town = dir.path().join("public").join("grafik").join("town");
    write_png(&town.join("well.png"), 64, 48);
    write_png(&town.join("wagon.png"), 32, 32);
    write_file(&dir.path().join("public").join("audio").join("coin.ogg"), "ogg");
    write_file(
        &dir.path().join(REGISTER_PATH),
        r#"{
  "_comment": "Test",
  "slots": [
    { "id": "farm", "art": "grafik", "kategorie": "gebaeude", "beschreibung": "Farm", "wahl": [] },
    { "id": "sfx:coin", "art": "sound", "kategorie": "sfx", "beschreibung": "Münze", "bus": "sfx", "wahl": [] }
  ]
}
"#,
    );
    dir
}

fn create_parent(path: &Path) {
    fs::create_dir_all(path.parent().expect("Pfad mit Ordner")).expect("Ordner anlegen");
}

pub(crate) fn write_file(path: &Path, text: &str) {
    create_parent(path);
    fs::write(path, text).expect("Datei schreiben");
}

fn crc32(data: &[u8]) -> u32 {
    let mut crc = u32::MAX;
    for &b in data {
        crc ^= u32::from(b);
        for _ in 0..8 {
            crc = (crc >> 1) ^ (0xedb8_8320 & 0u32.wrapping_sub(crc & 1));
        }
    }
    !crc
}

fn push_chunk(out: &mut Vec<u8>, kind: &[u8; 4], data: &[u8]) {
    out.extend_from_slice(&(data.len() as u32).to_be_bytes());
    let body = [kind.as_slice(), data].concat();
    out.extend_from_slice(&body);
    out.extend_from_slice(&crc32(&body).to_be_bytes());
}

/// PNG-Kopf (Signatur, IHDR, IEND) ohne Bilddaten: Rust liest nur die Maße.
pub(crate) fn png_bytes(w: u32, h: u32) -> Vec<u8> {
    let mut out = b"\x89PNG\r\n\x1a\n".to_vec();
    let ihdr = [w.to_be_bytes().as_slice(), &h.to_be_bytes(), &[8, 6, 0, 0, 0]].concat();
    push_chunk(&mut out, b"IHDR", &ihdr);
    push_chunk(&mut out, b"IEND", &[]);
    out
}

pub(crate) fn write_png(path: &Path, w: u32, h: u32) {
    create_parent(path);
    fs::write(path, png_bytes(w, h)).expect("PNG schreiben");
}
