//! Art und Eltern-Welt setzen (Port von `kind.go`, B-1217).

use serde_json::{Value, json};

use super::{CreateView, Editor, STANDARD_ID};

/// Leerer Text entfernt das Feld.
fn or_null(text: &str) -> Value {
    if text.is_empty() { Value::Null } else { json!(text) }
}

impl Editor {
    /// Setzt die Art der Welt (`spinoff` oder `new`; leer entfernt das Feld, dann gilt spinoff mit `extends`, sonst
    /// new) wie `world_set` und liefert die Antwort für die Seite.
    pub fn set_kind_view(&self, id: &str, kind: &str) -> CreateView {
        let mut patch = json!({"kind": or_null(kind)});
        // Ein Ableger braucht eine Eltern-Welt: ohne `extends` wird die Standard-Welt eingetragen (B-1217).
        if kind == "spinoff" && self.load(id).extends.is_empty() {
            patch["extends"] = json!(STANDARD_ID);
        }
        self.set_view(id, &patch)
    }

    /// Setzt die Eltern-Welt (leer entfernt sie) wie `world_set`; Unbekanntes, Zyklen und zu lange Ketten lehnt die
    /// Engine ab.
    pub fn set_extends_view(&self, id: &str, parent: &str) -> CreateView {
        self.set_view(id, &json!({"extends": or_null(parent)}))
    }
}

#[cfg(test)]
#[path = "kind_tests.rs"]
mod tests;
