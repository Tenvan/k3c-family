//! Datei-Import: eine (manuell geladene) Datei zu einem Recherche-Treffer ablegen (Go `import.go`; der Treffer-Download
//! steht in `import_download.rs`).

use std::fs::{self, File};
use std::io::{self, Read, Write};
use std::path::Path;

use chrono::{DateTime, FixedOffset};

use crate::credits::{add_credit, mark_imported};
use crate::error::{Error, Result};
use crate::fsutil::{self, TempFile};
use crate::register::{Choice, KIND_GRAPHIC, KIND_SOUND, Slot, load, set_choices};
use crate::research::{Hit, LICENSE_EXCLUDED, hits_of};
use crate::vorrat_adopt::append_choice;

/// Begrenzt Import und Download.
pub(crate) const MAX_IMPORT_BYTES: usize = 20 << 20;

/// Ablage unter `public/` je Art (B-165).
pub(crate) fn import_dir(kind: &str) -> &'static str {
    match kind {
        KIND_GRAPHIC => "grafik/kandidaten",
        KIND_SOUND => "audio/kandidaten",
        _ => "",
    }
}

/// Erlaubte Endungen je Art.
pub(crate) fn import_ext(kind: &str) -> &'static [&'static str] {
    match kind {
        KIND_GRAPHIC => &[".png"],
        KIND_SOUND => &[".ogg", ".mp3", ".wav"],
        _ => &[],
    }
}

/// Legt eine Datei zu einem Recherche-Treffer ab: Ablage, Credit-Zeile, Zuordnung als letzte Präferenz. Nicht
/// übernehmbare Lizenzen (Q70) werden abgelehnt; die Lizenz bleibt „prüfen“, bis 🧑 bestätigt.
pub fn import_file(root: &Path, slot_id: &str, hit_id: &str, src: &Path, now: DateTime<FixedOffset>) -> Result<String> {
    import_file_on(root, slot_id, hit_id, src, None, now)
}

/// [`import_file`]; mit `stand` hängt die Zuordnung nur auf diesem Stand des Registers an (sonst [`Error::Stale`], die
/// Datei samt Credit bleibt dann als Katalog-Eintrag liegen).
pub(crate) fn import_file_on(
    root: &Path,
    slot_id: &str,
    hit_id: &str,
    src: &Path,
    stand: Option<&str>,
    now: DateTime<FixedOffset>,
) -> Result<String> {
    let (slot, hit) = import_target(root, slot_id, hit_id)?;
    let (data, ext) = read_import(src, &slot.kind)?;
    let rel = store_import(root, &slot.kind, &hit.title, &ext, &data)?;
    add_credit(root, &slot.kind, &rel, &hit, now)?;
    mark_imported(root, slot_id, hit_id, &rel)?;
    if let Some(stand) = stand {
        append_choice(root, slot_id, &rel, Some(stand))?;
        return Ok(rel);
    }
    let register = load(root)?;
    let mut choices = register
        .find(slot_id)
        .ok_or_else(|| unknown_slot(slot_id))?
        .choices()
        .to_vec();
    choices.push(Choice {
        file: rel.clone(),
        ..Choice::default()
    });
    set_choices(root, slot_id, &choices)?;
    Ok(rel)
}

pub(crate) fn unknown_slot(slot_id: &str) -> Error {
    Error::Msg(format!("unbekannter Asset-Slot {slot_id:?}"))
}

pub(crate) fn import_target(root: &Path, slot_id: &str, hit_id: &str) -> Result<(Slot, Hit)> {
    let register = load(root)?;
    let slot = register.find(slot_id).ok_or_else(|| unknown_slot(slot_id))?;
    let Some(hit) = hits_of(root, slot_id)?.into_iter().find(|h| h.id == hit_id) else {
        return Err(Error::Msg(format!("treffer {hit_id:?} zu {slot_id} nicht gefunden")));
    };
    if hit.license_status == LICENSE_EXCLUDED {
        let license = &hit.license;
        return Err(Error::Msg(format!(
            "treffer {hit_id} ist nicht übernehmbar (Q70: {license})"
        )));
    }
    Ok((slot.clone(), hit))
}

/// Liest die Quelldatei: erlaubte Endung je Art, höchstens 20 MB, Signatur passt zur Endung. Liefert Inhalt und Endung.
fn read_import(src: &Path, kind: &str) -> Result<(Vec<u8>, String)> {
    let name = src
        .file_name()
        .map_or_else(String::new, |n| n.to_string_lossy().into_owned());
    let ext = fsutil::path_ext(&name).to_lowercase();
    if !import_ext(kind).contains(&ext.as_str()) {
        let allowed = import_ext(kind).join(", ");
        return Err(Error::Msg(format!("{name}: erlaubt sind {allowed}")));
    }
    let mut data = Vec::new();
    File::open(src)?
        .take(MAX_IMPORT_BYTES as u64 + 1)
        .read_to_end(&mut data)?;
    check_signature(&data, &ext)?;
    Ok((data, ext))
}

