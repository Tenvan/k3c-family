//! Auswertung im Markdown-Bericht (Port von `engine/balance/insights_markdown.go`, B-632): steht oben in balance-<Zeit>.md,
//! vor der Tabelle je Ziel. Dazu `ladderMarkdown` und seine Hilfen aus `insights_progress.go` und die Texte der
//! Kennzahl-Ampeln aus `insights_text.go`, die der Bericht braucht.

use std::fmt::Write;

use crate::figures::round1;
use crate::insights::{Insights, LIGHT_GREEN, LIGHT_GREY, LIGHT_RED, LIGHT_YELLOW, Trend, grade_name};
use crate::insights_figures::FigureLight;
use crate::insights_next::{WHERE_CHAT, WHERE_SHELL, WHERE_WORKBENCH, signed};
use crate::summary_text::num;

/// Das Zeichen der Ampel (Go: `lightIcon`).
pub fn light_icon(light: &str) -> &'static str {
    match light {
        LIGHT_GREEN => "🟢",
        LIGHT_YELLOW => "🟡",
        LIGHT_RED => "🔴",
        LIGHT_GREY => "⚪",
        _ => "",
    }
}

/// Wohin ein Befehl gehört (Go: `whereText`).
fn where_text(w: &str) -> &'static str {
    match w {
        WHERE_CHAT => "in den Chat an Claude",
        WHERE_SHELL => "im Terminal des Checkouts",
        WHERE_WORKBENCH => "Workbench › Balancing-Seite › Abstimmen",
        _ => "",
    }
}

/// Erklärt die Tabelle der Ziele in Klartext (steht im Markdown vor der Tabelle).
const TABLE_LEGEND: &str = "So liest du die Tabelle: **Wert** ist das Messergebnis (bei Anteilen: in wie viel Prozent der Läufe das Ziel erreicht wurde, \
„Läufe“ sind die Seeds). **Korridor** ist der Soll-Bereich dafür. **Abstand** ist der Unterschied zur verletzten Grenze in Prozentpunkten (Pkt.) \
bzw. in der Einheit des Werts; im Korridor steht dort die Reserve bis zur nächsten Grenze. **Trend** vergleicht mit der Baseline \
(▲ näher am Korridor, ▼ weiter weg).\n\n";

impl FigureLight {
    /// Name der Kennzahl mit Grad, wenn nicht normal (Go: `FigureLight.label`).
    pub fn label(&self) -> String {
        if self.grade.is_empty() || self.grade == "normal" {
            self.name.clone()
        } else {
            format!("{} ({})", self.name, self.grade)
        }
    }

    /// Messwert mit Einheit, „–“ ohne Messwert (Go: `FigureLight.valueText`).
    pub fn value_text(&self) -> String {
        match self.value {
            None => "–".into(),
            Some(v) => format!("{} {}", num(v), self.unit).trim().to_string(),
        }
    }
}

/// Schreibt die Auswertung: Kurzauswertung, Ampeln, Stärken und Schwächen, Hinweise, nächster Lauf, Kennzahlen
/// (Go: `Insights.Markdown`).
pub fn insights_markdown(inn: &Insights) -> String {
    let mut b = String::new();
    ladder_markdown(inn, &mut b);
    b.push_str("## Kurzauswertung\n\n");
    bullets(&mut b, &inn.summary, "");
    let count = |l: &str| inn.counts.get(l).copied().unwrap_or(0);
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
    for t in &inn.targets {
        let _ = writeln!(
            b,
            "| {} {} | {} | {} | {} | {} | {} | {} |",
            light_icon(&t.light),
            t.status,
            t.name,
            t.grade,
            t.entry.value_text(),
            t.entry.bounds_text(),
            t.gap,
            trend_text(t.trend.as_ref())
        );
    }
    b.push_str("\n## Stärken\n\n");
    bullets(&mut b, &inn.strengths, "Keine erkannt.");
    b.push_str("\n## Schwächen\n\n");
    bullets(&mut b, &inn.weaknesses, "Keine erkannt.");
    b.push_str("\n## Tuning-Hinweise\n\n");
    hints_markdown(inn, &mut b);
    delta_markdown(inn, &mut b);
    next_markdown(inn, &mut b);
    figures_markdown(inn, &mut b);
    b
}

fn hints_markdown(inn: &Insights, b: &mut String) {
    if inn.hints.is_empty() {
        b.push_str("Keine: alle Ziele im Korridor.\n");
    }
    if !inn.lever_groups.is_empty() {
        b.push_str("\n### Stellschrauben nach Wirkung\n\nDerselbe Wert in data/ taucht bei mehreren Verletzungen auf; hier je Wert einmal, mit den Verletzungen, die er verbessern soll.\n\n");
        for g in &inn.lever_groups {
            let _ = writeln!(b, "- {}", g.text);
        }
        b.push_str("\n### Hinweise je Ziel\n\n");
    }
    for h in &inn.hints {
        let _ = writeln!(b, "- {} **{}** – {} {}", light_icon(&h.light), h.target, h.text, h.why);
        for (i, l) in h.levers.iter().enumerate() {
            let _ = writeln!(b, "  - {} `{}`", l.label, l.path);
            if let Some(p) = h.plans.get(i) {
                let _ = writeln!(b, "    {}", p.text);
            }
        }
    }
}

