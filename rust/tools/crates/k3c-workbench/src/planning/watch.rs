//! Der Wächter der Planungsdateien (Go `watch.go`): Stempel je Datei, die IDs der geänderten Dokumente und das
//! Polling, aus dem das Ereignis `planning:changed` entsteht.

use std::collections::{BTreeMap, BTreeSet};
use std::fs;
use std::path::Path;
use std::sync::LazyLock;
use std::sync::mpsc::{self, RecvTimeoutError, Sender};
use std::thread::{self, JoinHandle};
use std::time::{Duration, UNIX_EPOCH};

use regex::Regex;
use serde::Serialize;

use super::planning::DOCS;

/// Ab so vielen geänderten Dateien nennt das Ereignis keine IDs mehr (die Oberfläche lädt alles neu).
pub(super) const MAX_CHANGED_IDS: usize = 50;

/// Nutzlast des Ereignisses `planning:changed`: die IDs der geänderten Dokumente (B-123, WZ61, WZ61.1, `plan`,
/// `fragen`, `glossar`), sortiert und ohne Doppelte. Fehlt `ids`, ist die Lage unklar oder die Änderung groß: die
/// Oberfläche lädt dann die ganze Liste neu.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize)]
pub struct Changed {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub ids: Vec<String>,
}

/// Stempel je Planungsdatei: Pfad relativ zur Wurzel mit `/` → Größe und Änderungszeit.
pub type Snapshot = BTreeMap<String, String>;

fn file_stamp(meta: &fs::Metadata) -> String {
    let nanos = meta
        .modified()
        .ok()
        .and_then(|time| time.duration_since(UNIX_EPOCH).ok())
        .map_or(0, |since| since.as_nanos());
    format!("{}|{nanos}", meta.len())
}

/// Läuft durch `path` (Pfad ab der Wurzel `rel`). Verzeichnisse der erledigten Sprints zählen nur über ihren Namen:
/// ihre Änderungszeit ist unter Windows unzuverlässig, und nur die Zahl der erledigten Sprints ändert die Anzeige.
fn walk(path: &Path, rel: &str, dir: &str, snap: &mut Snapshot) {
    let Ok(entries) = fs::read_dir(path) else {
        return;
    };
    for entry in entries.flatten() {
        let name = entry.file_name().to_string_lossy().into_owned();
        let child = format!("{rel}/{name}");
        let is_dir = entry.file_type().is_ok_and(|kind| kind.is_dir());
        if is_dir && (dir == "backlog" || dir == "projekte") {
            continue; // archiv/ ändert die Anzeige nicht
        }
        if is_dir && dir.ends_with("erledigt") {
            snap.insert(format!("{child}/"), String::new());
            continue;
        }
        if name.ends_with(".md")
            && let Ok(meta) = entry.metadata()
        {
            snap.insert(child.clone(), file_stamp(&meta));
        }
        if is_dir {
            walk(&entry.path(), &child, dir, snap);
        }
    }
}

/// Der Stand aller Planungsdateien. Gelesen werden nur Metadaten.
pub fn snapshot(root: &Path) -> Snapshot {
    let mut snap = Snapshot::new();
    let docs = root.join("docs");
    for dir in [
        "sprints/aktiv",
        "sprints/geplant",
        "sprints/erledigt",
        "backlog",
        "projekte",
    ] {
        let mut top = docs.clone();
        top.extend(dir.split('/'));
        walk(&top, &format!("docs/{dir}"), dir, &mut snap);
    }
    for (_, file) in DOCS {
        if let Ok(meta) = fs::metadata(docs.join(file)) {
            snap.insert(format!("docs/{file}"), file_stamp(&meta));
        }
    }
    snap
}

/// Fingerabdruck aller Planungsdateien ([`snapshot`]): ändert sich irgendetwas daran, ändert sich der Wert.
pub fn stamp(root: &Path) -> String {
    stamp_of(&snapshot(root))
}

/// FNV-1a (64 Bit) über die sortierten Zeilen.
fn stamp_of(snap: &Snapshot) -> String {
    let mut hash: u64 = 0xcbf2_9ce4_8422_2325;
    for (key, value) in snap {
        for byte in format!("{key}|{value}\n").bytes() {
            hash ^= u64::from(byte);
            hash = hash.wrapping_mul(0x0000_0100_0000_01b3);
        }
    }
    format!("{hash:x}")
}

static TICKET: LazyLock<Option<Regex>> = LazyLock::new(|| Regex::new(r"^(B-[0-9]+)-.*\.md$").ok());
static PROJECT: LazyLock<Option<Regex>> = LazyLock::new(|| Regex::new(r"^([A-Z][A-Z0-9]*)-.*\.md$").ok());
static SESSION: LazyLock<Option<Regex>> =
    LazyLock::new(|| Regex::new(r"^([A-Za-z]+[0-9]+\.[0-9]+[a-z]?)-.*\.md$").ok());