/// Größe im Limit und Dateikopf passend zur Endung.
pub(crate) fn check_signature(data: &[u8], ext: &str) -> Result<()> {
    if data.len() > MAX_IMPORT_BYTES {
        return Err(Error::Msg(format!("datei größer als {} MB", MAX_IMPORT_BYTES >> 20)));
    }
    let ok = match ext {
        ".png" => data.starts_with(b"\x89PNG\r\n\x1a\n"),
        ".ogg" => data.starts_with(b"OggS"),
        ".wav" => data.starts_with(b"RIFF") && data.len() > 12 && &data[8..12] == b"WAVE",
        ".mp3" => data.starts_with(b"ID3") || (data.len() > 1 && data[0] == 0xFF && data[1] & 0xE0 == 0xE0),
        _ => false,
    };
    if !ok {
        let kind = ext.strip_prefix('.').unwrap_or(ext);
        return Err(Error::Msg(format!("inhalt ist kein {kind}")));
    }
    Ok(())
}

/// Schreibt unter `public/<Ablage>/` mit Namen aus dem Titel; vorhandene Dateien bleiben (Suffix `-2` …). Atomar
/// (B-511): erst eine Temp-Datei im selben Ordner, dann ein harter Link auf den Zielnamen.
pub(crate) fn store_import(root: &Path, kind: &str, title: &str, ext: &str, data: &[u8]) -> Result<String> {
    let dir = fsutil::join_below(&root.join("public"), import_dir(kind));
    fs::create_dir_all(&dir)?;
    let tmp = write_temp(&dir, data)?;
    let base = slug(title);
    for n in 1..100 {
        let name = match n {
            1 => format!("{base}{ext}"),
            _ => format!("{base}-{n}{ext}"),
        };
        match publish(tmp.path(), &dir.join(&name)) {
            Ok(()) => return Ok(format!("{}/{name}", import_dir(kind))),
            Err(e) if e.kind() == io::ErrorKind::AlreadyExists => {}
            Err(e) => return Err(Error::Io(e)),
        }
    }
    Err(Error::Msg(format!("kein freier Dateiname für {base:?}")))
}

/// Schreibt `data` in eine neue Temp-Datei in `dir` (mit Sync).
fn write_temp(dir: &Path, data: &[u8]) -> Result<TempFile> {
    let (file, path) = fsutil::create_temp(dir, ".import-")?;
    // Reihenfolge: `file` schließt vor dem Löschen durch `tmp`.
    let tmp = TempFile::new(path);
    let mut file = file;
    file.write_all(data)?;
    file.sync_all()?;
    drop(file);
    fsutil::make_readable(tmp.path());
    Ok(tmp)
}

/// Macht aus der Temp-Datei `tmp` die Datei `dst`, ohne eine vorhandene zu ersetzen (`AlreadyExists`). Ohne
/// Link-Unterstützung des Dateisystems: Umbenennen nach einer Prüfung (dann ist ein Rennen möglich, nie eine halbe
/// Datei).
fn publish(tmp: &Path, dst: &Path) -> io::Result<()> {
    match fs::hard_link(tmp, dst) {
        Ok(()) => Ok(()),
        Err(e) if e.kind() == io::ErrorKind::AlreadyExists => Err(e),
        Err(_) if fs::metadata(dst).is_ok() => Err(io::ErrorKind::AlreadyExists.into()),
        Err(_) => fs::rename(tmp, dst),
    }
}

/// Dateiname aus a–z, 0–9 und `-` aus einem Titel, ohne Pfadanteile.
pub(crate) fn slug(title: &str) -> String {
    // Wie Go `strings.ToLower` Zeichen für Zeichen (`İ` wird `i`, ohne Kombinationspunkt).
    let lower: String = title.chars().map(|c| c.to_lowercase().next().unwrap_or(c)).collect();
    let replaced = lower
        .replace('ä', "ae")
        .replace('ö', "oe")
        .replace('ü', "ue")
        .replace('ß', "ss");
    let mut out = String::new();
    for c in replaced.chars() {
        if c.is_ascii_lowercase() || c.is_ascii_digit() {
            out.push(c);
        } else if !out.is_empty() && !out.ends_with('-') {
            out.push('-');
        }
    }
    let mut s = out.trim_matches('-');
    if s.len() > 40 {
        s = s[..40].trim_matches('-');
    }
    match s {
        "" => "import".to_string(),
        name => name.to_string(),
    }
}

#[cfg(test)]
#[path = "import_tests.rs"]
mod tests;
