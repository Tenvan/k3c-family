//! Was die Worktrees an der Planung geändert haben (Go `wtdiff.go`).

use std::collections::{BTreeMap, BTreeSet};
use std::path::{Path, PathBuf};
use std::time::{Duration, Instant};

use serde::Serialize;

use super::gitrepo::git_out;
use super::mergebase::{load_at, load_with_archive, merge_base};
use super::planning::{Data, Session, Sprint, Ticket};
use super::projects::Project;
use super::ticketnr::same_path;
use super::wtflight::{Bases, Diffs, flight_for};

const LIST_TIMEOUT: Duration = Duration::from_secs(3);

const NEW: &str = "neu";
const CHANGED: &str = "geändert";
const REMOVED: &str = "entfernt";

/// Ein Unterschied eines Worktrees zur Wurzel: Art, Objekt (Projekt, Sprint, Session, Ticket), seine ID und ein
/// kurzer Text („Status aktiv → ruht“).
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize)]
pub struct Change {
    /// neu | geändert | entfernt
    pub kind: String,
    pub item: String,
    pub id: String,
    pub text: String,
    /// Projekt und Sprint der neuen Einträge (Sprint, Ticket), damit die Ansicht sie unter ihrem Projekt zeigt.
    #[serde(skip_serializing_if = "String::is_empty")]
    pub project: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    pub sprint: String,
}

fn change(kind: &str, item: &str, id: &str, text: String) -> Change {
    Change {
        kind: kind.to_owned(),
        item: item.to_owned(),
        id: id.to_owned(),
        text,
        ..Change::default()
    }
}

/// Die Planungsunterschiede eines Worktrees zur Wurzel.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize)]
pub struct WorktreeDiff {
    pub branch: String,
    pub path: String,
    pub changes: Vec<Change>,
    #[serde(skip_serializing_if = "String::is_empty")]
    pub error: String,
}

#[derive(Debug, Default, PartialEq, Eq)]
pub(super) struct Worktree {
    pub(super) path: PathBuf,
    pub(super) branch: String,
    pub(super) head: String,
}

/// Ordner und Branch jedes Worktrees außer der Wurzel selbst.
fn worktree_list(root: &Path) -> Result<Vec<Worktree>, String> {
    let out = git_out(root, &["worktree", "list", "--porcelain"], LIST_TIMEOUT)
        .map_err(|e| format!("git worktree list: {e}"))?;
    Ok(parse_worktree_paths(&out, root))
}

pub(super) fn parse_worktree_paths(porcelain: &str, root: &Path) -> Vec<Worktree> {
    let mut list: Vec<Worktree> = Vec::new();
    for line in porcelain.split('\n').map(str::trim) {
        if let Some(path) = line.strip_prefix("worktree ") {
            list.push(Worktree {
                path: Path::new(path).components().collect(),
                ..Worktree::default()
            });
            continue;
        }
        let Some(current) = list.last_mut() else {
            continue;
        };
        if let Some(head) = line.strip_prefix("HEAD ") {
            head.clone_into(&mut current.head);
        } else if let Some(branch) = line.strip_prefix("branch refs/heads/") {
            branch.clone_into(&mut current.branch);
        }
    }
    list.retain(|w| !same_path(&w.path, root));
    list
}

/// Je Worktree, was er an der Planung gegenüber der Merge-Base mit der Wurzel geändert hat (Commits und
/// Arbeitskopie); der Vorsprung der Wurzel zählt nicht mit. Nur lesend.
///
/// Gleichzeitige Aufrufe teilen sich einen Lauf, die Stände der Merge-Bases bleiben zwischen den Läufen im Cache:
/// Die Oberfläche ruft bei jedem Ereignis und jedem Fokus.
pub fn check_worktrees(root: &Path) -> Diffs {
    let arrived = Instant::now();
    flight_for(root).run(arrived, |bases| run_check(root, bases))
}

fn run_check(root: &Path, bases: &mut Bases) -> Diffs {
    let diffs = worktree_list(root)?.into_iter().map(|worktree| {
        let (changes, error) = match diff_worktree(root, &worktree, bases) {
            Ok(changes) => (changes, String::new()),
            Err(e) => (Vec::new(), e),
        };
        WorktreeDiff {
            branch: worktree.branch,
            path: worktree.path.to_string_lossy().into_owned(),
            changes,
            error,
        }
    });
    Ok(diffs.collect())
}

fn diff_worktree(root: &Path, worktree: &Worktree, bases: &mut Bases) -> Result<Vec<Change>, String> {
    let base_rev = merge_base(root, &worktree.head)?;
    if !bases.contains_key(&base_rev) {
        bases.insert(base_rev.clone(), load_at(root, &base_rev)?);
    }
    let current = load_with_archive(&worktree.path)?;
    let empty = Data::default();
    Ok(diff_data(bases.get(&base_rev).unwrap_or(&empty), &current))
}

