//! Lesen unter docs/ und Unterbau der Schreibwege (B-210): Jede Änderung wird erst vollständig berechnet
//! ([`ChangeSet`]) und dann geschrieben. Das Auflösen der IDs liegt in `store_resolve.rs`.

use std::collections::{BTreeMap, HashMap};
use std::fs;
use std::io::{self, Write as _};
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::{Mutex, MutexGuard, PoisonError};

use super::roadmap_refresh::refresh_roadmap;
use super::store_remove::{Undo, remove_with_backup};

pub use super::store_resolve::{Ref, is_ticket_id, resolve};

/// Ein Schreibweg zur Zeit, auch bei parallelen MCP-Aufrufen (Go `writeMu`).
static WRITE_LOCK: Mutex<()> = Mutex::new(());

/// Sperrt die Schreibwege vom Auflösen bis nach dem Schreiben. Nicht verschachtelt aufrufen.
pub fn lock_writes() -> MutexGuard<'static, ()> {
    // Die Sperre schützt keine Daten: nach einem Absturz mit ihr darf es weitergehen.
    WRITE_LOCK.lock().unwrap_or_else(PoisonError::into_inner)
}

/// Die Datei unter docs/ ist vorhanden (auch unlesbar).
pub fn exists(root: &Path, rel: &str) -> bool {
    docs_path(root, rel).exists()
}

/// Ordner unter docs/sprints, in Lebenslauf-Reihenfolge.
pub const STATES: [&str; 3] = ["geplant", "aktiv", "erledigt"];

/// Text in Anführungszeichen für Fehlermeldungen (Go `%q`).
pub fn quoted(text: &str) -> String {
    format!("{text:?}")
}

pub fn docs_path(root: &Path, rel: &str) -> PathBuf {
    let mut path = root.join("docs");
    path.extend(rel.split('/').filter(|part| !part.is_empty()));
    path
}

/// Eintrag eines Ordners; ein Link auf einen Ordner zählt wie in Go nicht als Ordner.
#[derive(Debug, Clone)]
pub struct Entry {
    pub name: String,
    pub is_dir: bool,
}

/// Einträge eines Ordners nach Name sortiert (wie Go `os.ReadDir`), nie in der Reihenfolge des Dateisystems.
pub fn read_dir_sorted(dir: &Path) -> io::Result<Vec<Entry>> {
    let mut out = Vec::new();
    for entry in fs::read_dir(dir)? {
        let entry = entry?;
        out.push(Entry {
            name: entry.file_name().to_string_lossy().into_owned(),
            is_dir: entry.file_type()?.is_dir(),
        });
    }
    out.sort_by(|a, b| a.name.cmp(&b.name));
    Ok(out)
}

/// Wie [`read_dir_sorted`], ein unlesbarer Ordner ist leer.
pub fn entries(dir: &Path) -> Vec<Entry> {
    read_dir_sorted(dir).unwrap_or_default()
}

/// Dateiinhalt zum Lesen; ungültige UTF-8-Folgen werden ersetzt.
pub fn read_lossy(path: &Path) -> io::Result<String> {
    Ok(match String::from_utf8(fs::read(path)?) {
        Ok(text) => text,
        Err(e) => String::from_utf8_lossy(e.as_bytes()).into_owned(),
    })
}

/// Liest eine Datei mit `\n`; der zweite Wert merkt sich CRLF fürs Zurückschreiben.
pub fn read_text(root: &Path, rel: &str) -> Result<(String, bool), String> {
    let path = docs_path(root, rel);
    let text = read_lossy(&path).map_err(|e| format!("open {}: {e}", path.display()))?;
    Ok((text.replace("\r\n", "\n"), text.contains("\r\n")))
}