fn first_group(pattern: &LazyLock<Option<Regex>>, name: &str) -> Option<String> {
    let found = pattern.as_ref()?.captures(name)?;
    found.get(1).map(|group| group.as_str().to_owned())
}

/// Die IDs der Dokumente, die neu, weg oder anders sind. Mehr als [`MAX_CHANGED_IDS`] Dateien, keine Änderung oder
/// eine Datei ohne erkennbare ID ergeben `None` (unklare Lage: alles neu laden).
pub fn changed_ids(old: &Snapshot, cur: &Snapshot) -> Option<Vec<String>> {
    let new_or_changed = cur.iter().filter(|(k, v)| old.get(*k) != Some(*v)).map(|(k, _)| k);
    let gone = old.keys().filter(|k| !cur.contains_key(*k));
    let files: BTreeSet<&String> = new_or_changed.chain(gone).collect();
    if files.is_empty() || files.len() > MAX_CHANGED_IDS {
        return None;
    }
    let mut ids = BTreeSet::new();
    for file in files {
        if is_index_file(file) {
            continue; // Index-README: ändert keine Liste, kommt mit jeder Ticket-Änderung mit
        }
        ids.insert(id_of_path(file)?);
    }
    Some(ids.into_iter().collect())
}

/// Die Index-READMEs von Backlog, Projekten und Sprint-Status-Ordnern.
fn is_index_file(path: &str) -> bool {
    let parts: Vec<&str> = path.split('/').collect();
    matches!(
        parts.as_slice(),
        [_, "backlog" | "projekte", "README.md"] | [_, "sprints", _, "README.md"]
    )
}

/// Bildet einen Planungspfad (docs/…) auf die ID ab; `None` bei unbekannter Form.
fn id_of_path(path: &str) -> Option<String> {
    if let Some((name, _)) = DOCS.iter().find(|(_, file)| path == format!("docs/{file}")) {
        return Some((*name).to_owned());
    }
    let trimmed = path.strip_suffix('/').unwrap_or(path);
    let parts: Vec<&str> = trimmed.split('/').collect();
    let sprint_of = |folder: &str| folder.split('-').next().unwrap_or_default().to_owned();
    let id = match parts.as_slice() {
        [_, "backlog", name] => first_group(&TICKET, name),
        [_, "projekte", name] => first_group(&PROJECT, name),
        // Ordner eines erledigten Sprints
        [_, "sprints", _, folder] => path.ends_with('/').then(|| sprint_of(folder)),
        [_, "sprints", _, folder, name] => first_group(&SESSION, name)
            .filter(|_| !name.eq_ignore_ascii_case("README.md"))
            .or_else(|| Some(sprint_of(folder))),
        _ => None,
    };
    id.filter(|id| !id.is_empty())
}

/// Der laufende Wächter; beim Fallenlassen hält sein Thread an.
pub struct Watcher {
    stop: Option<Sender<()>>,
    thread: Option<JoinHandle<()>>,
}

impl Drop for Watcher {
    fn drop(&mut self) {
        // Der geschlossene Kanal weckt den Thread sofort.
        drop(self.stop.take());
        if let Some(thread) = self.thread.take()
            && thread.join().is_err()
        {
            eprintln!("💥 planungs-wächter abgebrochen");
        }
    }
}

/// Prüft im Takt `every`, ob sich der Stand ändert, und ruft dann `on_change` mit den geänderten IDs (`None` bei
/// unklarer Lage, siehe [`changed_ids`]). Der Stand beim Start gilt als Ausgangsstand und löst nichts aus.
///
/// Polling statt Dateisystem-Ereignissen: ein Stat über ein paar hundert Dateien alle 2 s kostet nichts.
pub fn watch_ids(
    root: &Path,
    every: Duration,
    mut on_change: impl FnMut(Option<Vec<String>>) + Send + 'static,
) -> Watcher {
    let root = root.to_owned();
    let mut last = snapshot(&root);
    let mut last_stamp = stamp_of(&last);
    let (stop, stopped) = mpsc::channel::<()>();
    let thread = thread::spawn(move || {
        while stopped.recv_timeout(every) == Err(RecvTimeoutError::Timeout) {
            let cur = snapshot(&root);
            let cur_stamp = stamp_of(&cur);
            if cur_stamp != last_stamp {
                let ids = changed_ids(&last, &cur);
                (last, last_stamp) = (cur, cur_stamp);
                on_change(ids);
            }
        }
    });
    Watcher {
        stop: Some(stop),
        thread: Some(thread),
    }
}
