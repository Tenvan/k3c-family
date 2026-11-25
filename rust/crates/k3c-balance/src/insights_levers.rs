//! Stellschrauben nach Wirkung (Port von `engine/balance/insights_levers.go`): Derselbe Wert in data/ taucht bei mehreren
//! verletzten Zielen als Hebel auf. Die Zusammenfassung nennt ihn einmal und listet, welche Verletzungen er (laut Richtwerten)
//! verbessern soll.

use std::collections::{HashMap, HashSet};

use serde::{Deserialize, Serialize};

use crate::insights::Insights;
use crate::insights_next::{Command, WHERE_CHAT, worst};
use crate::insights_plain::LeverPlan;

/// Ein Wert in data/ mit allen verletzten Zielen, die er verbessern soll. `conflict`: Ein anderes verletztes Ziel braucht
/// denselben Wert in die Gegenrichtung; dann hilft der Hebel dem einen und schadet vermutlich dem anderen.
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
#[serde(default)]
pub struct LeverGroup {
    pub path: String,
    pub direction: String,
    pub plan: LeverPlan,
    pub targets: Vec<String>,
    pub conflict: String,
    pub text: String,
}

/// Fasst die Hebel der verletzten Ziele nach Pfad und Richtung zusammen; die meisten Verletzungen zuerst (Go: `leverGroups`).
pub fn lever_groups(inn: &Insights) -> Vec<LeverGroup> {
    let mut out: Vec<LeverGroup> = Vec::new();
    let mut index: HashMap<String, usize> = HashMap::new();
    let mut dirs: HashMap<&str, usize> = HashMap::new(); // Pfad → Zahl der Richtungen, die vorkommen
    for t in worst(inn) {
        for (i, l) in t.levers.iter().enumerate() {
            let Some(plan) = t.plans.get(i) else { break };
            let j = *index.entry(format!("{}|{}", l.path, l.direction)).or_insert_with(|| {
                out.push(LeverGroup {
                    path: l.path.clone(),
                    direction: l.direction.clone(),
                    plan: plan.clone(),
                    ..LeverGroup::default()
                });
                *dirs.entry(&l.path).or_insert(0) += 1;
                out.len() - 1
            });
            out[j].targets.push(t.label());
        }
    }
    for g in &mut out {
        if dirs.get(g.path.as_str()).copied().unwrap_or(0) > 1 {
            g.conflict = "Achtung: ein anderes verletztes Ziel braucht diesen Wert in die Gegenrichtung.".into();
        }
        g.text = g.text();
    }
    out.sort_by_key(|g| std::cmp::Reverse(g.targets.len())); // stabil wie Gos SortStableFunc
    out
}

impl LeverGroup {
    fn text(&self) -> String {
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
            s += " ";
            s += &self.conflict;
        }
        s
    }
}

/// Ein Kopierblock mit dem Startvorschlag jeder Stellschraube als Wert-Variante (`datei.json:a.b.c=wert`), je Zeile mit
/// Kommentar (Ziele, Bereich). Ein Wert in Gegenrichtung zu einem Vorschlag mit mehr Verletzungen entfällt, ebenso ein
/// Hebel ohne seriösen Startwert (der steht im Kommentar-Block darunter; Go: `recommendations`).
pub fn recommendations(groups: &[LeverGroup]) -> Option<Command> {
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
            lines.push(format!(
                "{}={}  # {}; aktuell {}, Bereich {} … {}; verbessert: {}",
                g.path,
                g.plan.start,
                g.plan.label,
                g.plan.current,
                g.plan.min,
                g.plan.max,
                g.targets.join("; ")
            ));
        }
    }
    if lines.is_empty() {
        return None;
    }
    lines.extend(skipped);
    let text = format!(
        "# Empfehlungen aller Stellschrauben (Startvorschläge) als Wert-Varianten, ein Lauf mit allen zugleich:\n{}",
        lines.join("\n")
    );
    Some(Command {
        label: "Alle Empfehlungen als ein Block".into(),
        r#where: WHERE_CHAT.into(),
        text,
    })
}
