//! Werte-Overrides schreiben (Port von `override.go`) und der gemeinsame Schreibweg (`writeMu`, `writeAtomic`). Das
//! Modul heißt `overrides`, weil `override` ein reserviertes Wort ist.

use std::collections::BTreeMap;
use std::fs::File;
use std::io::{self, Write};
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::{Mutex, MutexGuard, PoisonError};
use std::time::{SystemTime, UNIX_EPOCH};

use k3c_kit::logger::Logger;
use serde::{Deserialize, Serialize};
use serde_json::value::RawValue;

use super::Editor;
use super::edit::Warnings;
use super::patch::{patch_overrides, same_json};
use super::rename::rename_replace;
use super::values::{ValuesView, parse_entries};
use crate::kitx::go_quote;

/// Reiht schreibende Aufrufe: zwei gleichzeitige Änderungen derselben Datei verlieren keinen Wert.
static WRITE_MU: Mutex<()> = Mutex::new(());

pub(super) fn write_lock() -> MutexGuard<'static, ()> {
    WRITE_MU.lock().unwrap_or_else(PoisonError::into_inner)
}

/// Der Wert ist gesetzt (nicht leer, nicht `null`).
pub(super) fn given(value: &str) -> bool {
    let text = value.trim();
    !text.is_empty() && text != "null"
}

/// `overrides` der Welt-Datei: je Basis-Datei ihr Objekt.
pub(super) type Overrides = BTreeMap<String, Box<RawValue>>;

/// Liest `overrides` aus der Welt-Datei `raw`.
pub(super) fn overrides_of(raw: &[u8]) -> Result<Overrides, String> {
    let fail = |e: serde_json::Error| format!("Welt-Datei: {e}");
    let entries = parse_entries(raw).map_err(fail)?;
    let Some((_, block)) = entries.into_iter().rfind(|(k, _)| k == "overrides") else {
        return Ok(Overrides::new());
    };
    let found = serde_json::from_str::<Option<Overrides>>(block.get()).map_err(fail)?;
    Ok(found.unwrap_or_default())
}

/// JSON-Baum eines Overrides, wie Go ihn mit `UseNumber` liest und wieder schreibt: Schlüssel sortiert, Zahlen wie im
/// Text (1.50 bleibt 1.50).
enum Tree {
    Object(BTreeMap<String, Tree>),
    List(Vec<Tree>),
    /// Fertiger JSON-Text.
    Scalar(String),
}

/// Tiefste Schachtelung (Schutz vor Stapelüberlauf).
const MAX_DEPTH: usize = 512;
/// Längster Override-Pfad aus einem Aufruf.
const MAX_PATH: usize = 64;

impl Tree {
    fn read(value: &RawValue, depth: usize) -> Result<Tree, String> {
        if depth > MAX_DEPTH {
            return Err("zu tief geschachtelt".to_owned());
        }
        let text = value.get();
        Ok(match text.bytes().next() {
            Some(b'{') => Tree::Object(Tree::object(text, depth)?),
            Some(b'[') => {
                let items = serde_json::from_str::<Vec<&RawValue>>(text).map_err(|e| e.to_string())?;
                let items = items.into_iter().map(|v| Tree::read(v, depth + 1));
                Tree::List(items.collect::<Result<_, _>>()?)
            }
            Some(b'"') => Tree::Scalar(go_quote(
                &serde_json::from_str::<String>(text).map_err(|e| e.to_string())?,
            )),
            _ => Tree::Scalar(text.to_owned()),
        })
    }

    /// Die Einträge des Objekts `text` (der letzte doppelte Schlüssel gewinnt).
    fn object(text: &str, depth: usize) -> Result<BTreeMap<String, Tree>, String> {
        let entries = parse_entries(text.as_bytes()).map_err(|e| e.to_string())?;
        let entries = entries
            .into_iter()
            .map(|(k, v)| Ok::<_, String>((k, Tree::read(v, depth + 1)?)));
        entries.collect()
    }

