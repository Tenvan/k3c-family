//! Schreibschutz des Balancings (WZ63.7, Beschluss Ralf 2026-11-22): Quelldateien nur Welten-Dateien unter
//! `data/worlds/`, Laufausgaben (Berichte, Baselines, `simtest.md`, `ratings.json`, `result.json`, Auswertung) nur
//! unter `reports/`. Alles andere lehnt der Wächter ab. Jede Schreibstelle in `balance` und `simrun` geht über die
//! Helfer unten.
//!
//! In RU70.15 aus RU70.16 vorgezogen (V11), weil die Baselines über den Wächter schreiben.

use std::fs;
use std::io;
use std::path::{Path, PathBuf};

use k3c_kit::mcp::WriteGuard;

/// Der Wächter aller Schreibzugriffe des Balancings.
pub fn guard() -> WriteGuard {
    WriteGuard {
        name: "Balancing".to_owned(),
        allow: vec!["data/worlds".to_owned(), "reports".to_owned()],
        hint: "Code oder andere Daten nur nach Plan im Chat und Freigabe von Ralf ändern".to_owned(),
    }
}

/// Macht einen relativen Pfad zu einem unter `root` und prüft ihn, damit Wächter und Schreibaufruf dieselbe Datei
/// meinen. Eine Ablehnung ist `PermissionDenied` mit dem Text des Wächters.
fn checked(root: &Path, name: &Path) -> io::Result<PathBuf> {
    let full = if name.as_os_str().is_empty() || name.is_absolute() {
        name.to_path_buf()
    } else {
        root.join(name)
    };
    let deny = |msg: String| io::Error::new(io::ErrorKind::PermissionDenied, msg);
    let text = full
        .to_str()
        .ok_or_else(|| deny(format!("Balancing: Pfad {} ist kein Text", full.display())))?;
    guard().check(root, text).map_err(deny)?;
    Ok(full)
}

/// Schreibt `name` wie Go `os.WriteFile`, aber über Temp-Datei und Umbenennen (nie eine halbe Datei).
pub fn write_file(root: &Path, name: impl AsRef<Path>, data: &[u8]) -> io::Result<()> {
    let path = checked(root, name.as_ref())?;
    k3c_assetkit::write_atomic(&path, data).map_err(|e| io::Error::other(e.to_string()))
}

/// Legt `dir` samt Eltern an.
pub fn mkdir_all(root: &Path, dir: impl AsRef<Path>) -> io::Result<()> {
    fs::create_dir_all(checked(root, dir.as_ref())?)
}

/// Legt `dir` an; scheitert mit `AlreadyExists`, wenn es ihn gibt.
pub fn mkdir(root: &Path, dir: impl AsRef<Path>) -> io::Result<()> {
    fs::create_dir(checked(root, dir.as_ref())?)
}

/// Benennt `from` in `to` um; beide Pfade müssen erlaubt sein.
pub fn rename(root: &Path, from: impl AsRef<Path>, to: impl AsRef<Path>) -> io::Result<()> {
    let (from, to) = (checked(root, from.as_ref())?, checked(root, to.as_ref())?);
    fs::rename(from, to)
}

/// Löscht eine Datei oder einen leeren Ordner.
pub fn remove(root: &Path, name: impl AsRef<Path>) -> io::Result<()> {
    let path = checked(root, name.as_ref())?;
    if fs::symlink_metadata(&path)?.is_dir() {
        fs::remove_dir(path)
    } else {
        fs::remove_file(path)
    }
}

/// Löscht `dir` samt Inhalt; fehlt er, ist nichts zu tun. Links darin verschwinden selbst, ihr Ziel bleibt
/// (`std::fs::remove_dir_all` folgt ihnen nicht); ein Link auf dem Weg zu `dir` wird abgelehnt.
pub fn remove_all(root: &Path, dir: impl AsRef<Path>) -> io::Result<()> {
    let path = checked(root, dir.as_ref())?;
    match fs::symlink_metadata(&path) {
        Err(e) if e.kind() == io::ErrorKind::NotFound => Ok(()),
        Err(e) => Err(e),
        Ok(meta) if meta.is_dir() => fs::remove_dir_all(path),
        Ok(_) => fs::remove_file(path),
    }
}

#[cfg(test)]
#[path = "writeguard_tests.rs"]
mod tests;
