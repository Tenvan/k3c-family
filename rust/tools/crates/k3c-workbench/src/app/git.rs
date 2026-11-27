//! Seite Git (B-580, Port von `app_git.go` ohne PRs): Stand des Repos, Branches und Dateien; Aktionen ohne Force und
//! ohne Shell. Jede Aktion steht im Log der Workbench. Die Methoden blockieren wie die der MCP-Seite.

use serde::Serialize;
use serde_json::json;
use tokio::runtime::Handle;

use super::{App, lock};
use crate::github::MergedSource;
use crate::gitops::{Branch, Files, Repo, Status};

/// Alles, was die Seite beim Laden zeichnet; PRs kommen getrennt (`gh` braucht Netz und ist langsamer).
#[derive(Clone, Debug, Serialize)]
pub struct GitView {
    pub status: Status,
    pub branches: Vec<Branch>,
    pub files: Files,
}

/// Das Repo der Seite und die Quelle der gemergten PRs.
pub(super) struct GitSeat {
    pub(super) repo: Repo,
    pub(super) merged: MergedSource,
}

impl App {
    /// Anderes Repo und andere PR-Quelle (Tests).
    #[cfg(test)]
    pub(crate) fn with_git(mut self, repo: Repo, merged: MergedSource) -> App {
        self.git = GitSeat { repo, merged };
        self
    }

    fn git_runtime(&self) -> Result<Handle, String> {
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
        let prs = runtime.block_on((self.git.merged)()).unwrap_or_else(|e| {
            self.logger().warn(
                "❌ Gemergte PRs nicht gelesen, prüfe nur mit git",
                &[("ns", json!("main")), ("fehler", json!(e))],
            );
            Vec::new()
        });
        runtime.block_on(async {
            self.git.repo.check(names, &prs).await?;
            self.git.repo.branches().await
        })
    }

    /// Fingerabdruck des Repo-Zustands; die Seite fragt ihn im Takt und lädt bei einer Änderung die PRs neu.
    pub fn git_stamp(&self) -> Result<String, String> {
        self.git_runtime()?.block_on(self.git.repo.stamp())
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
