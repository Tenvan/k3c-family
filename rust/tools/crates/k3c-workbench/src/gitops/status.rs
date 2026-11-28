//! Stand des Checkouts (Port von `status.go`).

use std::collections::HashSet;
use std::path::PathBuf;
use std::time::UNIX_EPOCH;

use serde::Serialize;

use super::repo::{BASE, BASE_REMOTE, MAIN, Repo, lines, protected};

/// Eine Datei im Working Tree.
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize)]
pub struct Change {
    pub path: String,
    /// M, A, D, R, C, U (Konflikt), ? (neu, ungetrackt).
    pub code: String,
    /// Im Index.
    pub staged: bool,
}

/// Ein Eintrag von `git stash list`.
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize)]
pub struct Stash {
    /// `stash@{0}`.
    #[serde(rename = "ref")]
    pub name: String,
    /// „vor 2 Stunden“.
    pub age: String,
    pub subject: String,
}

/// Ein weiterer Worktree (nicht die Wurzel).
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize)]
pub struct Worktree {
    pub path: String,
    /// Leer bei detached HEAD.
    pub branch: String,
    /// Kurzer Hash.
    pub head: String,
}

/// Stand des Checkouts für die Statuskarte.
#[derive(Clone, Debug, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Status {
    /// Leer bei detached HEAD.
    pub branch: String,
    pub upstream: String,
    pub upstream_gone: bool,
    /// Zum Upstream.
    pub ahead: i64,
    /// Zum Upstream.
    pub behind: i64,
    pub base_ahead: i64,
    pub base_behind: i64,
    /// main hinter origin/main.
    pub main_behind: i64,
    pub changes: Vec<Change>,
    pub stashes: Vec<Stash>,
    pub worktrees: Vec<Worktree>,
    /// Unix-Millisekunden des letzten Fetch, 0 = unbekannt.
    pub fetched_at: i64,
    pub advice: Vec<String>,
}

impl Repo {
    /// Liest den Stand des Checkouts.
    pub async fn status(&self) -> Result<Status, String> {
        let out = self
            .git(&["status", "--porcelain=v2", "--branch", "--untracked-files=all"])
            .await?;
        let mut s = parse_status(&out);
        (s.base_ahead, s.base_behind) = self.ahead_behind("HEAD", BASE_REMOTE).await.unwrap_or_default();
        s.main_behind = self.ahead_behind(MAIN, BASE_REMOTE).await.unwrap_or_default().1;
        if let Ok(out) = self.git(&["stash", "list", "--format=%gd%x09%cr%x09%s"]).await {
            s.stashes = parse_stashes(&out);
        }
        if let Ok(out) = self.git(&["worktree", "list", "--porcelain"]).await {
            s.worktrees = parse_worktrees(&out);
        }
        s.fetched_at = self.fetched_at().await;
        s.advice = advise(&s);
        Ok(s)
    }

    /// Commits in `a`, die `b` fehlen, und umgekehrt. `None`, wenn git scheitert (Ref fehlt, Frist, Sperre): das darf
    /// nie als „nichts voraus“ gelten.
    pub(super) async fn ahead_behind(&self, a: &str, b: &str) -> Option<(i64, i64)> {
        // Ein Ref mit führendem „-“ wäre eine Option.
        if a.starts_with('-') || b.starts_with('-') {
            return None;
        }
        let range = format!("{a}...{b}");
        let out = self.git(&["rev-list", "--left-right", "--count", &range]).await.ok()?;
        let mut fields = out.split_whitespace();
        let (x, y) = (fields.next()?, fields.next()?);
        if fields.next().is_some() {
            return None;
        }
        Some((x.parse().ok()?, y.parse().ok()?))
    }

    /// Der gemeinsame Git-Ordner (bei Worktrees der der Wurzel), absolut zur Wurzel.
    pub(super) async fn read_common_dir(&self) -> Result<PathBuf, String> {
        let dir = PathBuf::from(self.git(&["rev-parse", "--git-common-dir"]).await?);
        Ok(if dir.is_absolute() { dir } else { self.0.root.join(dir) })
    }

    async fn fetched_at(&self) -> i64 {
        let Ok(dir) = self.read_common_dir().await else {
            return 0;
        };
        std::fs::metadata(dir.join("FETCH_HEAD"))
            .and_then(|m| m.modified())
            .ok()
            .and_then(|t| t.duration_since(UNIX_EPOCH).ok())
            .map_or(0, |d| i64::try_from(d.as_millis()).unwrap_or_default())
    }
}

