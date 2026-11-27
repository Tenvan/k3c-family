//! Bearbeiten im Gebäude-Planer (Port von `hubplan_edit.go`, B-462/AC-02): Das Frontend ändert den Kandidaten (Text der
//! hub.json) und lässt ihn hier prüfen; Matrix und Befunde kommen aus derselben Engine-Funktion wie beim Laden.

use super::{Editor, HUB_FILE, HubPlanView};

#[cfg(test)]
#[path = "hubplan_edit_tests.rs"]
mod tests;

impl Editor {
    /// Prüft einen Kandidaten des Planers und liefert Matrix und Befunde, ohne etwas zu schreiben.
    pub fn check(&self, candidate: &str) -> HubPlanView {
        self.plan_of(HUB_FILE, candidate)
    }
}