/// Sammelt Schreib-, Verschiebe- und Löschschritte; [`ChangeSet::apply`] führt sie erst aus, wenn alles berechnet ist.
#[derive(Debug, Default)]
pub struct ChangeSet {
    pub root: PathBuf,
    /// von, nach (relativ zu docs/)
    pub moves: Vec<(String, String)>,
    pub writes: BTreeMap<String, String>,
    pub crlf: HashMap<String, bool>,
    pub remove: Vec<String>,
    pub notes: Vec<String>,
    /// Sprint-Titel wurde geändert: Fahrplan-Thema überschreiben.
    pub retitle: bool,
    /// Dateien, die vor allen anderen geschrieben werden (Index, der eine Zeile bekommt; B-862).
    pub first: Vec<String>,
}

impl ChangeSet {
    pub fn new(root: &Path) -> ChangeSet {
        ChangeSet {
            root: root.to_owned(),
            ..ChangeSet::default()
        }
    }

    /// Stand einer Datei inklusive schon geplanter Änderungen. Anders als beim Lesen zur Anzeige ist ungültiges UTF-8
    /// ein Fehler: Zurückschreiben würde die Bytes sonst verändern.
    pub fn read(&mut self, rel: &str) -> Result<String, String> {
        if let Some(text) = self.writes.get(rel) {
            return Ok(text.clone());
        }
        let path = docs_path(&self.root, rel);
        let bytes = fs::read(&path).map_err(|e| format!("open {}: {e}", path.display()))?;
        let text = String::from_utf8(bytes).map_err(|_| format!("{rel}: kein gültiges UTF-8"))?;
        self.crlf.insert(rel.to_owned(), text.contains("\r\n"));
        Ok(text.replace("\r\n", "\n"))
    }

    pub fn write(&mut self, rel: &str, text: String) {
        self.writes.insert(rel.to_owned(), text);
    }

    /// Zuerst die Dateien aus `first` (in ihrer Reihenfolge), dann die übrigen sortiert.
    pub fn write_order(&self) -> Vec<String> {
        let mut keys: Vec<String> = Vec::new();
        for key in &self.first {
            if self.writes.contains_key(key) && !keys.contains(key) {
                keys.push(key.clone());
            }
        }
        let rest: Vec<String> = self.writes.keys().filter(|k| !keys.contains(k)).cloned().collect();
        keys.extend(rest);
        keys
    }

    /// Verschiebt, schreibt oder löscht etwas unter docs/sprints (Sprint, Session, Fahrplan).
    pub fn touches_sprints(&self) -> bool {
        let moved = self.moves.iter().flat_map(|(from, to)| [from, to]);
        self.writes
            .keys()
            .chain(&self.remove)
            .chain(moved)
            .any(|p| p.starts_with("sprints/"))
    }

    /// Führt Verschieben, Schreiben und Löschen aus. Scheitert ein Schritt, nimmt es die schon ausgeführten zurück
    /// (B-862). Danach folgt bei Sprint- oder Session-Änderungen der Fahrplan (B-842).
    pub fn apply(&self) -> Result<(), String> {
        let mut undo: Vec<Undo> = Vec::new();
        if let Err(error) = self.steps(&mut undo) {
            return Err(roll_back(undo, &error));
        }
        // Nur bei Sprint- oder Session-Änderungen (B-865: reine Ticket-Änderungen lassen den Fahrplan unberührt). Ein
        // Fehler meldet die schon geschriebene Änderung nicht als gescheitert (sonst legte ein zweiter Versuch ein
        // Duplikat an); der nächste Sprint-Aufruf heilt den Fahrplan.
        if self.touches_sprints() {
            let _ = refresh_roadmap(&self.root);
        }
        Ok(())
    }

    fn steps(&self, undo: &mut Vec<Undo>) -> Result<(), String> {
        for (from, to) in &self.moves {
            let (from, to) = (docs_path(&self.root, from), docs_path(&self.root, to));
            if let Some(parent) = to.parent() {
                fs::create_dir_all(parent).map_err(|e| e.to_string())?;
            }
            fs::rename(&from, &to).map_err(|e| format!("rename {}: {e}", from.display()))?;
            undo.push(Box::new(move || fs::rename(&to, &from)));
        }
        for rel in self.write_order() {
            self.write_step(&rel, undo)?;
        }
        for rel in &self.remove {
            // B-867: gesichert, auch ein gescheitertes Entfernen geht zurück.
            let (restore, result) = remove_with_backup(&docs_path(&self.root, rel));
            undo.push(restore);
            result.map_err(|e| e.to_string())?;
        }
        Ok(())
    }

