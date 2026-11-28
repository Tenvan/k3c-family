//! Liest und pflegt das Git-Repo der Workbench für die Seite Git (B-580, Port von `internal/gitops`): Status, Branches
//! mit Squash-Erkennung, geänderte Dateien, Fetch, Pull, Push, Stash und das Löschen gemergter Branches. git läuft als
//! Kindprozess ohne Shell mit festen Argumenten; Namen aus der Oberfläche prüft [`valid_branch`], bevor git sie sieht.

mod actions;
mod branches;
mod files;
mod merged;
mod repo;
mod stamp;
mod status;

#[cfg(test)]
pub(crate) mod fixture_tests;
#[cfg(test)]
mod parse_tests;
#[cfg(test)]
mod repo_tests;
#[cfg(test)]
mod safety_tests;

pub use actions::{ACT_FETCH, ACT_PULL, ACT_PUSH, ACT_STASH, ACT_STASH_POP};
pub use branches::{
    Branch, STATE_BASE, STATE_CURRENT, STATE_MERGED, STATE_OPEN, STATE_SQUASH, STATE_WORKTREE, TRACK_AHEAD,
    TRACK_BEHIND, TRACK_BOTH, TRACK_GONE, TRACK_NONE, TRACK_REMOTE, TRACK_SYNC,
};
pub use files::{DiffFile, Files};
pub use merged::MergedPr;
pub use repo::{BASE, EnvChange, MAIN, Repo, RunFuture, Runner, exec_runner, valid_branch};
pub use status::{Change, Stash, Status, Worktree};