    fn write(&self) -> String {
        match self {
            Tree::Object(map) => write_object(map),
            Tree::List(items) => {
                let items: Vec<String> = items.iter().map(Tree::write).collect();
                format!("[{}]", items.join(","))
            }
            Tree::Scalar(text) => text.clone(),
        }
    }
}

fn write_object(map: &BTreeMap<String, Tree>) -> String {
    let parts: Vec<String> = map
        .iter()
        .map(|(k, v)| format!("{}:{}", go_quote(k), v.write()))
        .collect();
    format!("{{{}}}", parts.join(","))
}

/// Setzt (`Some`) oder entfernt (`None`) das Blatt am Pfad und räumt leer gewordene Objekte auf.
fn set_path(node: &mut BTreeMap<String, Tree>, path: &[String], leaf: Option<Tree>) {
    let Some((key, rest)) = path.split_first() else {
        return;
    };
    if rest.is_empty() {
        match leaf {
            Some(leaf) => node.insert(key.clone(), leaf),
            None => node.remove(key),
        };
        return;
    }
    if !matches!(node.get(key), Some(Tree::Object(_))) {
        if leaf.is_none() {
            return;
        }
        node.insert(key.clone(), Tree::Object(BTreeMap::new()));
    }
    let Some(Tree::Object(child)) = node.get_mut(key) else {
        return;
    };
    set_path(child, rest, leaf);
    if child.is_empty() {
        node.remove(key);
    }
}

/// Die Overrides mit dem Blatt `base`/`path` = `value` (`None` entfernt es); leere Objekte entfallen.
fn with_leaf(mut ov: Overrides, base: &str, path: &[String], value: Option<&str>) -> Result<Overrides, String> {
    if path.is_empty() {
        return Err("der Pfad fehlt".to_owned());
    }
    // Der Baum wird rekursiv geschrieben und abgeräumt; ein Stapelüberlauf beendete das ganze Studio.
    if path.len() > MAX_PATH {
        return Err(format!("der Pfad ist zu tief (höchstens {MAX_PATH} Ebenen)"));
    }
    let mut tree = match ov.get(base).filter(|v| given(v.get())) {
        Some(v) => Tree::object(v.get(), 0).map_err(|e| format!("overrides › {base}: {e}"))?,
        None => BTreeMap::new(),
    };
    let leaf = value
        .map(|text| {
            let raw = serde_json::from_str::<&RawValue>(text).map_err(|e| e.to_string())?;
            Tree::read(raw, 0)
        })
        .transpose()
        .map_err(|e| format!("der Wert ist kein JSON: {e}"))?;
    set_path(&mut tree, path, leaf);
    if tree.is_empty() {
        ov.remove(base);
    } else {
        let text = RawValue::from_string(write_object(&tree)).map_err(|e| e.to_string())?;
        ov.insert(base.to_owned(), text);
    }
    Ok(ov)
}

#[derive(Default, Deserialize)]
struct Check {
    warnings: Warnings,
}

#[derive(Serialize)]
struct CheckRequest<'a> {
    op: &'a str,
    world: &'a str,
    draft: &'a RawValue,
}

/// Neue Datei `.tmp-world-*` im Ordner von `path` (das Präfix ignoriert der Datei-Watch).
pub(super) fn temp_file(path: &Path) -> io::Result<(PathBuf, File)> {
    static COUNTER: AtomicU64 = AtomicU64::new(0);
    let dir = path.parent().unwrap_or_else(|| Path::new("."));
    loop {
        let now = SystemTime::now().duration_since(UNIX_EPOCH).unwrap_or_default();
        let name = format!(
            ".tmp-world-{}-{}-{}",
            std::process::id(),
            now.as_nanos(),
            COUNTER.fetch_add(1, Ordering::Relaxed)
        );
        let tmp = dir.join(name);
        match File::create_new(&tmp) {
            Ok(file) => return Ok((tmp, file)),
            Err(e) if e.kind() == io::ErrorKind::AlreadyExists => {}
            Err(e) => return Err(e),
        }
    }
}

