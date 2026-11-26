//! Auswertung im Markdown-Bericht (B-632): steht oben in `balance-<Zeit>.md`, vor der Tabelle je Ziel.

use std::fmt::Write as _;
use std::path::Path;

use super::figures::round1;
use super::hints::default_hints;
use super::insights::{
    Insights, LIGHT_GREEN, LIGHT_GREY, LIGHT_RED, LIGHT_YELLOW, Meta, Trend, build_insights, grade_name,
};
use super::insights_next::{WHERE_CHAT, WHERE_SHELL, WHERE_WORKBENCH, signed};
use super::summary::{Summary, num};

/// Das Zeichen zur Ampel, leer bei unbekanntem Wert.
pub(crate) fn light_icon(light: &str) -> &'static str {
    match light {
        LIGHT_GREEN => "🟢",
        LIGHT_YELLOW => "🟡",
        LIGHT_RED => "🔴",
        LIGHT_GREY => "⚪",
        _ => "",
    }
}

/// Wohin ein Befehl gehört (`Command.place`).
fn where_text(place: &str) -> &'static str {
    match place {
        WHERE_CHAT => "in den Chat an Claude",
        WHERE_SHELL => "im Terminal des Checkouts",
        WHERE_WORKBENCH => "Workbench › Balancing-Seite › Abstimmen",
        _ => "",
    }
}

/// Erklärt die Tabelle der Ziele in Klartext (steht im Markdown vor der Tabelle).
const TABLE_LEGEND: &str = "So liest du die Tabelle: **Wert** ist das Messergebnis (bei Anteilen: in wie viel Prozent der \
Läufe das Ziel erreicht wurde, „Läufe“ sind die Seeds). **Korridor** ist der Soll-Bereich dafür. **Abstand** ist der \
Unterschied zur verletzten Grenze in Prozentpunkten (Pkt.) bzw. in der Einheit des Werts; im Korridor steht dort die \
Reserve bis zur nächsten Grenze. **Trend** vergleicht mit der Baseline (▲ näher am Korridor, ▼ weiter weg).\n\n";

impl Summary {
    /// Auswertung mit den Richtwerten (`data/balance-hints.json` des Checkouts `root`).
    pub fn insights(&self, root: &Path, base: Option<&Summary>, meta: Meta) -> Result<Insights, String> {
        let hints = default_hints(root)?;
        Ok(build_insights(root, self, base, &hints, &meta))
    }
}

impl Insights {
    /// Kurzauswertung, Ampeln, Stärken und Schwächen, Hinweise, nächster Lauf, Kennzahlen.
    pub fn markdown(&self) -> String {
        let mut b = String::new();
        self.ladder_markdown(&mut b);
        b.push_str("## Kurzauswertung\n\n");
        bullets(&mut b, &self.summary, "");
        self.targets_markdown(&mut b);
        b.push_str("\n## Stärken\n\n");
        bullets(&mut b, &self.strengths, "Keine erkannt.");
        b.push_str("\n## Schwächen\n\n");
        bullets(&mut b, &self.weaknesses, "Keine erkannt.");
        self.hints_markdown(&mut b);
        self.delta_markdown(&mut b);
        self.next_markdown(&mut b);
        self.figures_markdown(&mut b);
        b
    }

    fn targets_markdown(&self, b: &mut String) {
        let count = |light: &str| self.counts.get(light).copied().unwrap_or_default();
        let _ = write!(
            b,
            "\n## Ampeln\n\n🟢 {} gut · 🟡 {} knapp · 🔴 {} verletzt · ⚪ {} ungültig\n\n",
            count(LIGHT_GREEN),
            count(LIGHT_YELLOW),
            count(LIGHT_RED),
            count(LIGHT_GREY)
        );
        b.push_str(TABLE_LEGEND);
        b.push_str(
            "| Ampel | Ziel | Grad | Wert | Korridor | Abstand zum Korridor | Trend |\n|---|---|---|---|---|---|---|\n",
        );
        for t in &self.targets {
            let _ = writeln!(
                b,
                "| {} {} | {} | {} | {} | {} | {} | {} |",
                light_icon(&t.light),
                t.status,
                t.name,
                t.grade,
                t.value_text(),
                t.bounds_text(),
                t.gap,
                trend_cell(t.trend.as_ref())
            );
        }
    }

