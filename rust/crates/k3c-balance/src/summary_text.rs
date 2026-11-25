//! Markdown des Berichts (Port von `Summary.Markdown`/`MarkdownWith` und den Textfunktionen aus `summary.go`).

use std::fmt::Write;

use crate::evaluate::STATUS_INVALID;
use crate::seedmode::seed_text;
use crate::summary::{Entry, Summary, compare};
use crate::targets::KIND_SHARE;

/// Eine Zahl so kurz wie möglich mit Komma als Dezimalzeichen (Go: `num`, `strconv.FormatFloat(x, 'f', -1, 64)`).
pub fn num(x: f64) -> String {
    format!("{x}").replace('.', ",")
}

impl Entry {
    pub(crate) fn unit(&self) -> &'static str {
        if self.verdict.target.kind == KIND_SHARE {
            " %"
        } else {
            ""
        }
    }

    pub(crate) fn value_text(&self) -> String {
        if self.verdict.status == STATUS_INVALID {
            return "–".into();
        }
        format!("{:.1}", self.verdict.value).replace('.', ",") + self.unit()
    }

    pub(crate) fn bounds_text(&self) -> String {
        let t = &self.verdict.target;
        match (t.lower, t.upper) {
            (Some(lo), Some(hi)) => format!("{}–{}{}", num(lo), num(hi), self.unit()),
            (Some(lo), None) => format!("≥ {}{}", num(lo), self.unit()),
            (None, hi) => format!("≤ {}{}", num(hi.unwrap_or_default()), self.unit()),
        }
    }

    fn status_text(&self) -> String {
        let word = if self.pass { "Pass" } else { "Fail" };
        format!("{word} ({})", self.verdict.status)
    }

    fn seeds_text(&self) -> String {
        let mut parts: Vec<String> = Vec::new();
        if !self.verdict.fail_seeds.is_empty() {
            parts.push(self.verdict.fail_seeds.join(", "));
        }
        if !self.verdict.invalid_seeds.is_empty() {
            parts.push(format!("ungültig: {}", self.verdict.invalid_seeds.join(", ")));
        }
        if parts.is_empty() {
            return "–".into();
        }
        parts.join("; ")
    }
}

impl Summary {
    /// Die Kopfzeile zu Seeds und Modus (Go: `seedText`).
    pub fn seed_text(&self) -> String {
        seed_text(self.seeds, &self.seed_mode, &self.seed_names)
    }

    /// Schreibt Auswertung, Tabelle je Ziel und Vergleich (Go: `MarkdownWith`). `insights` ist der fertige Text der Auswertung
    /// (B-632, Go: `Insights.Markdown()`; kommt mit RU31), `None` = ohne Auswertung; `base == None`: Hinweis „keine Baseline“.
    pub fn markdown_with(&self, base: Option<&Summary>, insights: Option<&str>) -> String {
        let mut b = String::new();
        let mut verdict = if self.pass { "Pass" } else { "Fail" };
        if let Some(g) = self.goal.as_ref().filter(|g| g.total > 0) {
            verdict = if g.reached {
                "Ziel erreicht"
            } else {
                "Ziel nicht erreicht"
            };
        }
        let _ = write!(
            b,
            "# Balancing-Bericht\n\nGesamt: **{verdict}**, {}, Ziele nach `data/balance-targets.json`.\n\n",
            self.seed_text()
        );
        if let Some(text) = insights {
            b.push_str(text);
            b.push_str("\n## Ziele im Detail\n\n");
        }
        b.push_str(
            "| Ziel | Szenario | Wert | Grenzen | Bewertung | Seeds zum Nachspielen |\n|---|---|---|---|---|---|\n",
        );
        for e in &self.targets {
            let t = &e.verdict.target;
            let _ = writeln!(
                b,
                "| {} | {} Spieler, {}, Tiefe {}, {} Tage | {} | {} | {} | {} |",
                t.name,
                t.players,
                t.bot,
                t.depth,
                t.days,
                e.value_text(),
                e.bounds_text(),
                e.status_text(),
                e.seeds_text()
            );
        }
        b.push_str("\n## Vergleich mit der Baseline\n\n");
        let Some(base) = base else {
            b.push_str(
                "Keine Baseline vorhanden (`testdata/balance/baseline.json`); `task balance:baseline` legt sie an.\n",
            );
            return b;
        };
        if base.seeds != self.seeds {
            let _ = write!(
                b,
                "Hinweis: Die Baseline hat {} Seeds, dieser Lauf {}.\n\n",
                base.seeds, self.seeds
            );
        }
        let changes = compare(base, self);
        if changes.is_empty() {
            b.push_str("Keine Bewertung hat sich geändert.\n");
        }
        for c in &changes {
            let mark = if c.tipped { " **gekippt**" } else { "" };
            let _ = writeln!(b, "- {}: {} → {}{mark}", c.name, c.before, c.after);
        }
        b
    }
}
