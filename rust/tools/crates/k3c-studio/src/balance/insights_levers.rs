//! Stellschrauben nach Wirkung: Derselbe Wert in `data/` taucht bei mehreren verletzten Zielen als Hebel auf. Die
//! Zusammenfassung nennt ihn einmal und listet, welche Verletzungen er (laut Richtwerten) verbessern soll.

use std::cmp::Reverse;
use std::collections::{HashMap, HashSet};

use serde::{Deserialize, Serialize};

use super::insights::Insights;
use super::insights_next::{Command, WHERE_CHAT};
use super::insights_plain::LeverPlan;
use super::insights_text::worst;

/// Ein Wert in `data/` mit allen verletzten Zielen, die er verbessern soll. `conflict`: ein anderes verletztes Ziel
/// braucht denselben Wert in die Gegenrichtung.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(default)]
pub struct LeverGroup {
    pub path: String,
    pub direction: String,
    pub plan: LeverPlan,
    pub targets: Vec<String>,
    pub conflict: String,
    pub text: String,
}

/// Fasst die Hebel der verletzten Ziele nach Pfad und Richtung zusammen; die meisten Verletzungen zuerst.
pub(crate) fn lever_groups(insights: &Insights) -> Vec<LeverGroup> {
    let mut out: Vec<LeverGroup> = Vec::new();
    let mut index: HashMap<String, usize> = HashMap::new();
    // Pfad → Zahl der Richtungen, die vorkommen
    let mut directions: HashMap<String, usize> = HashMap::new();
    for t in worst(insights) {
        for (l, plan) in t.levers.iter().zip(&t.plans) {
            let key = format!("{}|{}", l.path, l.direction);
            let j = *index.entry(key).or_insert_with(|| {
                out.push(LeverGroup {
                    path: l.path.clone(),
                    direction: l.direction.clone(),
                    plan: plan.clone(),
                    ..LeverGroup::default()
                });
                *directions.entry(l.path.clone()).or_default() += 1;
                out.len() - 1
            });
            out[j].targets.push(t.label());
        }
    }
    for g in &mut out {
        if directions.get(&g.path).is_some_and(|n| *n > 1) {
            "Achtung: ein anderes verletztes Ziel braucht diesen Wert in die Gegenrichtung."
                .clone_into(&mut g.conflict);
        }
        g.text = g.sentence();
    }
    out.sort_by_key(|g| Reverse(g.targets.len()));
    out
}

impl LeverGroup {
    fn sentence(&self) -> String {
        let way = if self.direction == "-" { "senken" } else { "erhöhen" };
        let mut s = format!(
            "{} `{}` – soll {} Verletzung(en) verbessern: {}. {}",
            self.plan.label,
            self.path,
            self.targets.len(),
            self.targets.join("; "),
            self.plan.short
        );
        if !self.plan.label.contains(way) {
            s = format!("({way}) {s}");
        }
        if !self.conflict.is_empty() {
            s.push(' ');
            s.push_str(&self.conflict);
        }
        s
    }
}

/// Kopierblock mit dem Startvorschlag jeder Stellschraube als Wert-Variante (`datei.json:a.b.c=wert`), je Zeile mit
/// Kommentar (Ziele, Bereich). Ein Wert in Gegenrichtung zu einem Vorschlag mit mehr Verletzungen entfällt, ebenso ein
/// Hebel ohne seriösen Startwert (der steht im Kommentar-Block darunter).
pub(crate) fn recommendations(groups: &[LeverGroup]) -> Option<Command> {
    let (mut lines, mut skipped) = (Vec::new(), Vec::new());
    let mut taken: HashSet<&str> = HashSet::new();
    for g in groups {
        if taken.contains(g.path.as_str()) {
            skipped.push(format!(
                "# {}: Gegenrichtung zu einem Vorschlag mit mehr Verletzungen, nicht mitgenommen.",
                g.path
            ));
        } else if !g.plan.known {
            skipped.push(format!(
                "# {}: Startwert nicht seriös abschätzbar, Wert von Hand wählen.",
                g.path
            ));
        } else {
            taken.insert(&g.path);
            let p = &g.plan;
            lines.push(format!(
                "{}={}  # {}; aktuell {}, Bereich {} … {}; verbessert: {}",
                g.path,
                p.start,
                p.label,
                p.current,
                p.min,
                p.max,
                g.targets.join("; ")
            ));
        }
    }
    if lines.is_empty() {
        return None;
    }
    lines.append(&mut skipped);
    Some(Command {
        label: "Alle Empfehlungen als ein Block".to_owned(),
        place: WHERE_CHAT.to_owned(),
        text: format!(
            "# Empfehlungen aller Stellschrauben (Startvorschläge) als Wert-Varianten, ein Lauf mit allen zugleich:\n{}",
            lines.join("\n")
        ),
    })
}
