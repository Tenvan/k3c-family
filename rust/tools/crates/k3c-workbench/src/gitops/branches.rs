//! Lokale und nur entfernte Branches mit Zustand (Port von `branches.go`).

use std::collections::HashSet;
use std::sync::Arc;

use serde::Serialize;
use tokio::sync::Semaphore;
use tokio::task::JoinSet;

use super::repo::{BASE_REMOTE, Repo, lines, protected};
use super::status::parse_worktrees;

/// Zustände eines Branches gegenüber main: ausgecheckt in der Wurzel.
pub const STATE_CURRENT: &str = "aktuell";
/// Ausgecheckt in einem anderen Worktree.
pub const STATE_WORKTREE: &str = "worktree";
/// main.
pub const STATE_BASE: &str = "basis";
/// Vorfahre von origin/main.
pub const STATE_MERGED: &str = "gemergt";
/// Inhalt per Squash-Merge in origin/main.
pub const STATE_SQUASH: &str = "squash";
/// Eigene Commits, die main fehlen.
pub const STATE_OPEN: &str = "ungemergt";

/// Remote-Zustände eines Branches.
pub const TRACK_SYNC: &str = "synchron";
pub const TRACK_AHEAD: &str = "voraus";
pub const TRACK_BEHIND: &str = "zurück";
pub const TRACK_BOTH: &str = "auseinander";
pub const TRACK_GONE: &str = "gone";
/// Kein Upstream.
pub const TRACK_NONE: &str = "fehlt";
pub const TRACK_REMOTE: &str = "nur remote";

/// Gleichzeitige git-Aufrufe beim Lesen und Prüfen.
pub(super) const PARALLEL: usize = 6;

/// Ein lokaler oder nur entfernter Branch.
#[derive(Clone, Debug, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Branch {
    /// Ohne `origin/` auch bei nur entfernten.
    pub name: String,
    pub remote: bool,
    pub upstream: String,
    pub track: &'static str,
    pub ahead: i64,
    pub behind: i64,
    pub base_ahead: i64,
    pub base_behind: i64,
    pub state: &'static str,
    /// Merge-Zustand sicher (Vorfahre oder geprüft); falsch: ungemergt nur vermutet.
    pub checked: bool,
    /// Gemergter PR, an dem der Branch hängt (Check).
    pub pr: i64,
    pub sha: String,
    /// Unix-Sekunden des letzten Commits.
    pub date: i64,
    pub subject: String,
    pub deletable: bool,
    /// Warum nicht löschbar.
    pub reason: &'static str,
}

#[derive(Debug, Default)]
pub(super) struct Ref {
    pub(super) name: String,
    pub(super) upstream: String,
    pub(super) track: String,
    pub(super) sha: String,
    pub(super) subject: String,
    pub(super) date: i64,
    pub(super) remote: bool,
}

/// Was alle Branches gemeinsam haben.
struct Around {
    /// Hash von origin/main, leer ohne.
    base: String,
    current: String,
    worktrees: HashSet<String>,
}

impl Repo {
    /// Alle lokalen und nur entfernten Branches mit Zustand; der Merge-Zustand ist je Commit-Paar gemerkt.
    pub async fn branches(&self) -> Result<Vec<Branch>, String> {
        let format = "--format=%(refname)%09%(upstream:short)%09%(upstream:track)%09%(objectname)%09\
                      %(committerdate:unix)%09%(subject)";
        let out = self
            .git(&["for-each-ref", format, "refs/heads", "refs/remotes/origin"])
            .await?;
        let refs = parse_refs(&out);
        // Ohne aktuellen Branch, Worktrees oder Basis wäre der Schutz in `deletable` blind: dann lieber gar keine Liste.
        let current = self.git(&["branch", "--show-current"]).await?;
        let listed = self.git(&["worktree", "list", "--porcelain"]).await?;
        let worktrees = parse_worktrees(&listed)
            .into_iter()
            .map(|w| w.branch)
            .filter(|b| !b.is_empty())
            .collect();
        // Leer: kein origin/main, alles bleibt ungemergt.
        let base = self
            .git(&["rev-parse", "--verify", "--quiet", BASE_REMOTE])
            .await
            .unwrap_or_default();
        let around = Arc::new(Around {
            base,
            current,
            worktrees,
        });
        let gate = Arc::new(Semaphore::new(PARALLEL));
        let mut running = JoinSet::new();
        for (index, f) in refs.into_iter().enumerate() {
            let (repo, around, gate) = (self.clone(), Arc::clone(&around), Arc::clone(&gate));
            running.spawn(async move {
                let _turn = gate.acquire_owned().await;
                (index, repo.branch(f, &around).await)
            });
        }
        let mut list = Vec::new();
        while let Some(done) = running.join_next().await {
            list.push(done.map_err(|e| format!("Branch lesen: {e}"))?);
        }
        list.sort_by_key(|(index, _)| *index);
        Ok(list.into_iter().map(|(_, b)| b).collect())
    }

