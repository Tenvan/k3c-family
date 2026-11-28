//! B-519 (Go `next_origin.go`): Status und Dateien eines aktiven Sprints stehen bis zum Sprint-PR nur auf
//! origin/sprint/<präfix>. `plan_next` überlagert sie aus dem lokalen Ref refs/remotes/origin/… (kein fetch, kein
//! Netz); fehlt der Ref, gilt der lokale Stand.

use std::collections::HashMap;
use std::path::Path;

use super::gitrepo::in_repo;
use super::mergebase::{load_at, run_git};
use super::next::session_head;
use super::planning::{Data, Sprint};
use super::tables::first_of;

/// Überlagert die Sessions jedes aktiven Sprints in `data` mit dem Stand von origin/sprint/<präfix> und liefert je
/// Sprint eine Hinweiszeile, ob origin oder der lokale Stand gilt, dazu Sprint-ID → README von origin.
pub(super) fn origin_overlay(root: &Path, data: &mut Data) -> (Vec<String>, HashMap<String, String>) {
    let (mut notes, mut readmes) = (Vec::new(), HashMap::new());
    // Ohne Repo scheiterte jeder git-Aufruf; die Prüfung spart ihn (B-830).
    let repo = in_repo(root);
    for sprint in data.sprints.iter_mut().filter(|s| s.status == "aktiv") {
        let prefix = sprint_prefix(sprint);
        let reference = format!("refs/remotes/origin/sprint/{prefix}");
        if !repo || run_git(root, &["rev-parse", "--verify", "--quiet", &reference]).is_err() {
            notes.push(format!("{}: kein origin/sprint/{prefix}, lokaler Stand", sprint.id));
            continue;
        }
        let origin = match load_at(root, &reference) {
            Ok(origin) => origin,
            Err(e) => {
                notes.push(format!(
                    "{}: origin/sprint/{prefix} nicht lesbar ({e}), lokaler Stand",
                    sprint.id
                ));
                continue;
            }
        };
        let Some(found) = origin.sprints.into_iter().find(|o| o.id == sprint.id) else {
            continue;
        };
        sprint.sessions = found.sessions;
        let path = origin_readme_path(root, &reference, &sprint.id);
        if let Ok(text) = run_git(root, &["show", &format!("{reference}:docs/{path}")]) {
            readmes.insert(sprint.id.clone(), String::from_utf8_lossy(&text).into_owned());
        }
        notes.push(format!("{}: Stand von origin/sprint/{prefix}", sprint.id));
    }
    (notes, readmes)
}

/// Pfad der Sprint-README unter docs/ im Stand `reference` (der Sprint kann dort in einem anderen Zustandsordner
/// liegen); leer, wenn nicht gefunden.
fn origin_readme_path(root: &Path, reference: &str, id: &str) -> String {
    let Ok(out) = run_git(root, &["ls-tree", "-r", "--name-only", reference, "docs/sprints"]) else {
        return String::new();
    };
    let prefix = format!("{id}-");
    String::from_utf8_lossy(&out)
        .lines()
        .map(str::trim)
        .find(|line| {
            let parts: Vec<&str> = line.split('/').collect();
            matches!(parts.as_slice(), [_, _, _, dir, "README.md"] if dir.starts_with(&prefix))
        })
        .map(|line| line.strip_prefix("docs/").unwrap_or(line).to_owned())
        .unwrap_or_default()
}

/// Der Branch-Präfix ohne „sprint/“: aus „Branch“ der ersten Session, die einen trägt, sonst die Sprint-ID klein
/// (B-782).
pub(super) fn sprint_prefix(sprint: &Sprint) -> String {
    sprint
        .sessions
        .iter()
        .find_map(|x| {
            let prefix = branch_prefix(session_head(x).get("Branch")?);
            (!prefix.is_empty()).then_some(prefix)
        })
        .unwrap_or_else(|| sprint.id.to_lowercase())
}

/// Der Sprint-Präfix aus dem Session-Feld „Branch“ (B-782): `sprint/cu1` → `cu1`, `w15/1-fackeln` → `w15`; leer oder
/// ohne `/` → leer (dann gilt die Sprint-ID).
fn branch_prefix(field: &str) -> String {
    let value = field.trim().trim_matches('`');
    if let Some(rest) = value.strip_prefix("sprint/") {
        return rest.split('/').next().unwrap_or_default().to_owned();
    }
    value
        .split_once('/')
        .map_or_else(String::new, |(prefix, _)| prefix.to_owned())
}

/// Der Sprint-Branch einer Session: `sprint/` + Präfix aus „Branch“, sonst aus der Sprint-ID (B-782).
pub fn sprint_branch(sprint_id: &str, field: &str) -> String {
    let (prefix, id) = (branch_prefix(field), sprint_id.to_lowercase());
    format!("sprint/{}", first_of(&[&prefix, &id]))
}