pub(super) fn diff_data(base: &Data, wt: &Data) -> Vec<Change> {
    let mut changes = diff_by_id(
        "Projekt",
        &base.projects,
        &wt.projects,
        |p: &Project| (&p.id, &p.title),
        |old, p| {
            let mut out = changed("Projekt", &p.id, "Status", &old.status, &p.status);
            out.extend(changed("Projekt", &p.id, "Rang", &old.rang, &p.rang));
            out
        },
    );
    let sprints = diff_by_id(
        "Sprint",
        &base.sprints,
        &wt.sprints,
        |s: &Sprint| (&s.id, &s.title),
        |old, s| {
            let mut out = changed("Sprint", &s.id, "Status", &old.status, &s.status);
            out.extend(changed("Sprint", &s.id, "Projekt", &old.project, &s.project));
            out.extend(diff_sessions(&old.sessions, &s.sessions));
            out
        },
    );
    changes.extend(with_new_sessions(sprints, &wt.sprints));
    changes.extend(diff_by_id(
        "Ticket",
        &base.tickets,
        &wt.tickets,
        |t: &Ticket| (&t.nr, &t.title),
        |old, t| {
            let mut out = changed("Ticket", &t.nr, "Status", &old.status, &t.status);
            out.extend(changed("Ticket", &t.nr, "Prio", &old.prio, &t.prio));
            out.extend(changed("Ticket", &t.nr, "Sprint", &old.sprint, &t.sprint));
            out
        },
    ));
    with_placement(changes, wt)
}

/// Hängt hinter jeden neuen Sprint seine Sessions als neu an (B-484); die Wurzel kennt keine davon.
fn with_new_sessions(changes: Vec<Change>, sprints: &[Sprint]) -> Vec<Change> {
    let mut out = Vec::with_capacity(changes.len());
    for found in changes {
        let added = (found.kind == NEW).then(|| found.id.clone());
        out.push(found);
        let Some(id) = added else {
            continue;
        };
        for sprint in sprints.iter().filter(|s| s.id == id) {
            out.extend(diff_sessions(&[], &sprint.sessions));
        }
    }
    out
}

/// Trägt bei neuen Sprints und Tickets Projekt (und Sprint des Tickets) ein, wie der Worktree sie führt.
fn with_placement(mut changes: Vec<Change>, wt: &Data) -> Vec<Change> {
    for found in changes.iter_mut().filter(|c| c.kind == NEW) {
        match found.item.as_str() {
            "Sprint" => {
                for sprint in wt.sprints.iter().filter(|s| s.id == found.id) {
                    found.project.clone_from(&sprint.project);
                }
            }
            "Ticket" => {
                for ticket in wt.tickets.iter().filter(|t| t.nr == found.id) {
                    found.project.clone_from(&ticket.project);
                    found.sprint.clone_from(&ticket.sprint);
                }
            }
            _ => {}
        }
    }
    changes
}

/// Vergleicht zwei Listen über die ID: in Reihenfolge des Worktrees neu oder die Feldänderungen aus `fields`, danach
/// die in `base` vorhandenen, im Worktree fehlenden Einträge als entfernt.
fn diff_by_id<T>(
    item: &str,
    base: &[T],
    wt: &[T],
    key: impl Fn(&T) -> (&String, &String),
    fields: impl Fn(&T, &T) -> Vec<Change>,
) -> Vec<Change> {
    let old: BTreeMap<&String, &T> = base.iter().map(|b| (key(b).0, b)).collect();
    let mut seen = BTreeSet::new();
    let mut changes = Vec::new();
    for now in wt {
        let (id, title) = key(now);
        seen.insert(id);
        match old.get(id) {
            Some(before) => changes.extend(fields(before, now)),
            None => changes.push(change(NEW, item, id, title.clone())),
        }
    }
    for before in base {
        let (id, title) = key(before);
        if !seen.contains(id) {
            changes.push(change(REMOVED, item, id, title.clone()));
        }
    }
    changes
}

/// Eine Änderung „name a → b“, wenn sich das Feld unterscheidet, sonst nichts.
fn changed(item: &str, id: &str, name: &str, a: &str, b: &str) -> Vec<Change> {
    if a == b {
        return Vec::new();
    }
    vec![change(CHANGED, item, id, format!("{name} {a} → {b}"))]
}

fn diff_sessions(old: &[Session], now: &[Session]) -> Vec<Change> {
    let before: BTreeMap<&String, &Session> = old.iter().map(|s| (&s.nr, s)).collect();
    let mut have = BTreeSet::new();
    let mut changes = Vec::new();
    for session in now {
        have.insert(&session.nr);
        match before.get(&session.nr) {
            None => changes.push(change(NEW, "Session", &session.nr, session.titel.clone())),
            Some(o) if o.status != session.status => {
                let text = format!("Status {} → {}", o.status, session.status);
                changes.push(change(CHANGED, "Session", &session.nr, text));
            }
            Some(_) => {}
        }
    }
    for session in old.iter().filter(|s| !have.contains(&s.nr)) {
        changes.push(change(REMOVED, "Session", &session.nr, session.titel.clone()));
    }
    changes
}
