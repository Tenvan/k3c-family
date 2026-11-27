//! GitHub-Stand der Sprints für die Seite Planung (Port von `GitHubStatus` aus `app_planning.go`).

use super::App;
use crate::github::Data;

impl App {
    /// PR, CI und Merge-Stand je Sprint aus `gh` (B-212); `force` umgeht den Zwischenspeicher.
    pub fn github_status(&self, force: bool) -> Result<Data, String> {
        Ok(self.git_runtime()?.block_on(self.git.github.status(force)))
    }
}
