//! `git status --porcelain=v1 -z --branch` lesen.

use serde::Serialize;

/// Eine Datei im Index (Status A, M, D, R, C, T) oder im Arbeitsbaum (M, D, T, U, "?" für untracked).
#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
pub struct File {
    pub path: String,
    pub status: String,
}

/// Stand von Branch, Index und Arbeitsbaum.
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize)]
pub struct State {
    pub branch: String,
    pub staged: Vec<File>,
    pub unstaged: Vec<File>,
    /// Letzte Commits, eine Zeile je Commit.
    pub recent: Vec<String>,
}

/// "## branch...upstream\0", dann "XY pfad\0", bei einer Umbenennung im Index "XY neu\0alt\0". X ist der Index, Y der
/// Arbeitsbaum.
pub fn parse_status(out: &[u8]) -> Result<State, String> {
    let mut state = State::default();
    let text = String::from_utf8_lossy(out);
    let mut fields = text.trim_end_matches('\0').split('\0');
    while let Some(entry) = fields.next() {
        if entry.is_empty() {
            continue;
        }
        if let Some(rest) = entry.strip_prefix("## ") {
            let branch = rest.split("...").next().unwrap_or_default();
            state.branch = branch.strip_prefix("No commits yet on ").unwrap_or(branch).to_owned();
            continue;
        }
        let mut chars = entry.chars();
        let (Some(x), Some(y), Some(' ')) = (chars.next(), chars.next(), chars.next()) else {
            return Err(format!("git status: unerwarteter Eintrag {entry:?}"));
        };
        let path = chars.as_str();
        if path.is_empty() {
            return Err(format!("git status: unerwarteter Eintrag {entry:?}"));
        }
        if x == 'R' || x == 'C' {
            fields.next(); // der alte Pfad steht als eigenes Feld dahinter
        }
        let (staged, unstaged) = classify(x, y);
        let file = |status: char| File {
            path: path.to_owned(),
            status: status.to_string(),
        };
        state.staged.extend(staged.map(file));
        state.unstaged.extend(unstaged.map(file));
    }
    Ok(state)
}

/// Spalten X (Index) und Y (Arbeitsbaum) zu je einem Status; `None` heißt: nicht in dieser Liste.
fn classify(x: char, y: char) -> (Option<char>, Option<char>) {
    match (x, y) {
        ('?', '?') => (None, Some('?')),
        // Konflikt: erst auflösen, dann stagen.
        ('U', _) | (_, 'U') | ('A', 'A') | ('D', 'D') => (None, Some('U')),
        _ => {
            let listed = |c: char| (c != ' ' && c != '!').then_some(c);
            (listed(x), listed(y))
        }
    }
}