/// Schreibt über eine Temp-Datei im selben Ordner, damit nie eine halbe Datei liegt; die Rechte bleiben.
pub(super) fn write_atomic(path: &Path, content: &[u8], log: &Logger) -> Result<(), String> {
    let (tmp, mut file) = temp_file(path).map_err(|e| e.to_string())?;
    if let Ok(meta) = std::fs::metadata(path) {
        let _ = file.set_permissions(meta.permissions());
    }
    let written = file.write_all(content);
    drop(file);
    let result = written
        .map_err(|e| format!("schreiben: {e}"))
        .and_then(|()| rename_replace(&tmp, path, log).map_err(|e| e.to_string()));
    if result.is_err() {
        let _ = std::fs::remove_file(&tmp);
    }
    result
}

impl Editor {
    /// Übersteuert das Feld `base`/`path` der Welt `id` mit `value` (JSON). Leer, `null` oder ein Wert gleich dem
    /// Basiswert entfernt den Override. Nur Felder der Ansicht „Werte“ sind erlaubt. Vor dem Schreiben prüft die
    /// Spiel-EXE den Entwurf für die Welt und alle Nachfolger; ungültig bleibt die Datei, wie sie ist, und `error` nennt
    /// den Grund. Die Antwort ist die neue Ansicht.
    pub fn set_override(&self, id: &str, base: &str, path: &[String], value: &str) -> ValuesView {
        let _guard = write_lock();
        let mut view = self.values(id);
        if !view.error.is_empty() {
            return view;
        }
        let found = view.entries.iter().find(|v| v.file == base && v.path == path);
        let Some(base_value) = found.map(|v| v.base.clone()) else {
            view.error = self.reject_override(id, &view.entries, base, path);
            return view;
        };
        match self.write_override(id, base, path, value, base_value.get()) {
            Ok(warnings) => {
                let mut next = self.values(id);
                next.warnings = warnings.unwrap_or_default();
                next
            }
            Err(e) => {
                view.error = e;
                view
            }
        }
    }

    /// Setzt das Blatt, lässt den Entwurf prüfen und schreibt die Datei nur bei einer inhaltlichen Änderung.
    fn write_override(
        &self,
        id: &str,
        base: &str,
        path: &[String],
        value: &str,
        base_value: &str,
    ) -> Result<Warnings, String> {
        let (raw, out) = self.draft_override(id, base, path, value, base_value)?;
        self.commit_change(id, &raw, &out)
    }

    /// Die Datei der Welt `id` und der Entwurf mit dem Blatt `base`/`path` = `value`.
    pub(super) fn draft_override(
        &self,
        id: &str,
        base: &str,
        path: &[String],
        value: &str,
        base_value: &str,
    ) -> Result<(Vec<u8>, Vec<u8>), String> {
        let raw = self.read(id)?;
        let current = overrides_of(&raw)?;
        let reset = !given(value) || same_json(value.as_bytes(), base_value.as_bytes());
        let ov = with_leaf(current, base, path, (!reset).then_some(value))?;
        let out = patch_overrides(&raw, &ov)?;
        Ok((raw, out))
    }

    /// Lässt die Spiel-EXE den Entwurf `out` der Welt `id` prüfen; die Antwort sind die Warnungen.
    pub(super) fn check_draft(&self, id: &str, out: &[u8]) -> Result<Warnings, String> {
        let draft = serde_json::from_slice::<&RawValue>(out).map_err(|e| e.to_string())?;
        let check = self.call::<Check>(&CheckRequest {
            op: "check",
            world: id,
            draft,
        })?;
        Ok(check.warnings)
    }
}

#[cfg(test)]
#[path = "override_tests.rs"]
mod tests;
