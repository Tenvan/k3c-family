//! Liegt ein Ordner in einem Git-Checkout? Ohne `.git` scheitert jeder git-Aufruf dort ohnehin; die Prüfung spart den
//! Prozess (unter Windows samt Virenscanner gut 100 ms, B-830).

use std::path::Path;

/// `.git` (Ordner oder Datei) in `dir` oder darüber. Im Zweifel (Pfad nicht auflösbar) true.
pub fn in_repo(dir: &Path) -> bool {
    let Ok(absolute) = std::path::absolute(dir) else {
        return true;
    };
    absolute.ancestors().any(has_git_entry)
}

/// `dir` selbst trägt `.git` (Wurzel eines Repos oder Worktrees).
pub fn has_git_entry(dir: &Path) -> bool {
    dir.join(".git").exists()
}
