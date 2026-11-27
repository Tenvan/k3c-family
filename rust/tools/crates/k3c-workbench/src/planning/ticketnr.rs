//! B-387: Die nächste Ticket-Nummer berücksichtigt alle Checkouts des Repos (Wurzel, Git-Worktrees samt Archiv) und die
//! lokalen Remote-Refs origin/main und origin/sprint/* (aus dem lokalen Stand, ohne Fetch). Die Nummer eines Checkouts
//! und der Dateinamen rechnet der Ticket-Kern (`k3c_assetkit::ticket`, B-595).

use std::collections::BTreeMap;
use std::path::{Path, PathBuf};
use std::time::Duration;

use k3c_assetkit::ticket::{highest, local_highest};

use super::gitrepo::{git_found, git_out, has_git_entry};

const TIMEOUT: Duration = Duration::from_secs(5);

fn git(root: &Path, args: &[&str]) -> Result<String, String> {
    git_out(root, args, TIMEOUT)
}

/// Pfade aller Worktrees (`git worktree list`), die Wurzel eingeschlossen.
fn worktree_paths(root: &Path) -> Result<Vec<PathBuf>, String> {
    let out = git(root, &["worktree", "list", "--porcelain"])?;
    Ok(out
        .replace("\r\n", "\n")
        .split('\n')
        .filter_map(|l| l.strip_prefix("worktree "))
        .map(PathBuf::from)
        .collect())
}

/// Derselbe Ordner. Zuerst der Textvergleich ohne Groß-/Kleinschreibung; sonst die aufgelösten Pfade: git nennt den
/// Langpfad, ein Aufrufer kann einen 8.3-Kurzpfad oder einen Link übergeben (B-482).
fn same_path(a: &Path, b: &Path) -> bool {
    let text = |p: &Path| {
        let clean: PathBuf = p.components().collect();
        clean.to_string_lossy().replace('\\', "/").to_lowercase()
    };
    if text(a) == text(b) {
        return true;
    }
    matches!((a.canonicalize(), b.canonicalize()), (Ok(a), Ok(b)) if a == b)
}

/// Größte Ticket-Nummer je lokalem Remote-Ref (origin/main, origin/sprint/*); `None`, wenn die Refs nicht lesbar sind.
fn remote_highest(root: &Path) -> Option<BTreeMap<String, i64>> {
    let refs = [
        "for-each-ref",
        "--format=%(refname)",
        "refs/remotes/origin/main",
        "refs/remotes/origin/sprint",
    ];
    let out = git(root, &refs).ok()?;
    let mut found = BTreeMap::new();
    for name in out.split_whitespace() {
        let tree = [
            "ls-tree",
            "--name-only",
            name,
            "--",
            "docs/backlog/",
            "docs/backlog/archiv/",
        ];
        let Ok(files) = git(root, &tree) else {
            continue;
        };
        let files: Vec<&str> = files.split_whitespace().collect();
        let short = name.strip_prefix("refs/remotes/").unwrap_or(name);
        found.insert(short.to_owned(), highest(&files));
    }
    Some(found)
}

/// git-Auskünfte gelten nur, wenn `root` selbst die Wurzel eines Worktrees ist; ein Ordner unterhalb eines Repos
/// (z. B. ein Test-Verzeichnis) würde sonst die Worktrees und Refs des umgebenden Repos liefern.
fn is_repo_root(root: &Path) -> bool {
    // Eine Wurzel trägt immer .git; ohne spart das den git-Aufruf (B-830).
    has_git_entry(root)
        && git(root, &["rev-parse", "--show-toplevel"]).is_ok_and(|top| same_path(Path::new(top.trim()), root))
}

/// Höchste bisher gefundene Nummer und ihre Herkunft.
struct Top {
    nr: i64,
    from: String,
}

impl Top {
    fn consider(&mut self, nr: i64, from: impl FnOnce() -> String) {
        if nr > self.nr {
            self.nr = nr;
            self.from = from();
        }
    }
}

/// Berücksichtigt die übrigen Worktrees und die Remote-Refs.
fn look_around(root: &Path, top: &mut Top, notes: &mut Vec<String>) {
    let paths = worktree_paths(root).unwrap_or_else(|e| {
        notes.push(format!("Warnung: Worktrees nicht gelesen ({e})"));
        Vec::new()
    });
    for path in paths.iter().filter(|p| !same_path(p, root)) {
        let name = path
            .file_name()
            .map_or_else(String::new, |n| n.to_string_lossy().into_owned());
        match local_highest(path) {
            Ok(nr) => top.consider(nr, || format!("Worktree {name}")),
            Err(e) => notes.push(format!("übersprungen: Worktree {name} nicht lesbar ({e})")),
        }
    }
    let Some(refs) = remote_highest(root) else {
        notes.push("Warnung: Remote-Refs nicht gelesen".to_owned());
        return;
    };
    // Feste Reihenfolge nach Name: bei Gleichstand gewinnt der erste.
    for (name, nr) in refs {
        top.consider(nr, || name);
    }
}

/// Vergibt die nächste Nummer über alle Checkouts und Remote-Refs; die Hinweise nennen den bestimmenden Checkout,
/// übersprungene Worktrees und fehlendes git.
pub fn next_ticket(root: &Path) -> Result<(String, Vec<String>), String> {
    let own = local_highest(root).map_err(|e| e.to_string())?;
    let mut top = Top {
        nr: own,
        from: "eigener Checkout".to_owned(),
    };
    let mut notes = Vec::new();
    if !git_found() {
        notes.push("Warnung: git nicht gefunden, Nummer nur aus dem eigenen Checkout".to_owned());
    } else if !is_repo_root(root) {
        notes.push("Warnung: Ordner ist keine Repo-Wurzel, Worktrees und Refs nicht gelesen".to_owned());
    } else {
        look_around(root, &mut top, &mut notes);
    }
    let nr = format!("B-{:03}", top.nr.saturating_add(1));
    notes.push(format!(
        "{nr} vergeben, höchste bekannte Nummer B-{:03}: {}",
        top.nr, top.from
    ));
    Ok((nr, notes))
}
