//! Welche Branches stehen in Worktrees, und zu welchen Sprints gehören sie?

use std::path::Path;
use std::time::Duration;

use regex::Regex;

use super::gitrepo::{git_out, in_repo};
use super::planning::Sprint;

const TIMEOUT: Duration = Duration::from_secs(3);

/// Branches aller Worktrees des Repos; ohne git oder Repo keine.
pub fn worktrees(root: &Path) -> Vec<String> {
    if !in_repo(root) {
        return Vec::new();
    }
    git_out(root, &["worktree", "list", "--porcelain"], TIMEOUT)
        .map(|out| parse_worktrees(&out))
        .unwrap_or_default()
}

pub fn parse_worktrees(porcelain: &str) -> Vec<String> {
    porcelain
        .replace("\r\n", "\n")
        .split('\n')
        .filter_map(|l| l.strip_prefix("branch refs/heads/"))
        .map(str::to_owned)
        .collect()
}

/// Ordnet Branches den Sprints zu: `sprint/s4`, `s4-4-work`, `f4/4-review`, `claude/sprint-s4-ab12`,
/// `claude/s4-review-ab12` gehören zu S4 bzw. F4. Alle Treffer landen in `worktrees` (mehrere Agenten je Sprint), der
/// erste in `worktree`. Nur über den Branch-Namen; ein Worktree auf einem fremd benannten Branch bleibt unerkannt.
pub fn mark_worktrees(sprints: &mut [Sprint], branches: &[String]) {
    if branches.is_empty() {
        return;
    }
    for sprint in sprints.iter_mut().filter(|s| !s.id.is_empty()) {
        let pattern = format!(
            r"(?i)^((sprint|claude)/)?(sprint-)?{}([-/.]|$)",
            regex::escape(&sprint.id)
        );
        let Ok(re) = Regex::new(&pattern) else {
            continue;
        };
        sprint
            .worktrees
            .extend(branches.iter().filter(|b| re.is_match(b)).cloned());
        if let Some(first) = sprint.worktrees.first() {
            sprint.worktree.clone_from(first);
        }
    }
}
