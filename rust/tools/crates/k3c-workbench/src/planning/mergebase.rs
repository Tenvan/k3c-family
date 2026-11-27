//! Die Planung eines Commits (Go `mergebase.go`). Stand RU69.11: `run_git` und `load_at` für das origin-Overlay;
//! `mergeBase` für den Worktree-Abgleich folgt in RU69.12.

use std::fs;
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicU64, Ordering};
use std::time::Duration;

use super::gitrepo::git_raw;
use super::list::archived;
use super::planning::{Data, load};
use super::store::STATES;
use super::untar::untar;

const TIMEOUT: Duration = Duration::from_secs(15);

/// stdout von `git -C <root> <args>`; Fehler enthalten die Meldung von git.
pub(super) fn run_git(root: &Path, args: &[&str]) -> Result<Vec<u8>, String> {
    let name = args.first().copied().unwrap_or_default();
    let (status, out, err) = git_raw(root, args, TIMEOUT).map_err(|e| format!("git {name}: {e} "))?;
    if status.success() {
        return Ok(out);
    }
    Err(format!(
        "git {name}: exit status {} {}",
        status.code().unwrap_or(-1),
        String::from_utf8_lossy(&err).trim()
    ))
}

/// Wegwerf-Ordner unter `<root>/.work/tmp` (Regel „Schreiben nur im Repo“); er verschwindet beim Fallenlassen.
struct Scratch(PathBuf);

impl Scratch {
    fn new(root: &Path) -> Result<Scratch, String> {
        static NUMBER: AtomicU64 = AtomicU64::new(0);
        let base = root.join(".work").join("tmp");
        fs::create_dir_all(&base).map_err(|e| e.to_string())?;
        loop {
            let number = NUMBER.fetch_add(1, Ordering::Relaxed);
            let dir = base.join(format!("k3c-planbase-{}-{number}", std::process::id()));
            match fs::create_dir(&dir) {
                Ok(()) => return Ok(Scratch(dir)),
                Err(e) if e.kind() == std::io::ErrorKind::AlreadyExists => {}
                Err(e) => return Err(e.to_string()),
            }
        }
    }
}

impl Drop for Scratch {
    fn drop(&mut self) {
        // Bleibt er liegen, räumt `.work` ihn auf.
        let _ = fs::remove_dir_all(&self.0);
    }
}

/// Die Planung, wie sie im Commit `rev` stand: docs/ kommt als tar aus git in einen Wegwerf-Ordner und wird dort wie
/// eine Wurzel geladen. Die Arbeitskopie bleibt unberührt.
pub(super) fn load_at(root: &Path, rev: &str) -> Result<Data, String> {
    let archive = run_git(root, &["archive", "--format=tar", rev, "docs"])?;
    let scratch = Scratch::new(root)?;
    untar(&archive, &scratch.0)?;
    for state in STATES {
        // git kennt keine leeren Ordner; `load` erwartet sie.
        fs::create_dir_all(scratch.0.join("docs").join("sprints").join(state)).map_err(|e| e.to_string())?;
    }
    load_with_archive(&scratch.0)
}

/// `load` plus die archivierten Tickets: Ohne sie wirkt ein abgeschlossenes Ticket im Vergleich wie ein
/// verschwundenes.
fn load_with_archive(root: &Path) -> Result<Data, String> {
    let mut data = load(root)?;
    data.tickets.extend(archived(root));
    Ok(data)
}