/// Wertet `git status --porcelain=v2 --branch` aus.
pub(super) fn parse_status(out: &str) -> Status {
    let mut s = Status::default();
    let mut has_ab = false;
    for l in lines(out) {
        if let Some(head) = l.strip_prefix("# branch.head ") {
            if head != "(detached)" {
                s.branch = head.to_owned();
            }
        } else if let Some(upstream) = l.strip_prefix("# branch.upstream ") {
            s.upstream = upstream.to_owned();
        } else if let Some(ab) = l.strip_prefix("# branch.ab ") {
            has_ab = true;
            let mut fields = ab.split_whitespace();
            let mut count = |sign| fields.next().and_then(|f| f.strip_prefix(sign)?.parse().ok());
            s.ahead = count('+').unwrap_or_default();
            s.behind = count('-').unwrap_or_default();
        } else if let Some(path) = l.strip_prefix("? ") {
            s.changes.push(change(path, "?", false));
        } else if matches!(l.as_bytes(), [b'1' | b'2' | b'u', b' ', _, ..]) {
            s.changes.extend(parse_entry(l));
        }
    }
    s.upstream_gone = !s.upstream.is_empty() && !has_ab;
    s
}

fn change(path: &str, code: &str, staged: bool) -> Change {
    Change {
        path: path.to_owned(),
        code: code.to_owned(),
        staged,
    }
}

/// Eine Zeile 1 (geändert), 2 (umbenannt/kopiert) oder u (Konflikt). Eine Datei mit Änderungen im Index (X) und im
/// Working Tree (Y) liefert zwei Einträge: einmal gestaged, einmal nicht.
fn parse_entry(l: &str) -> Vec<Change> {
    // Felder vor dem Pfad.
    let fields = match l.as_bytes().first() {
        Some(b'1') => 8,
        Some(b'2') => 9,
        _ => 10,
    };
    let parts: Vec<&str> = l.splitn(fields + 1, ' ').collect();
    let mut path = parts.last().copied().unwrap_or_default();
    if l.starts_with('2') {
        // Neuer Pfad, dann Tab, dann alter.
        path = path.split('\t').next().unwrap_or_default();
    }
    if l.starts_with('u') {
        return vec![change(path, "U", false)];
    }
    let xy = parts.get(1).copied().unwrap_or_default();
    xy.chars()
        .take(2)
        .zip([true, false])
        .filter(|(code, _)| *code != '.')
        .map(|(code, staged)| change(path, &code.to_string(), staged))
        .collect()
}

/// Zählt Dateien, nicht Einträge (gestaged und ungestaged zählt einmal).
fn changed_files(changes: &[Change]) -> usize {
    changes.iter().map(|c| c.path.as_str()).collect::<HashSet<_>>().len()
}

fn parse_stashes(out: &str) -> Vec<Stash> {
    lines(out)
        .filter_map(|l| {
            let mut f = l.splitn(3, '\t');
            Some(Stash {
                name: f.next()?.to_owned(),
                age: f.next()?.to_owned(),
                subject: f.next()?.to_owned(),
            })
        })
        .collect()
}

/// Liest `git worktree list --porcelain`; der erste Block ist die Wurzel und fällt weg.
pub(super) fn parse_worktrees(out: &str) -> Vec<Worktree> {
    let mut list: Vec<Worktree> = Vec::new();
    let mut first = true;
    for l in out.split('\n').map(|l| l.trim_end_matches('\r')) {
        if let Some(path) = l.strip_prefix("worktree ") {
            if !first {
                list.push(Worktree {
                    path: path.to_owned(),
                    ..Worktree::default()
                });
            }
            first = false;
        } else if let (Some(cur), Some(head)) = (list.last_mut(), l.strip_prefix("HEAD ")) {
            cur.head = short_hash(head).to_owned();
        } else if let (Some(cur), Some(branch)) = (list.last_mut(), l.strip_prefix("branch ")) {
            cur.branch = branch.strip_prefix("refs/heads/").unwrap_or(branch).to_owned();
        }
    }
    list
}

pub(super) fn short_hash(h: &str) -> &str {
    h.get(..8).unwrap_or(h)
}

/// Die nächsten Handgriffe aus dem Stand, passend zu „ein PR je Sprint gegen main“.
pub(super) fn advise(s: &Status) -> Vec<String> {
    let mut a = Vec::new();
    if !s.changes.is_empty() {
        let n = changed_files(&s.changes);
        a.push(format!("{n} geänderte Dateien committen oder stashen"));
    }
    if s.branch.is_empty() {
        a.push("Detached HEAD: auf einen Branch wechseln".to_owned());
    } else if s.upstream_gone {
        a.push("Remote-Branch gelöscht (PR gemergt?): auf main wechseln und den Branch aufräumen".to_owned());
    } else if s.behind > 0 {
        a.push(format!("Pull: {} Commits auf dem Remote", s.behind));
    } else if s.ahead > 0 {
        a.push(format!("Push: {} Commits nicht gepusht", s.ahead));
    } else if s.upstream.is_empty() && !protected(&s.branch) && s.base_ahead > 0 {
        a.push("Push mit Upstream, dann PR gegen main öffnen".to_owned());
    }
    if s.branch == BASE && s.base_behind > 0 && s.behind == 0 {
        a.push(format!(
            "main liegt {} hinter origin/main: Fetch und Pull",
            s.base_behind
        ));
    }
    if s.main_behind > 0 {
        a.push(format!(
            "main liegt {} hinter origin/main (Fast-Forward macht Ralf)",
            s.main_behind
        ));
    }
    a
}
