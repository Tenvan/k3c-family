//! Die Aktionen der Seite (Port von `actions.go`); jede läuft allein, nie mit Force.

use std::collections::HashMap;
use std::fmt;

use super::branches::{Branch, TRACK_GONE, TRACK_NONE};
use super::repo::{BASE_REMOTE, Repo, protected, valid_branch};
use super::status::short_hash;

pub const ACT_FETCH: &str = "fetch";
pub const ACT_PULL: &str = "pull";
pub const ACT_PUSH: &str = "push";
pub const ACT_STASH: &str = "stash";
pub const ACT_STASH_POP: &str = "stash-pop";

impl Repo {
    /// Führt eine Aktion aus und liefert einen Satz zum Ergebnis.
    pub async fn act(&self, action: &str) -> Result<String, String> {
        let _write = self.0.write.lock().await;
        match action {
            ACT_FETCH => {
                self.git(&["fetch", "--prune", "origin"]).await?;
                Ok("Fetch fertig".to_owned())
            }
            ACT_PULL => {
                self.git(&["pull", "--ff-only"]).await?;
                Ok("Pull fertig (Fast-Forward)".to_owned())
            }
            ACT_PUSH => self.push().await,
            ACT_STASH => {
                let message = format!("workbench {}", chrono::Local::now().format("%Y-%m-%d %H:%M"));
                self.git(&["stash", "push", "--include-untracked", "-m", &message])
                    .await?;
                Ok("Änderungen gestasht".to_owned())
            }
            ACT_STASH_POP => {
                self.git(&["stash", "pop"]).await?;
                Ok("Letzter Stash zurückgeholt".to_owned())
            }
            _ => Err(format!("unbekannte Aktion {action:?}")),
        }
    }

    /// Schiebt den aktuellen Branch; ohne Upstream mit -u nach origin. main nimmt die Seite nicht.
    async fn push(&self) -> Result<String, String> {
        let branch = self.git(&["branch", "--show-current"]).await?;
        if branch.is_empty() {
            return Err("detached HEAD: kein Branch zum Pushen".to_owned());
        }
        if protected(&branch) {
            return Err(format!("{branch} pusht die Seite nicht (nur über PR bzw. Release)"));
        }
        if !valid_branch(&branch) {
            return Err(format!("ungewöhnlicher Branch-Name {branch:?}"));
        }
        // Immer mit ausdrücklichem Ziel gleichen Namens: ein bloßes `git push` ginge an den Upstream, und der könnte
        // (mit push.default=upstream) origin/main sein.
        let refspec = format!("HEAD:refs/heads/{branch}");
        let upstream = ["rev-parse", "--abbrev-ref", "--symbolic-full-name", "@{upstream}"];
        if self.git(&upstream).await.is_ok() {
            self.git(&["push", "origin", &refspec]).await?;
            return Ok("Push fertig".to_owned());
        }
        self.git(&["push", "--set-upstream", "origin", &refspec]).await?;
        Ok(format!("Push fertig, Upstream origin/{branch} gesetzt"))
    }

    /// Löscht lokale Branches, die main schon enthält; `remote` löscht dazu den Branch auf origin, falls es ihn noch
    /// gibt. Die Prüfung läuft hier noch einmal, die Oberfläche könnte einen alten Stand zeigen.
    pub async fn delete(&self, names: &[String], remote: bool) -> Result<String, String> {
        if names.is_empty() {
            return Err("keine Branches gewählt".to_owned());
        }
        if let Some(bad) = names.iter().find(|n| !valid_branch(n)) {
            return Err(format!("ungültiger Branch-Name {bad:?}"));
        }
        let _write = self.0.write.lock().await;
        let local: HashMap<String, Branch> = self
            .branches()
            .await?
            .into_iter()
            .filter(|b| !b.remote)
            .map(|b| (b.name.clone(), b))
            .collect();
        let mut refused = Vec::new();
        let mut chosen = Vec::new();
        for n in names {
            match local.get(n) {
                None => refused.push(format!("{n}: kein lokaler Branch")),
                Some(b) if !b.deletable => refused.push(format!("{n}: {}", b.reason)),
                Some(b) => chosen.push(b),
            }
        }
        if !refused.is_empty() {
            return Err(format!("nicht gelöscht: {}", refused.join("; ")));
        }
        self.delete_all(&chosen, remote).await
    }

