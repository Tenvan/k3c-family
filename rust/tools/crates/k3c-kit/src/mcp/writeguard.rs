//! Schreibschutz (Port von `mcpkit/writeguard.go`).

use std::fs;
use std::path::{Path, PathBuf};

use super::workspace::{ambiguous, canon, clean, join, rel_under, same_name};

/// Lässt Schreibzugriffe nur in bestimmten Ordnern der Repo-Wurzel zu (WZ63.7: das Balancing des Studios schreibt
/// Quelldateien nur unter `data/worlds/`, Laufausgaben unter `reports/`). Absolute Pfade und Windows-Kurznamen werden
/// aufgelöst; ein `..` im Pfad und jeder Link oder Junction auf dem Weg unterhalb der Wurzel werden abgelehnt.
#[derive(Clone, Debug, Default)]
pub struct WriteGuard {
    /// Wer schreibt, für die Meldung („Balancing“).
    pub name: String,
    /// Erlaubte Ordner relativ zur Wurzel, mit `/` („data/worlds“, „reports“).
    pub allow: Vec<String>,
    /// Zusatz der Ablehnung, z. B. wie es stattdessen geht.
    pub hint: String,
}

impl WriteGuard {
    /// Prüft `path` (absolut oder relativ zu `root`) und liefert eine deutsche Ablehnung, wenn er nicht erlaubt ist.
    pub fn check(&self, root: impl AsRef<Path>, path: &str) -> Result<(), String> {
        let root = root.as_ref();
        if root.as_os_str().is_empty() {
            return Err(format!("{}: Repo-Wurzel fehlt, Schreiben abgelehnt", self.name));
        }
        if path.is_empty() || path.split(['/', '\\']).any(|part| part == "..") {
            return Err(self.deny(path, "Pfad leer oder mit „..“"));
        }
        if ambiguous(path) {
            return Err(self.deny(path, "Pfad ohne Laufwerk oder mit „:“"));
        }
        let full = if Path::new(path).is_absolute() {
            PathBuf::from(path)
        } else {
            join(root, path)
        };
        let root = canon_unlinked(root);
        let Some(rel) = rel_under(&root, &canon_unlinked(&full)) else {
            return Err(self.deny(path, "außerhalb der Repo-Wurzel"));
        };
        if let Some(link) = link_on_way(&root, &rel) {
            let why = format!("Weg führt über den Link {}", to_slash(&link));
            return Err(self.deny(path, &why));
        }
        let rel = if rel.as_os_str().is_empty() {
            ".".to_owned()
        } else {
            to_slash(&rel)
        };
        if self.allow.iter().any(|dir| has_dir_prefix(&rel, dir)) {
            return Ok(());
        }
        Err(self.deny(&rel, &format!("nicht in {}/", self.allow.join("/, "))))
    }

    fn deny(&self, path: &str, why: &str) -> String {
        let path = to_slash(Path::new(path));
        let mut msg = format!("{} darf {path:?} nicht schreiben ({why})", self.name);
        if !self.hint.is_empty() {
            msg.push_str("; ");
            msg.push_str(&self.hint);
        }
        msg
    }
}

fn to_slash(p: &Path) -> String {
    let s = p.to_string_lossy();
    if cfg!(windows) {
        s.replace('\\', "/")
    } else {
        s.into_owned()
    }
}

/// Symlink, Junction oder (unter Windows) ein anderer Reparse-Punkt.
fn is_link(p: &Path) -> bool {
    let Ok(meta) = fs::symlink_metadata(p) else {
        return false;
    };
    #[cfg(windows)]
    {
        use std::os::windows::fs::MetadataExt;
        const FILE_ATTRIBUTE_REPARSE_POINT: u32 = 0x400;
        meta.file_attributes() & FILE_ATTRIBUTE_REPARSE_POINT != 0
    }
    #[cfg(not(windows))]
    {
        meta.file_type().is_symlink()
    }
}

/// Wie [`canon`], löst aber nur bis vor den ersten Link auf. `canonicalize` folgt Junctions, Go `EvalSymlinks` seit 1.23
/// nicht: Ohne diese Grenze verschwände die Junction aus dem Weg, bevor [`link_on_way`] sie sieht.
fn canon_unlinked(p: &Path) -> PathBuf {
    let p = clean(p);
    let link = p.ancestors().filter(|a| is_link(a)).last();
    let split = link.and_then(Path::parent).and_then(|parent| {
        let rest = p.strip_prefix(parent).ok()?;
        Some(canon(parent).join(rest))
    });
    split.unwrap_or_else(|| canon(&p))
}

/// Erster vorhandener Teil von `rel` (unterhalb von `root`), der ein Link ist, relativ zur Wurzel.
fn link_on_way(root: &Path, rel: &Path) -> Option<PathBuf> {
    let mut cur = root.to_path_buf();
    let mut seen = PathBuf::new();
    for part in rel.components() {
        cur.push(part);
        seen.push(part);
        // Fehlt der Teil, entsteht der Rest erst.
        fs::symlink_metadata(&cur).ok()?;
        if is_link(&cur) {
            return Some(seen);
        }
    }
    None
}

/// Ob `rel` der Ordner `dir` ist oder darin liegt; unter Windows ohne Groß- und Kleinschreibung.
fn has_dir_prefix(rel: &str, dir: &str) -> bool {
    if same_name(rel, dir) {
        return true;
    }
    rel.as_bytes().get(dir.len()) == Some(&b'/') && rel.get(..dir.len()).is_some_and(|head| same_name(head, dir))
}

#[cfg(test)]
#[path = "writeguard_tests.rs"]
mod tests;
