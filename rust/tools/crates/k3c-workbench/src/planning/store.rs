//! Auflösen der IDs und Unterbau der Schreibwege (B-210): Jede Änderung wird erst vollständig berechnet
//! ([`ChangeSet`]) und dann geschrieben; Pfade entstehen nur aus geprüften IDs und Kurznamen unter docs/.

use std::collections::{BTreeMap, HashMap};
use std::fs;
use std::io::{self, Write as _};
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::{LazyLock, Mutex, MutexGuard, PoisonError};

use regex::Regex;

use super::projects::is_project_id;
use super::roadmap_refresh::refresh_roadmap;
use super::store_remove::{Undo, remove_with_backup};

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

// Ab B-1000 vierstellig (B-863).
static TICKET_ID: LazyLock<Option<Regex>> = LazyLock::new(|| Regex::new(r"^B-[0-9]{3,}$").ok());
static SPRINT_ID: LazyLock<Option<Regex>> = LazyLock::new(|| Regex::new(r"^[A-Z]+[0-9]+$").ok());
static SESSION_ID: LazyLock<Option<Regex>> = LazyLock::new(|| Regex::new(r"^([A-Z]+[0-9]+)\.([0-9]+[a-z]?)$").ok());
// Trennt doppelt vergebene Nummern (zwei Rechner, gleiche Nummer): B-1287-kurzname wählt genau die Datei.
static TICKET_SLUG: LazyLock<Option<Regex>> =
    LazyLock::new(|| Regex::new(r"^(B-[0-9]{3,})-([a-z0-9]+(?:-[a-z0-9]+)*)$").ok());

fn matches(re: &LazyLock<Option<Regex>>, text: &str) -> bool {
    re.as_ref().is_some_and(|re| re.is_match(text))
}

fn group(re: &LazyLock<Option<Regex>>, text: &str) -> Option<String> {
    Some(re.as_ref()?.captures(text)?.get(1)?.as_str().to_owned())
}

/// `B-123` (ohne Kurzname).
pub fn is_ticket_id(id: &str) -> bool {
    matches(&TICKET_ID, id)
}

/// Ordner unter docs/sprints, in Lebenslauf-Reihenfolge.
pub const STATES: [&str; 3] = ["geplant", "aktiv", "erledigt"];

/// Ein aufgelöstes Planungs-Dokument; `rel` ist der Pfad relativ zu docs/ mit `/`.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct Ref {
    /// ticket | sprint | session | projekt
    pub kind: &'static str,
    pub id: String,
    /// Datei (beim Sprint die README).
    pub rel: String,
    /// Sprint-Ordner relativ zu docs/ (Sprint und Session).
    pub dir: String,
    /// Ordner des Sprints oder „archiv“/leer beim Ticket.
    pub state: String,
}

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

/// Sucht im Ordner den Eintrag mit dem Präfix (Datei mit .md oder Ordner).
fn find_entry(dir: &Path, prefix: &str, want_dir: bool) -> Option<String> {
    entries(dir)
        .into_iter()
        .find(|e| e.is_dir == want_dir && e.name.starts_with(prefix) && (want_dir || e.name.ends_with(".md")))
        .map(|e| e.name)
}

fn find_sprint(root: &Path, id: &str) -> Option<Ref> {
    STATES.iter().find_map(|state| {
        let name = find_entry(&docs_path(root, &format!("sprints/{state}")), &format!("{id}-"), true)?;
        let dir = format!("sprints/{state}/{name}");
        Some(Ref {
            kind: "sprint",
            id: id.to_owned(),
            rel: format!("{dir}/README.md"),
            dir,
            state: (*state).to_owned(),
        })
    })
}

fn ticket_ref(id: &str, folder: &str, file: &str) -> Ref {
    let rel = if folder.is_empty() {
        format!("backlog/{file}")
    } else {
        format!("backlog/{folder}/{file}")
    };
    Ref {
        kind: "ticket",
        id: id.to_owned(),
        rel,
        state: folder.to_owned(),
        ..Ref::default()
    }
}

fn find_ticket(root: &Path, id: &str) -> Option<Ref> {
    ["", "archiv"].into_iter().find_map(|folder| {
        let file = find_entry(&docs_path(root, &format!("backlog/{folder}")), &format!("{id}-"), false)?;
        Some(ticket_ref(id, folder, &file))
    })
}

/// `B-123-kurzname`: genau die Datei `<id>.md`; die ID der Referenz ist die Nummer.
fn find_ticket_file(root: &Path, id: &str) -> Option<Ref> {
    let number = group(&TICKET_SLUG, id)?;
    ["", "archiv"].into_iter().find_map(|folder| {
        let found = ticket_ref(&number, folder, &format!("{id}.md"));
        docs_path(root, &found.rel).exists().then_some(found)
    })
}

fn find_project_file(root: &Path, id: &str) -> Option<Ref> {
    let file = find_entry(&docs_path(root, "projekte"), &format!("{id}-"), false)?;
    Some(Ref {
        kind: "projekt",
        id: id.to_owned(),
        rel: format!("projekte/{file}"),
        ..Ref::default()
    })
}

fn find_session(root: &Path, id: &str) -> Option<Ref> {
    let sprint = find_sprint(root, &group(&SESSION_ID, id)?)?;
    let file = find_entry(&docs_path(root, &sprint.dir), &format!("{id}-"), false)?;
    Some(Ref {
        kind: "session",
        id: id.to_owned(),
        rel: format!("{}/{file}", sprint.dir),
        ..sprint
    })
}

/// Findet ein vorhandenes Dokument zur ID; jede andere Form ist ein Fehler.
pub fn resolve(root: &Path, id: &str) -> Result<Ref, String> {
    let found = if matches(&TICKET_ID, id) {
        find_ticket(root, id)
    } else if matches(&TICKET_SLUG, id) {
        find_ticket_file(root, id)
    } else if matches(&SPRINT_ID, id) {
        find_sprint(root, id)
    } else if is_project_id(id) {
        find_project_file(root, id)
    } else if matches(&SESSION_ID, id) {
        find_session(root, id)
    } else {
        return Err(format!(
            "ungültige ID {} (B-123, M8, M8.1 oder Projekt GRA)",
            quoted(id)
        ));
    };
    found.ok_or_else(|| format!("{id} nicht gefunden"))
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
