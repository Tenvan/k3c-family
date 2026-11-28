//! „Steckt der Branch in main?“ (Port von `merged.go`).

use std::collections::HashSet;
use std::sync::{Arc, PoisonError};

use tokio::sync::Semaphore;
use tokio::task::JoinSet;

use super::branches::{Branch, PARALLEL, STATE_MERGED, STATE_OPEN, STATE_SQUASH};
use super::repo::{BASE_REMOTE, Repo};

/// Ein gemergter PR (aus gh); `head` ist der Branch-Name, `sha` der letzte Commit des PRs.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct MergedPr {
    pub number: i64,
    pub head: String,
    pub sha: String,
}

/// Ergebnis der Prüfung, gemerkt je Commit-Paar (Branch, main).
#[derive(Clone, Copy, Debug)]
pub(super) struct Verdict {
    /// [`STATE_SQUASH`] oder [`STATE_OPEN`].
    state: &'static str,
    /// Nummer des gemergten PRs, 0 = über git erkannt oder ungemergt.
    pr: i64,
}

fn memo_key(sha: &str, base: &str) -> String {
    format!("{sha} {base}")
}

impl Repo {
    /// Vorfahre von main heißt gemergt; sonst gilt das gemerkte Ergebnis von [`Repo::check`]. Ohne Prüfung bleibt der
    /// Branch ungemergt und ungeprüft, die teure Prüfung läuft nur auf Knopfdruck. Der Aufrufer stellt sicher, dass
    /// `base_ahead` gemessen ist.
    pub(super) fn merged_state(&self, sha: &str, base: &str, base_ahead: i64) -> (&'static str, i64, bool) {
        if base.is_empty() || sha.is_empty() {
            return (STATE_OPEN, 0, false);
        }
        if base_ahead == 0 {
            return (STATE_MERGED, 0, true);
        }
        let memo = self.0.memo.lock().unwrap_or_else(PoisonError::into_inner);
        memo.get(&memo_key(sha, base))
            .map_or((STATE_OPEN, 0, false), |v| (v.state, v.pr, true))
    }

    /// Prüft die genannten lokalen Branches (leer = alle ungeprüften) gründlich und merkt das Ergebnis:
    /// 1. ein gemergter PR, dessen Head der Branch ist oder enthält (der übliche Squash-Merge, auch Sammel-PRs),
    /// 2. ein Merge in main ändert nichts mehr (merge-tree),
    /// 3. der Netto-Diff des Branches ist als Commit in main (Squash-Patch, git cherry).
    pub async fn check(&self, names: &[String], prs: &[MergedPr]) -> Result<(), String> {
        let all = self.branches().await?;
        let base = self
            .git(&["rev-parse", "--verify", "--quiet", BASE_REMOTE])
            .await
            .unwrap_or_default();
        if base.is_empty() {
            return Ok(()); // ohne origin/main gibt es nichts zu prüfen
        }
        let want: HashSet<&str> = names.iter().map(String::as_str).collect();
        let prs: Arc<[MergedPr]> = prs.into();
        let gate = Arc::new(Semaphore::new(PARALLEL));
        let mut running = JoinSet::new();
        for b in all {
            let skip = b.remote
                || b.state != STATE_OPEN
                || (!names.is_empty() && !want.contains(b.name.as_str()))
                || (names.is_empty() && b.checked);
            if skip {
                continue;
            }
            let (repo, prs, gate, base) = (self.clone(), Arc::clone(&prs), Arc::clone(&gate), base.clone());
            running.spawn(async move {
                let _turn = gate.acquire_owned().await;
                // Nur sichere Antworten merken: eine Sperre oder Frist hieße sonst dauerhaft „ungemergt“.
                if let (verdict, true) = repo.verdict(&b, &prs).await {
                    let mut memo = repo.0.memo.lock().unwrap_or_else(PoisonError::into_inner);
                    memo.insert(memo_key(&b.sha, &base), verdict);
                }
            });
        }
        while let Some(done) = running.join_next().await {
            done.map_err(|e| format!("Branch prüfen: {e}"))?;
        }
        Ok(())
    }