    fn write_step(&self, rel: &str, undo: &mut Vec<Undo>) -> Result<(), String> {
        let path = docs_path(&self.root, rel);
        let old = match fs::read(&path) {
            Ok(bytes) => Some(bytes),
            Err(e) if e.kind() == io::ErrorKind::NotFound => None,
            // Ohne alten Inhalt kein sicheres Zurücknehmen; eine vorhandene Datei nie löschen.
            Err(e) => return Err(format!("open {}: {e}", path.display())),
        };
        let text = self.writes.get(rel).map_or("", String::as_str);
        let crlf = self.crlf.get(rel).copied().unwrap_or(false);
        write_file(&path, text, crlf).map_err(|e| format!("{}: {e}", path.display()))?;
        undo.push(Box::new(move || match old {
            Some(bytes) => write_bytes_atomic(&path, &bytes), // Bytes wie gelesen, Zeilenenden inklusive
            None => fs::remove_file(&path),                   // die Datei war neu
        }));
        Ok(())
    }
}

fn roll_back(mut undo: Vec<Undo>, error: &str) -> String {
    let mut failed = Vec::new();
    while let Some(step) = undo.pop() {
        if let Err(e) = step() {
            failed.push(e.to_string());
        }
    }
    if failed.is_empty() {
        format!("{error} (Änderung zurückgenommen)")
    } else {
        format!("{error} (Zurücknehmen unvollständig: {})", failed.join("\n"))
    }
}

/// Schreibt eine Datei des `ChangeSet`; Tests erzwingen hier einen Schreibfehler (B-862).
fn write_file(path: &Path, text: &str, crlf: bool) -> io::Result<()> {
    #[cfg(test)]
    super::testhooks::before_write(path)?;
    write_atomic(path, text, crlf)
}

static TEMP_NUMBER: AtomicU64 = AtomicU64::new(0);

/// Neue Temp-Datei `.plan-*` im selben Ordner.
fn temp_file(dir: &Path) -> io::Result<(PathBuf, fs::File)> {
    loop {
        let number = TEMP_NUMBER.fetch_add(1, Ordering::Relaxed);
        let path = dir.join(format!(".plan-{}-{number}", std::process::id()));
        match fs::OpenOptions::new().write(true).create_new(true).open(&path) {
            Ok(file) => return Ok((path, file)),
            Err(e) if e.kind() == io::ErrorKind::AlreadyExists => {}
            Err(e) => return Err(e),
        }
    }
}

fn write_bytes_atomic(path: &Path, bytes: &[u8]) -> io::Result<()> {
    let dir = path.parent().unwrap_or_else(|| Path::new("."));
    fs::create_dir_all(dir)?;
    let (temp, mut file) = temp_file(dir)?;
    let written = file.write_all(bytes);
    drop(file);
    if let Err(e) = written {
        let _ = fs::remove_file(&temp);
        return Err(io::Error::new(e.kind(), format!("schreiben: {e}")));
    }
    k3c_assetkit::ticket::rename(&temp, path).map_err(|e| io::Error::other(e.to_string()))
}

/// Schreibt über eine Temp-Datei im selben Ordner, damit nie eine halbe Datei liegt; `crlf` stellt die Zeilenenden der
/// gelesenen Datei wieder her.
pub fn write_atomic(path: &Path, text: &str, crlf: bool) -> io::Result<()> {
    if crlf {
        write_bytes_atomic(path, text.replace('\n', "\r\n").as_bytes())
    } else {
        write_bytes_atomic(path, text.as_bytes())
    }
}
