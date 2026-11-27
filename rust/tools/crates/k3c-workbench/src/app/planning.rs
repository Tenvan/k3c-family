//! Die Commands der Seite Planung (Port von `app_planning.go` ohne `GitHubStatus`, das steht in `github.rs`) und ihr
//! Wächter. Die Dateien sind die Quelle: Jeder Aufruf liest frisch aus docs/.

use std::collections::BTreeMap;
use std::time::Duration;

use super::App;
use super::events::{Events, PLANNING};
use crate::planning::{self, Changed, Data, Item, Watcher, WorktreeDiff};

/// Takt des Planungs-Wächters: Änderungen an docs/ erreichen die Oberfläche als `planning:changed`, auch wenn die
/// Seite nicht offen ist.
const POLL: Duration = Duration::from_secs(2);

/// Startet den Wächter der Planungsdateien; er läuft, bis der Rückgabewert fällt.
pub(super) fn watch(app: &App, events: &Events) -> Watcher {
    let events = events.clone();
    planning::watch_ids(&app.root, POLL, move |ids| {
        let changed = Changed {
            ids: ids.unwrap_or_default(),
        };
        events.emit_value(PLANNING, &changed);
    })
}

impl App {
    /// Sprints und Tickets mit Fließtext, dieselben Daten wie `plan_list`.
    pub fn planning_data(&self) -> Result<Data, String> {
        self.wait();
        planning::load(&self.root)
    }

    /// Wie [`App::planning_data`] ohne die Fließtexte der Session-Dateien (B-909).
    pub fn planning_list(&self) -> Result<Data, String> {
        self.wait();
        planning::load_list(&self.root)
    }

    /// Pfad und Fließtext zu den IDs (höchstens 100) in der Reihenfolge der Anfrage.
    pub fn planning_items(&self, ids: &[String]) -> Result<Vec<Item>, String> {
        self.wait();
        planning::items(&self.root, ids)
    }

    /// Die vorhandenen Dokumente in Umschalter-Reihenfolge.
    pub fn planning_docs(&self) -> Vec<String> {
        self.wait();
        planning::available(&self.root)
    }

    /// Eines der Dokumente `plan`, `fragen` oder `glossar` als Markdown.
    pub fn planning_doc(&self, name: &str) -> Result<String, String> {
        self.wait();
        planning::doc(&self.root, name)
    }

    /// Ändert ein Kopf-Feld wie `plan_set`; die Oberfläche lädt danach über `planning:changed` neu.
    pub fn planning_set(&self, id: &str, field: &str, value: &str) -> Result<String, String> {
        self.wait();
        let values = BTreeMap::from([(field.to_owned(), value.to_owned())]);
        planning::set(&self.root, id, &values)
    }

    /// Ticket, Sprint-README, Session oder Projekt als Markdown mit Pfad in der ersten Zeile, wie `plan_get`.
    pub fn planning_get(&self, id: &str) -> Result<String, String> {
        self.wait();
        planning::get(&self.root, id)
    }

    /// Vergleicht die Planung aller Worktrees mit der Wurzel; nur lesend.
    pub fn worktree_check(&self) -> Result<Vec<WorktreeDiff>, String> {
        self.wait();
        planning::check_worktrees(&self.root)
    }
}
