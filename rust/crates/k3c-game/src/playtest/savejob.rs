//! Spielstand am Marker (B-1221): Das Spiel speichert über seine vorhandene Funktion (Schnellspeichern, im Takt), das
//! Kopieren der Datei nach `<ordner>/saves/marker-<nr>.json` läuft in einem eigenen Thread und blockiert kein Bild.
//! Am Spielstand-Format ändert das nichts.

use std::fs::{copy, create_dir_all, read_dir};
use std::path::{Path, PathBuf};
use std::sync::mpsc::{Receiver, channel};
use std::thread::{sleep, spawn};
use std::time::{Duration, Instant, SystemTime};

/// So lange wartet der Thread auf den frisch geschriebenen Spielstand.
const WAIT: Duration = Duration::from_millis(if cfg!(test) { 300 } else { 2000 });
const POLL: Duration = Duration::from_millis(25);
/// Dateizeiten dürfen so weit vor dem Auslöser liegen (grobe Dateisystem-Uhr).
const SLACK: Duration = Duration::from_millis(50);

/// Pfad des Spielstands relativ zum Session-Ordner.
pub fn rel_name(nr: u32) -> String {
    format!("saves/marker-{nr}.json")
}

/// Die neueste `*.json` direkt in `dir`, die nicht älter als `after` ist.
pub fn newest_since(dir: &Path, after: SystemTime) -> Option<PathBuf> {
    let limit = after.checked_sub(SLACK).unwrap_or(after);
    read_dir(dir)
        .ok()?
        .flatten()
        .filter(|e| e.path().extension().is_some_and(|x| x == "json"))
        .filter_map(|e| Some((e.metadata().ok()?.modified().ok()?, e.path())))
        .filter(|(t, _)| *t >= limit)
        .max_by_key(|(t, _)| *t)
        .map(|(_, p)| p)
}

/// Kopiert den Spielstand, der nach `after` in `src` entsteht, nach `<session>/saves/marker-<nr>.json`; das Ergebnis
/// (`Ok(relativer Name)` oder der Grund) kommt über den Empfänger.
pub fn spawn_copy(src: PathBuf, session: PathBuf, nr: u32, after: SystemTime) -> Receiver<Result<String, String>> {
    let (tx, rx) = channel();
    spawn(move || {
        let _ = tx.send(copy_when_ready(&src, &session, nr, after));
    });
    rx
}

fn copy_when_ready(src: &Path, session: &Path, nr: u32, after: SystemTime) -> Result<String, String> {
    let t0 = Instant::now();
    let file = loop {
        if let Some(f) = newest_since(src, after) {
            break f;
        }
        if t0.elapsed() > WAIT {
            return Err(format!("kein neuer Spielstand in {}", src.display()));
        }
        sleep(POLL);
    };
    let name = rel_name(nr);
    let dest = session.join(&name);
    if let Some(d) = dest.parent() {
        create_dir_all(d).map_err(|e| e.to_string())?;
    }
    copy(&file, &dest).map_err(|e| format!("{}: {e}", dest.display()))?;
    Ok(name)
}
