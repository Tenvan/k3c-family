//! Seite Git (B-580, Port von `app_git.go`): Stand des Repos, Branches, Dateien und PRs; Aktionen ohne Force und
//! ohne Shell. Jede Aktion steht im Log der Workbench. Die Methoden blockieren wie die der MCP-Seite.

use std::sync::Arc;

use serde::Serialize;
use serde_json::json;
use tokio::runtime::Handle;

use super::{App, lock};
use crate::github::{self, PrList};
use crate::gitops::{self, Branch, Files, MergedPr, Repo, Status};

/// Alles, was die Seite beim Laden zeichnet; PRs kommen getrennt (`gh` braucht Netz und ist langsamer).
#[derive(Clone, Debug, Serialize)]
pub struct GitView {
    pub status: Status,
    pub branches: Vec<Branch>,
    pub files: Files,
}

/// Das Repo der Seite und der GitHub-Client.
pub(super) struct GitSeat {
    pub(super) repo: Repo,
    pub(super) github: Arc<github::Client>,
}

impl App {
    /// Anderes Repo und anderer GitHub-Client (Tests).
    #[cfg(test)]
    pub(crate) fn with_git(mut self, repo: Repo, github: github::Client) -> App {
        self.git = GitSeat {
            repo,
            github: Arc::new(github),
        };
        self
    }

    pub(super) fn git_runtime(&self) -> Result<Handle, String> {
        self.wait();
        lock(&self.running)
            .as_ref()
            .map(|r| r.runtime.clone())
            .ok_or_else(|| "backend beendet".to_owned())
    }

    /// Status, Branches und der Diff zu origin/main.
    pub fn git_overview(&self) -> Result<GitView, String> {
        let repo = &self.git.repo;
        self.git_runtime()?.block_on(async {
            let status = repo.status().await?;
            let branches = repo.branches().await?;
            // Ohne origin/main geht der Rest.
            let files = repo.branch_files().await.unwrap_or_else(|_| Files::empty());
            Ok(GitView {
                status,
                branches,
                files,
            })
        })
    }

    /// Führt fetch, pull, push, stash oder stash-pop aus.
    pub fn git_action(&self, action: &str) -> Result<String, String> {
        let result = self.git_runtime()?.block_on(self.git.repo.act(action));
        self.git_log(&format!("git {action}"), &result);
        result
    }

    /// Löscht gemergte lokale Branches, mit `remote` auch auf origin.
    pub fn git_delete_branches(&self, names: &[String], remote: bool) -> Result<String, String> {
        let result = self.git_runtime()?.block_on(self.git.repo.delete(names, remote));
        match &result {
            Ok(message) => self.logger().info(
                "🧹 Branches gelöscht",
                &[
                    ("ns", json!("main")),
                    ("ergebnis", json!(message)),
                    ("remote", json!(remote)),
                ],
            ),
            Err(_) => self.git_log("Branches löschen", &result),
        }
        result
    }

    /// Prüft Branches gründlich auf Squash-Merges (gemergter PR, merge-tree, Squash-Patch) und liefert die Branch-Liste
    /// neu; `names` leer = alle ungeprüften. Ohne `gh` prüft git allein.
    pub fn git_check_merged(&self, names: &[String]) -> Result<Vec<Branch>, String> {
        let runtime = self.git_runtime()?;
        let heads = runtime.block_on(self.git.github.merged()).unwrap_or_else(|e| {
            self.logger().warn(
                "❌ Gemergte PRs nicht gelesen, prüfe nur mit git",
                &[("ns", json!("main")), ("fehler", json!(e))],
            );
            Vec::new()
        });
        let prs: Vec<MergedPr> = heads
            .into_iter()
            .map(|h| MergedPr {
                number: h.number,
                head: h.head,
                sha: h.sha,
            })
            .collect();
        runtime.block_on(async {
            self.git.repo.check(names, &prs).await?;
            self.git.repo.branches().await
        })
    }

    /// Fingerabdruck des Repo-Zustands; die Seite fragt ihn im Takt und lädt bei einer Änderung die PRs neu.
    pub fn git_stamp(&self) -> Result<String, String> {
        self.git_runtime()?.block_on(self.git.repo.stamp())
    }

    /// Die offenen PRs gegen main; ein Fehler von `gh` steht in `error`.
    pub fn git_prs(&self) -> Result<PrList, String> {
        Ok(self.git_runtime()?.block_on(self.git.github.open()))
    }

    /// Legt für den aktuellen Branch einen PR gegen main an und liefert die Adresse.
    pub fn git_pr_create(&self, title: &str, body: &str) -> Result<String, String> {
        let runtime = self.git_runtime()?;
        let status = runtime.block_on(self.git.repo.status())?;
        if status.branch.is_empty() || !gitops::valid_branch(&status.branch) {
            return Err("kein gültiger Branch ausgecheckt".to_owned());
        }
        if status.upstream.is_empty() || status.upstream_gone {
            return Err("der Branch ist nicht auf origin: erst pushen".to_owned());
        }
        if status.ahead > 0 {
            return Err(format!("{} Commits nicht gepusht: erst pushen", status.ahead));
        }
        let url = runtime.block_on(self.git.github.create(&status.branch, title, body));
        self.git_log("PR anlegen", &url);
        url
    }

    /// Mergt (`merge`, `squash`) oder schließt (`close`) den PR `number`.
    pub fn git_pr_action(&self, number: i64, action: &str) -> Result<String, String> {
        let runtime = self.git_runtime()?;
        if number <= 0 {
            return Err("ungültige PR-Nummer".to_owned());
        }
        let gh = &self.git.github;
        let (done, message) = match action {
            "merge" | "squash" => (
                runtime.block_on(gh.merge(number, action == "squash")),
                format!("PR #{number} gemergt"),
            ),
            "close" => (runtime.block_on(gh.close(number)), format!("PR #{number} geschlossen")),
            _ => return Err(format!("unbekannte PR-Aktion {action:?}")),
        };
        let result = done.map(|()| message);
        self.git_log(&format!("PR #{number} {action}"), &result);
        result
    }

    fn git_log(&self, what: &str, result: &Result<String, String>) {
        let log = self.logger();
        match result {
            Ok(message) => log.info(
                &format!("✅ {what}"),
                &[("ns", json!("main")), ("ergebnis", json!(message))],
            ),
            Err(e) => log.warn(
                &format!("❌ {what} fehlgeschlagen"),
                &[("ns", json!("main")), ("fehler", json!(e))],
            ),
        }
    }
}
