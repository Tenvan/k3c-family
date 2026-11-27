//! Fingerabdruck des Repo-Zustands (Port von `stamp.go`).

use std::fmt::Write;
use std::path::Path;
use std::time::UNIX_EPOCH;

use sha2::{Digest, Sha256};
use walkdir::WalkDir;

use super::repo::Repo;

/// Sie ändern sich bei Checkout, Commit, Fetch, Merge und Reset. Der Index fehlt bewusst: Editoren frischen ihn mit
/// `git status` ständig auf, ohne dass sich etwas ändert; gestagte Änderungen zählen über `diff --cached`.
const STAMP_FILES: [&str; 6] = [
    "HEAD",
    "FETCH_HEAD",
    "ORIG_HEAD",
    "MERGE_HEAD",
    "packed-refs",
    "logs/HEAD",
];

impl Repo {
    /// Größe und Änderungszeit von HEAD, FETCH_HEAD, packed-refs und allen losen Refs (Branches, origin, Stash) sowie
    /// die gestagten Änderungen. Er kostet ein paar stat-Aufrufe und einen schnellen git-Aufruf, damit die Seite ihn im
    /// Takt fragen kann (B-580 Rev. 5). Ungestagtes sieht er nicht.
    pub async fn stamp(&self) -> Result<String, String> {
        // Der Ordner ändert sich nicht und bleibt gemerkt.
        let dir = self.0.dir.get_or_init(|| self.read_common_dir()).await.clone()?;
        let mut text = self.git(&["diff", "--cached", "--raw", "--no-renames"]).await?;
        for file in STAMP_FILES {
            entry(&mut text, file, &dir.join(file));
        }
        let refs = WalkDir::new(dir.join("refs")).sort_by_file_name();
        for found in refs
            .into_iter()
            .filter_map(Result::ok)
            .filter(|e| !e.file_type().is_dir())
        {
            let name = found.path().strip_prefix(&dir).unwrap_or_else(|_| found.path());
            entry(&mut text, &name.to_string_lossy().replace('\\', "/"), found.path());
        }
        let sum = Sha256::digest(text.as_bytes());
        Ok(sum.iter().take(8).fold(String::new(), |mut hex, byte| {
            // Schreiben in einen String scheitert nicht.
            let _ = write!(hex, "{byte:02x}");
            hex
        }))
    }
}

fn entry(text: &mut String, name: &str, path: &Path) {
    let Ok(meta) = std::fs::metadata(path) else {
        return;
    };
    let nanos = meta
        .modified()
        .ok()
        .and_then(|t| t.duration_since(UNIX_EPOCH).ok())
        .map_or(0, |d| d.as_nanos());
    let _ = writeln!(text, "{name} {} {nanos}", meta.len());
}
