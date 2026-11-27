//! Änderungen des Branches seit der Abzweigung von main (Port von `files.go`; Commits, ohne Working Tree).

use std::collections::HashMap;

use serde::Serialize;

use super::repo::{BASE_REMOTE, Repo, lines};

/// Eine Datei im Diff des Branches zu origin/main.
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize)]
pub struct DiffFile {
    pub path: String,
    /// A, M, D, R …
    pub code: String,
    pub added: i64,
    pub deleted: i64,
    pub binary: bool,
}

#[derive(Clone, Debug, Default, Serialize)]
pub struct Files {
    /// origin/main.
    pub base: String,
    pub files: Vec<DiffFile>,
}

impl Files {
    /// Ohne Dateien (auch ohne origin/main).
    pub fn empty() -> Files {
        Files {
            base: BASE_REMOTE.to_owned(),
            files: Vec::new(),
        }
    }
}

impl Repo {
    /// Der Diff von der Merge-Basis mit origin/main bis HEAD.
    pub async fn branch_files(&self) -> Result<Files, String> {
        let range = format!("{BASE_REMOTE}...HEAD");
        let names = self.git(&["diff", "--name-status", "--no-renames", &range]).await?;
        let numbers = self.git(&["diff", "--numstat", "--no-renames", &range]).await?;
        Ok(Files {
            files: parse_diff(&names, &numbers),
            ..Files::empty()
        })
    }
}

/// Verbindet `--name-status` (Code) mit `--numstat` (Zeilen) über den Pfad.
pub(super) fn parse_diff(names: &str, numbers: &str) -> Vec<DiffFile> {
    let counts: HashMap<&str, (i64, i64, bool)> = lines(numbers)
        .filter_map(|l| {
            let fields: Vec<&str> = l.splitn(3, '\t').collect();
            let [added, deleted, path] = fields[..] else {
                return None;
            };
            let count = if added == "-" {
                (0, 0, true)
            } else {
                (
                    added.parse().unwrap_or_default(),
                    deleted.parse().unwrap_or_default(),
                    false,
                )
            };
            Some((path, count))
        })
        .collect();
    lines(names)
        .filter_map(|l| {
            let (code, path) = l.split_once('\t')?;
            let code = code.chars().next()?.to_string();
            let (added, deleted, binary) = counts.get(path).copied().unwrap_or_default();
            Some(DiffFile {
                path: path.to_owned(),
                code,
                added,
                deleted,
                binary,
            })
        })
        .collect()
}