    async fn branch(&self, f: Ref, around: &Around) -> Branch {
        let (track, ahead, behind) = parse_track(&f);
        let target = if f.remote {
            format!("origin/{}", f.name)
        } else {
            f.name.clone()
        };
        let state = if protected(&f.name) {
            STATE_BASE
        } else if !f.remote && f.name == around.current {
            STATE_CURRENT
        } else if !f.remote && around.worktrees.contains(&f.name) {
            STATE_WORKTREE
        } else {
            ""
        };
        let measured = self.ahead_behind(&target, BASE_REMOTE).await;
        let mut b = Branch {
            name: f.name,
            remote: f.remote,
            upstream: f.upstream,
            track,
            ahead,
            behind,
            state,
            sha: f.sha,
            date: f.date,
            subject: f.subject,
            ..Branch::default()
        };
        (b.base_ahead, b.base_behind) = measured.unwrap_or_default();
        if b.state.is_empty() {
            if measured.is_none() || around.base.is_empty() {
                b.state = STATE_OPEN;
                b.reason = "Stand zu origin/main unbekannt";
                return b;
            }
            (b.state, b.pr, b.checked) = self.merged_state(&b.sha, &around.base, b.base_ahead);
        }
        (b.deletable, b.reason) = deletable(&b);
        b
    }
}

/// Nur lokale Branches, die main schon enthält und die nirgends ausgecheckt sind.
fn deletable(b: &Branch) -> (bool, &'static str) {
    let reason = if b.remote {
        "nur remote"
    } else if b.state == STATE_BASE {
        "main bleibt"
    } else if b.state == STATE_CURRENT {
        "ausgecheckt"
    } else if b.state == STATE_WORKTREE {
        "in einem Worktree ausgecheckt"
    } else if b.state == STATE_OPEN && !b.checked {
        "ungeprüft: erst „Prüfen“"
    } else if b.state == STATE_OPEN {
        "nicht in main"
    } else {
        return (true, "");
    };
    (false, reason)
}

/// Liest `for-each-ref`; origin/HEAD und entfernte Branches mit lokalem Gegenstück fallen weg.
pub(super) fn parse_refs(out: &str) -> Vec<Ref> {
    let (mut local, mut remote) = (Vec::new(), Vec::new());
    let mut tracked = HashSet::new();
    for l in lines(out) {
        let fields: Vec<&str> = l.splitn(6, '\t').collect();
        let [full, upstream, track, sha, date, subject] = fields[..] else {
            continue;
        };
        let mut x = Ref {
            upstream: upstream.to_owned(),
            track: track.to_owned(),
            sha: sha.to_owned(),
            date: date.parse().unwrap_or_default(),
            subject: subject.to_owned(),
            ..Ref::default()
        };
        if let Some(name) = full.strip_prefix("refs/heads/") {
            x.name = name.to_owned();
            tracked.insert(x.upstream.clone());
            local.push(x);
        } else if let Some(name) = full.strip_prefix("refs/remotes/origin/").filter(|n| *n != "HEAD") {
            x.name = name.to_owned();
            x.remote = true;
            remote.push(x);
        }
    }
    local.extend(
        remote
            .into_iter()
            .filter(|x| !tracked.contains(&format!("origin/{}", x.name))),
    );
    local
}

/// Deutet `%(upstream:track)`: "", "[gone]", "[ahead 1]", "[behind 2]", "[ahead 1, behind 2]".
pub(super) fn parse_track(f: &Ref) -> (&'static str, i64, i64) {
    if f.remote {
        return (TRACK_REMOTE, 0, 0);
    }
    if f.upstream.is_empty() {
        return (TRACK_NONE, 0, 0);
    }
    let track = f.track.trim_matches(['[', ']']);
    if track == "gone" {
        return (TRACK_GONE, 0, 0);
    }
    let count = |prefix: &str| {
        track
            .split(", ")
            .find_map(|part| part.strip_prefix(prefix))
            .and_then(|n| n.parse::<i64>().ok())
            .unwrap_or_default()
    };
    let (ahead, behind) = (count("ahead "), count("behind "));
    match (ahead > 0, behind > 0) {
        (true, true) => (TRACK_BOTH, ahead, behind),
        (true, false) => (TRACK_AHEAD, ahead, 0),
        (false, true) => (TRACK_BEHIND, 0, behind),
        (false, false) => (TRACK_SYNC, 0, 0),
    }
}