/// Vergleich je Werte-Gruppe (`delta.rs`), nur mit Baseline.
fn delta_markdown(inn: &Insights, b: &mut String) {
    if inn.delta.is_empty() {
        return;
    }
    let _ = write!(b, "\n## Delta je Werte-Gruppe ({})\n\n", inn.baseline.note);
    for d in &inn.delta {
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
            let _ = write!(b, " – ⚠️ {}", d.warning);
        }
        b.push('\n');
    }
}

fn next_markdown(inn: &Insights, b: &mut String) {
    b.push_str("\n## Nächster Lauf\n\n");
    for (i, s) in inn.next.steps.iter().enumerate() {
        let _ = writeln!(b, "{}. {s}", i + 1);
    }
    for c in &inn.next.commands {
        let _ = write!(
            b,
            "\n{} ({}):\n\n```\n{}\n```\n",
            c.label,
            where_text(&c.r#where),
            c.text
        );
    }
}

fn figures_markdown(inn: &Insights, b: &mut String) {
    let f = &inn.figures;
    b.push_str("\n## Kennzahlen\n\n");
    if !f.note.is_empty() {
        b.push_str(&f.note);
        b.push_str("\n\n");
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
    if let Some(i) = &f.island {
        i.markdown(b);
    }
    if let Some(s) = &f.ship {
        s.markdown(b);
    }
    if f.nights.is_empty() {
        return;
    }
    b.push_str("\n| Grad | Nacht | Läufe | Überleben % | Gold Tag | Gold Nacht | Holz | Stein | Leerlauf % |\n|---|---|---|---|---|---|---|---|---|\n");
    for n in &f.nights {
        let _ = writeln!(
            b,
            "| {} | {} | {} | {} | {} | {} | {} | {} | {} |",
            grade_name(&n.grade),
            n.night,
            n.runs,
            num(round1(n.survival)),
            num(round1(n.day_income - n.day_expense)),
            num(round1(n.night_income - n.night_expense)),
            num(round1(n.resource_in.wood - n.resource_out.wood)),
            num(round1(n.resource_in.stone - n.resource_out.stone)),
            num(round1(n.idle_share))
        );
    }
}

fn bullets(b: &mut String, items: &[String], empty: &str) {
    if items.is_empty() && !empty.is_empty() {
        b.push_str(empty);
        b.push('\n');
    }
    for s in items {
        b.push_str("- ");
        b.push_str(s);
        b.push('\n');
    }
}

fn trend_text(t: Option<&Trend>) -> String {
    let Some(t) = t else { return "–".into() };
    let mark = if t.better { "▲" } else { "▼" };
    format!("{mark} {}", signed(t.delta))
}

/// Schreibt Urteil, Konfiguration, Leiter und Blocker (Reihenfolge laut B-648: Ergebnis, Leiter, Blocker; Go:
/// `ladderMarkdown`).
fn ladder_markdown(inn: &Insights, b: &mut String) {
    let Some(goal) = &inn.goal else { return };
    b.push_str("## Ergebnis\n\n");
    let mark = if goal.reached { "✅" } else { "❌" };
    let _ = write!(b, "{mark} {}\n\n", inn.goal_text);
    if let Some(q) = &inn.quality {
        let _ = write!(b, "Qualität: {} von {} Korridoren im Korridor.\n\n", q.passed, q.total);
    }
    let Some(p) = &inn.progress else { return };
    let c = &p.config;
    let _ = write!(
        b,
        "Konfiguration: Welt {} · Bot {} · {} Spieler · {} Tage{}{} – {}.\n\n",
        c.world,
        c.bot,
        c.players,
        c.days,
        opt_text(" · Grad ", &c.grade),
        opt_text(" · Hilfen ", &c.helps),
        p.test_match
    );
    b.push_str("### Meilensteine\n\n| | Meilenstein | Seeds | Median-Tag |\n|---|---|---|---|\n");
    for r in &p.ladder {
        let arrow = if r.arrow.is_empty() { "·" } else { &r.arrow };
        let day = r.median_day.map_or("–".to_string(), |d| format!("Tag {d}"));
        let _ = writeln!(b, "| {arrow} | {} | {}/{} | {day} |", r.label, r.reached, r.runs);
    }
    if !inn.previous.is_empty() {
        let _ = write!(
            b,
            "\nPfeile gegen den letzten Lauf gleicher Konfiguration ({}).\n",
            inn.previous
        );
    }
    if !p.blocker.is_empty() {
        let _ = write!(b, "\n**Blocker:** {}\n", p.blocker);
    }
    b.push('\n');
}

fn opt_text(prefix: &str, v: &str) -> String {
    if v.is_empty() {
        String::new()
    } else {
        format!("{prefix}{v}")
    }
}