    fn hints_markdown(&self, b: &mut String) {
        b.push_str("\n## Tuning-Hinweise\n\n");
        if self.hints.is_empty() {
            b.push_str("Keine: alle Ziele im Korridor.\n");
        }
        if !self.lever_groups.is_empty() {
            b.push_str(
                "\n### Stellschrauben nach Wirkung\n\nDerselbe Wert in data/ taucht bei mehreren Verletzungen auf; \
                 hier je Wert einmal, mit den Verletzungen, die er verbessern soll.\n\n",
            );
            for g in &self.lever_groups {
                let _ = writeln!(b, "- {}", g.text);
            }
            b.push_str("\n### Hinweise je Ziel\n\n");
        }
        for h in &self.hints {
            let _ = writeln!(b, "- {} **{}** – {} {}", light_icon(&h.light), h.target, h.text, h.why);
            for (i, l) in h.levers.iter().enumerate() {
                let _ = writeln!(b, "  - {} `{}`", l.label, l.path);
                if let Some(plan) = h.plans.get(i) {
                    let _ = writeln!(b, "    {}", plan.text);
                }
            }
        }
    }

    /// Vergleich je Werte-Gruppe, nur mit Baseline.
    fn delta_markdown(&self, b: &mut String) {
        if self.delta.is_empty() {
            return;
        }
        let _ = write!(b, "\n## Delta je Werte-Gruppe ({})\n\n", self.baseline.note);
        for d in &self.delta {
            let state = if d.frozen { "eingefroren" } else { "frei" };
            let _ = write!(
                b,
                "- {} **{}** ({state}): {}, {} besser, {} schlechter von {} Zielen",
                light_icon(&d.light),
                d.group,
                d.trend,
                d.better,
                d.worse,
                d.targets
            );
            if !d.warning.is_empty() {
                b.push_str(" – ⚠️ ");
                b.push_str(&d.warning);
            }
            b.push('\n');
        }
    }

    fn next_markdown(&self, b: &mut String) {
        b.push_str("\n## Nächster Lauf\n\n");
        for (i, s) in self.next.steps.iter().enumerate() {
            let _ = writeln!(b, "{}. {s}", i + 1);
        }
        for c in &self.next.commands {
            let _ = write!(b, "\n{} ({}):\n\n```\n{}\n```\n", c.label, where_text(&c.place), c.text);
        }
    }

    fn figures_markdown(&self, b: &mut String) {
        let f = &self.figures;
        b.push_str("\n## Kennzahlen\n\n");
        if !f.note.is_empty() {
            let _ = write!(b, "{}\n\n", f.note);
        }
        for l in &f.lights {
            let _ = writeln!(
                b,
                "- {} {}: {} ({})",
                light_icon(&l.light),
                l.label(),
                l.value_text(),
                l.text
            );
        }
        if let Some(island) = &f.island {
            island.markdown(b);
        }
        if let Some(ship) = &f.ship {
            ship.markdown(b);
        }
        self.nights_markdown(b);
    }

    fn nights_markdown(&self, b: &mut String) {
        if self.figures.nights.is_empty() {
            return;
        }
        b.push_str(
            "\n| Grad | Nacht | Läufe | Überleben % | Gold Tag | Gold Nacht | Holz | Stein | Leerlauf % |\n\
             |---|---|---|---|---|---|---|---|---|\n",
        );
        let cell = |v: f64| num(round1(v));
        for n in &self.figures.nights {
            let _ = writeln!(
                b,
                "| {} | {} | {} | {} | {} | {} | {} | {} | {} |",
                grade_name(&n.grade),
                n.night,
                n.runs,
                cell(n.survival),
                cell(n.day_income - n.day_expense),
                cell(n.night_income - n.night_expense),
                cell(n.resource_in.wood - n.resource_out.wood),
                cell(n.resource_in.stone - n.resource_out.stone),
                cell(n.idle_share)
            );
        }
    }
}

fn bullets(b: &mut String, items: &[String], empty: &str) {
    if items.is_empty() && !empty.is_empty() {
        let _ = writeln!(b, "{empty}");
    }
    for s in items {
        let _ = writeln!(b, "- {s}");
    }
}

fn trend_cell(trend: Option<&Trend>) -> String {
    let Some(t) = trend else {
        return "–".to_owned();
    };
    let mark = if t.better { "▲" } else { "▼" };
    format!("{mark} {}", signed(t.delta))
}