    async fn verdict(&self, b: &Branch, prs: &[MergedPr]) -> (Verdict, bool) {
        let (pr, sure_pr) = self.pr_for(b, prs).await;
        if pr > 0 {
            return (
                Verdict {
                    state: STATE_SQUASH,
                    pr,
                },
                true,
            );
        }
        let (inside, sure_git) = self.squashed(&b.name).await;
        if inside {
            return (
                Verdict {
                    state: STATE_SQUASH,
                    pr: 0,
                },
                true,
            );
        }
        (
            Verdict {
                state: STATE_OPEN,
                pr: 0,
            },
            sure_pr && sure_git,
        )
    }

    /// Ein nach main gemergter PR mit dem Branch als Head, über den Upstream-Namen oder den eigenen Namen; der Branch
    /// muss der Head des PRs sein oder in ihm stecken. Das zweite Ergebnis ist falsch, wenn git nicht antworten konnte.
    async fn pr_for(&self, b: &Branch, prs: &[MergedPr]) -> (i64, bool) {
        let up = b.upstream.strip_prefix("origin/").unwrap_or(&b.upstream);
        let mut sure = true;
        for p in prs.iter().filter(|p| p.head == b.name || p.head == up) {
            if p.sha == b.sha {
                return (p.number, true);
            }
            // Werte von außen nie als Option an git.
            if p.sha.starts_with('-') || b.sha.starts_with('-') {
                continue;
            }
            match self.git(&["merge-base", "--is-ancestor", &b.sha, &p.sha]).await {
                Ok(_) => return (p.number, true),
                // Fehlt der Head-Commit lokal, ist das ein sicheres Nein; alles andere nicht.
                Err(e) if no_answer(&e) && !e.contains("Not a valid commit") => sure = false,
                Err(_) => {}
            }
        }
        (0, sure)
    }

    /// main enthält den Stand von `target`, wenn ein Merge nichts mehr ändert (merge-tree) oder ein Commit mit dem
    /// Netto-Diff des Branches für `git cherry` gleich einem Commit auf main ist (Squash-Merge). Bekannte Grenze: hat
    /// main die Änderung später zurückgenommen, gilt der Branch über merge-tree ebenfalls als enthalten. Das zweite
    /// Ergebnis ist falsch, wenn git nicht antworten konnte.
    pub(super) async fn squashed(&self, target: &str) -> (bool, bool) {
        if target.starts_with('-') {
            return (false, false);
        }
        match self.git(&["merge-tree", "--write-tree", BASE_REMOTE, target]).await {
            Err(e) if no_answer(&e) => return (false, false),
            Err(_) => {}
            Ok(tree) => {
                let base_tree = format!("{BASE_REMOTE}^{{tree}}");
                if let Ok(dev) = self.git(&["rev-parse", &base_tree]).await
                    && first_line(&tree) == dev
                {
                    return (true, true);
                }
            }
        }
        self.squash_patch(target)
            .await
            .map_or((false, false), |inside| (inside, true))
    }

    /// Ein Commit mit dem Baum des Branches auf der Merge-Basis ist für `git cherry` gleich einem Commit auf main, wenn
    /// ein Squash-Merge genau diese Änderung gebracht hat.
    async fn squash_patch(&self, target: &str) -> Result<bool, String> {
        let merge_base = self.git(&["merge-base", BASE_REMOTE, target]).await?;
        let tree = self.git(&["rev-parse", &format!("{target}^{{tree}}")]).await?;
        // Feste Identität: commit-tree verlangt eine, das Objekt bleibt unerreichbar und verschwindet beim nächsten gc.
        let probe = self
            .git(&[
                "-c",
                "user.name=k3c-workbench",
                "-c",
                "user.email=workbench@localhost",
                "commit-tree",
                &tree,
                "-p",
                &merge_base,
                "-m",
                "squash-probe",
            ])
            .await?;
        let out = self.git(&["cherry", BASE_REMOTE, &probe]).await?;
        Ok(out.starts_with('-'))
    }
}

/// git ist gescheitert, statt „nein“ zu sagen. `merge-base --is-ancestor` und `merge-tree` melden ein Nein (kein
/// Vorfahre, Konflikt) mit Exit 1 ohne Text auf stderr; daraus wird „…: exit status 1“.
fn no_answer(error: &str) -> bool {
    !error.ends_with("exit status 1")
}

fn first_line(s: &str) -> &str {
    s.split('\n').next().unwrap_or_default().trim()
}
