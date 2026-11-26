//! EXE des Checkouts suchen und auf Frische prüfen (Port von `find.go`). Die Orte stehen fest unterhalb der Wurzel,
//! nichts davon kommt aus Eingaben und nichts wird über `PATH` gesucht.

use std::path::{Path, PathBuf};
use std::time::SystemTime;

/// Baut die Spiel-EXE; die Fehler bei fehlender oder veralteter EXE nennen ihn.
pub const BUILD_COMMAND: &str = "task build";

/// Orte der EXE, zuerst der Build im Checkout, dann das Spiel-Paket, je mit dem Arbeitsordner, unter dem das Spiel
/// `public/` findet. Pfade relativ zur Wurzel des Checkouts.
const CANDIDATES: [(&str, &str); 2] = [
    ("rust/target/release/k3c-game.exe", "."),
    ("dist/k3c-game/k3c-game.exe", "dist/k3c-game"),
];

/// Hängt `rel` (mit `/`) an `root`; `.` ist die Wurzel selbst.
fn below(root: &Path, rel: &str) -> PathBuf {
    let mut path = root.to_path_buf();
    path.extend(rel.split('/').filter(|part| *part != "."));
    path
}

fn is_file(path: &Path) -> bool {
    std::fs::metadata(path).is_ok_and(|m| !m.is_dir())
}

fn modified(path: &Path) -> Option<SystemTime> {
    std::fs::metadata(path).and_then(|m| m.modified()).ok()
}

/// Die erste vorhandene EXE im Checkout `root` und ihr Arbeitsordner, ohne die Frische zu prüfen („Im Spiel ansehen“
/// nimmt sie so). Ohne EXE ein Fehler mit dem Bau-Befehl.
pub fn locate(root: &Path) -> Result<(PathBuf, PathBuf), String> {
    for (exe, dir) in CANDIDATES {
        let exe = below(root, exe);
        if is_file(&exe) {
            return Ok((exe, below(root, dir)));
        }
    }
    Err(format!(
        "natives Spiel fehlt ({}) – erst bauen: {BUILD_COMMAND}",
        CANDIDATES[0].0
    ))
}

/// Die erste frische EXE (siehe [`stale`]). Ist keine frisch, kommt ein Fehler mit dem Bau-Befehl und es wird nicht
/// gerechnet; fehlt jede EXE, ebenso.
pub fn find(root: &Path) -> Result<(PathBuf, PathBuf), String> {
    let mut stale_error = None;
    for (candidate, dir) in CANDIDATES {
        let exe = below(root, candidate);
        if !is_file(&exe) {
            continue;
        }
        let Some(source) = stale(root, &exe) else {
            return Ok((exe, below(root, dir)));
        };
        stale_error.get_or_insert_with(|| {
            let source = source.to_string_lossy().replace('\\', "/");
            format!("natives Spiel ist veraltet ({source} ist jünger als {candidate}) – erst bauen: {BUILD_COMMAND}")
        });
    }
    match stale_error {
        Some(e) => Err(e),
        None => locate(root),
    }
}

/// Die jüngste Datei, die jünger als die EXE ist.
struct Newest {
    built: SystemTime,
    found: Option<(SystemTime, PathBuf)>,
}

impl Newest {
    fn consider(&mut self, path: &Path, mtime: SystemTime) {
        if mtime > self.built && self.found.as_ref().is_none_or(|(latest, _)| mtime > *latest) {
            self.found = Some((mtime, path.to_path_buf()));
        }
    }

    /// Alle `*.rs` und `Cargo.toml` unter `dir`, ohne Ordner `target`, in Namensfolge.
    fn walk(&mut self, dir: &Path) {
        let Ok(entries) = std::fs::read_dir(dir) else {
            return;
        };
        let mut entries: Vec<_> = entries.filter_map(Result::ok).collect();
        entries.sort_by_key(std::fs::DirEntry::file_name);
        for entry in entries {
            let name = entry.file_name();
            let name = name.to_string_lossy();
            if entry.file_type().is_ok_and(|t| t.is_dir()) {
                if name != "target" {
                    self.walk(&entry.path());
                }
            } else if (name.ends_with(".rs") || name == "Cargo.toml")
                && let Ok(mtime) = entry.metadata().and_then(|m| m.modified())
            {
                self.consider(&entry.path(), mtime);
            }
        }
    }
}

/// Ist eine Datei unter `rust/crates` (`*.rs`, `Cargo.toml`) oder `rust/Cargo.lock` jünger als die EXE (mtime, erfasst
/// auch nicht committete Änderungen, kein git-Aufruf), kommt die jüngste relativ zum Checkout.
pub fn stale(root: &Path, exe: &Path) -> Option<PathBuf> {
    let mut newest = Newest {
        built: modified(exe)?,
        found: None,
    };
    let lock = below(root, "rust/Cargo.lock");
    if let Some(mtime) = modified(&lock) {
        newest.consider(&lock, mtime);
    }
    newest.walk(&below(root, "rust/crates"));
    let (_, path) = newest.found?;
    Some(path.strip_prefix(root).map(Path::to_path_buf).unwrap_or(path))
}
