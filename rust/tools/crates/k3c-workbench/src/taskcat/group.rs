//! Gruppen und Suche (Port von `group.go`). Beides liegt hier und nicht im Frontend, weil `task_list` denselben Baum
//! zeigen muss wie die Oberfläche.

use std::collections::BTreeMap;

use serde::Serialize;

use super::{Task, WORKSPACE};

/// Eine Gruppe des Baums.
#[derive(Clone, Debug, Serialize)]
pub struct Namespace {
    pub name: String,
    pub tasks: Vec<Task>,
}

/// Ordnet die Tasks ihren Namensräumen zu: Workspace zuerst, die übrigen alphabetisch, Tasks innerhalb nach Name.
pub fn group(tasks: &[Task]) -> Vec<Namespace> {
    let mut by_name: BTreeMap<&str, Vec<Task>> = BTreeMap::new();
    for task in tasks {
        by_name.entry(&task.namespace).or_default().push(task.clone());
    }
    let mut groups: Vec<Namespace> = by_name
        .into_iter()
        .map(|(name, mut tasks)| {
            tasks.sort_by(|a, b| a.name.cmp(&b.name));
            Namespace {
                name: name.to_owned(),
                tasks,
            }
        })
        .collect();
    // Stabil: nur Workspace wandert nach vorn.
    groups.sort_by_key(|g| g.name != WORKSPACE);
    groups
}

/// Behält die Tasks, deren Name oder Beschreibung `query` enthält, ohne Groß- und Kleinschreibung; leer: alle.
pub fn filter(tasks: Vec<Task>, query: &str) -> Vec<Task> {
    let query = query.trim().to_lowercase();
    if query.is_empty() {
        return tasks;
    }
    let hit = |text: &str| text.to_lowercase().contains(&query);
    tasks.into_iter().filter(|t| hit(&t.name) || hit(&t.desc)).collect()
}
