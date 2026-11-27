//! Abweichung der Regeltabelle (Port von `hubplan_rules.go`, B-462/AC-03): der Block „Hub-Stufe → Gebäude“ in § 3 gegen
//! den aus dem Planer-Stand erzeugten Block, mit derselben Funktion wie der Task (`table::gegenprobe`). Nur lesen.

use std::path::Path;

use serde::Serialize;

use super::{Editor, HUB_FILE, table};

#[cfg(test)]
#[path = "hubplan_rules_tests.rs"]
mod tests;

pub(super) const RULES_FILE: &str = "docs/rules/materialien-gebaeude.md";
pub(super) const RULES_TASK: &str = "task gen:rules";

/// Eine abweichende Zeile des Blocks (1-basiert); fehlt sie auf einer Seite, ist der Text dort leer.
#[derive(Clone, Debug, Serialize)]
pub struct HubRulesLine {
    pub line: usize,
    pub ist: String,
    pub soll: String,
}

/// Die Abweichung: Ist (Block im Regeltext), Soll (erzeugt), abweichende Zeilen und der Task, der den Block neu
/// schreibt; `error` bei nicht lesbarer Datei, fehlenden Marken oder unlesbarer hub.json.
#[derive(Clone, Debug, Default, Serialize)]
pub struct HubRulesView {
    pub path: String,
    pub task: String,
    pub same: bool,
    pub ist: String,
    pub soll: String,
    pub lines: Vec<HubRulesLine>,
    pub error: String,
}

fn empty_view() -> HubRulesView {
    HubRulesView {
        path: RULES_FILE.to_owned(),
        task: RULES_TASK.to_owned(),
        ..HubRulesView::default()
    }
}

fn failed(error: String) -> HubRulesView {
    HubRulesView { error, ..empty_view() }
}

impl Editor {
    /// Vergleicht die Regeltabelle mit dem Planer-Stand `hub_json`; leer = `data/hub.json` des Checkouts.
    pub fn rules(&self, hub_json: &str) -> HubRulesView {
        if !hub_json.is_empty() {
            return rules_from(&self.root, hub_json.as_bytes());
        }
        match std::fs::read(self.root.join(HUB_FILE)) {
            Ok(raw) => rules_from(&self.root, &raw),
            Err(e) => failed(format!("{HUB_FILE} nicht lesbar: {e}")),
        }
    }
}

/// Liest den Regeltext unter `root` und vergleicht seinen Block mit dem aus `hub_json` erzeugten.
fn rules_from(root: &Path, hub_json: &[u8]) -> HubRulesView {
    let doc = match std::fs::read_to_string(root.join(RULES_FILE)) {
        Ok(doc) => doc,
        Err(e) => return failed(format!("{RULES_FILE} nicht lesbar: {e}")),
    };
    match table::gegenprobe(&doc, hub_json) {
        Ok(check) => HubRulesView {
            same: check.same,
            lines: diff_lines(&check.ist, &check.soll),
            ist: check.ist,
            soll: check.soll,
            ..empty_view()
        },
        Err(e) => failed(format!("Regeltabelle nicht vergleichbar: {e}")),
    }
}

/// Stellt die Zeilen gleicher Nummer gegenüber und liefert die verschiedenen.
fn diff_lines(ist: &str, soll: &str) -> Vec<HubRulesLine> {
    let a: Vec<&str> = ist.split('\n').collect();
    let b: Vec<&str> = soll.split('\n').collect();
    (0..a.len().max(b.len()))
        .filter_map(|i| {
            let x = a.get(i).copied().unwrap_or_default();
            let y = b.get(i).copied().unwrap_or_default();
            (x != y).then(|| HubRulesLine {
                line: i + 1,
                ist: x.to_owned(),
                soll: y.to_owned(),
            })
        })
        .collect()
}