    /// Löscht erst die Remote-Branches (nur, wenn main auch deren Stand enthält), dann lokal. Scheitert ein
    /// Remote-Schritt, bleibt der lokale Branch stehen; die übrigen laufen weiter.
    async fn delete_all(&self, chosen: &[&Branch], remote: bool) -> Result<String, String> {
        let mut result = Deleted::default();
        for b in chosen {
            let n = &b.name;
            if remote && b.track != TRACK_GONE && b.track != TRACK_NONE {
                if let Err(e) = self.delete_remote(b).await {
                    result.failed.push(format!("{n}: {e}"));
                    continue;
                }
                result.remote += 1;
            }
            // update-ref mit erwartetem Hash statt branch -D: kam seit der Prüfung ein Commit dazu, scheitert das
            // Löschen. (-d prüft „gemergt“ nur gegen Vorfahren und lehnt Squash-Merges ab; geprüft hat `delete` selbst.)
            if let Err(e) = self
                .git(&["update-ref", "-d", &format!("refs/heads/{n}"), &b.sha])
                .await
            {
                result.failed.push(format!("{n}: {e}"));
                continue;
            }
            // Upstream-Eintrag; fehlt er, ist es gleich.
            let _ = self.git(&["config", "--remove-section", &format!("branch.{n}")]).await;
            result.local.push(format!("{n} ({})", short_hash(&b.sha)));
        }
        if result.failed.is_empty() {
            Ok(result.to_string())
        } else {
            Err(result.to_string())
        }
    }

    /// Löscht `origin/<upstream>`, wenn main dessen Stand enthält (Vorfahre oder Squash); Commits, die nur auf origin
    /// liegen (anderes Gerät, andere Session), bleiben so erhalten.
    async fn delete_remote(&self, b: &Branch) -> Result<(), String> {
        let up = b
            .upstream
            .strip_prefix("origin/")
            .filter(|up| valid_branch(up) && !protected(up))
            .ok_or_else(|| format!("upstream {:?} wird nicht gelöscht", b.upstream))?;
        let seen = self
            .git(&["rev-parse", "--verify", "--quiet", &b.upstream])
            .await
            .map_err(|_| "der Stand von origin ist unbekannt; nichts gelöscht".to_owned())?;
        // Über einen gemergten PR geprüft: origin darf nur genau diesen Stand haben.
        if b.pr == 0 || seen != b.sha {
            let measured = self.ahead_behind(&b.upstream, BASE_REMOTE).await;
            let (inside, _) = self.squashed(&b.upstream).await;
            if !measured.is_some_and(|(ahead, _)| ahead <= 0 || inside) {
                return Err("origin hat Commits, die main fehlen; nichts gelöscht".to_owned());
            }
        }
        // Lease auf den zuletzt gesehenen Stand: hat seit dem letzten Fetch jemand nachgepusht, lehnt origin ab.
        let lease = format!("--force-with-lease=refs/heads/{up}:{seen}");
        self.git(&["push", &lease, "origin", "--delete", up]).await?;
        Ok(())
    }
}

#[derive(Default)]
struct Deleted {
    local: Vec<String>,
    remote: usize,
    failed: Vec<String>,
}

/// Nennt die gelöschten Branches mit Hash (zum Wiederherstellen: `git branch <name> <hash>`).
impl fmt::Display for Deleted {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} Branches gelöscht", self.local.len())?;
        if self.remote > 0 {
            write!(f, ", {} auf origin", self.remote)?;
        }
        if !self.local.is_empty() {
            write!(f, ": {}", self.local.join(", "))?;
        }
        if !self.failed.is_empty() {
            write!(f, "; nicht gelöscht: {}", self.failed.join("; "))?;
        }
        Ok(())
    }
}
