//! Bericht von `task balance` (B-157): Pass/Fail je Ziel als JSON und Markdown, Vergleich mit einer Baseline. Die
//! Baseline ist selbst ein Summary-JSON (`testdata/balance/baseline.json`).

use std::collections::{HashMap, HashSet};
use std::fmt::Write as _;
use std::path::Path;

use serde::{Deserialize, Serialize};

use super::evaluate::{STATUS_INVALID, STATUS_NARROW, STATUS_OK, Verdict};
use super::figures::Figures;
use super::insights::{Insights, Meta};
use super::progress::{GoalView, Progress, QualityView, progress_of};
use super::report_types::{Report, Scenario, nil_empty, read_go};
use super::targets::{KIND_SHARE, ROLE_GOAL, ROLE_GUARD, Targets};
use crate::kitx::go_marshal_indent;

/// Ein bewertetes Ziel; `pass` gilt für „ok“ und „knapp“.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct Entry {
    #[serde(flatten)]
    pub verdict: Verdict,
    pub pass: bool,
}

/// Der Bericht: alle Ziele mit Urteil. `seeds` ist die Zahl der gespielten Seeds.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default, rename_all = "camelCase")]
pub struct Summary {
    pub version: i64,
    pub seeds: i64,
    /// dev, all oder auswahl; fehlt in älteren Berichten.
    #[serde(skip_serializing_if = "String::is_empty")]
    pub seed_mode: String,
    /// Die Seeds bei höchstens 5, sonst leer.
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub seed_names: Vec<String>,
    /// Alle Ziele mit Rolle goal erreicht; ohne Rollen (alte Berichte) alle Ziele.
    pub pass: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub goal: Option<GoalView>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quality: Option<QualityView>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub progress: Option<Progress>,
    #[serde(serialize_with = "nil_empty")]
    pub targets: Vec<Entry>,
    /// Wert-Varianten des Laufs (`datei.json:a.b.c=wert`).
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub varianten: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kennzahlen: Option<Figures>,
}

/// Ein Ziel, dessen Bewertung gegenüber der Baseline anders ist; `tipped`: Pass wechselt.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct Change {
    pub name: String,
    pub before: String,
    pub after: String,
    pub tipped: bool,
}

impl Targets {
    /// Bewertet einen Bericht gegen die Ziele. `root` liefert die Daten des Spielfortschritts.
    pub fn summarize(&self, root: &Path, r: &Report) -> Summary {
        let seeds = &r.matrix.seeds;
        let mut s = Summary {
            version: 1,
            seeds: i64::try_from(seeds.len()).unwrap_or(i64::MAX),
            ..Summary::default()
        };
        if !self.seeds.is_empty() {
            s.seed_mode = self.seed_mode_of(seeds).to_owned();
        }
        if seeds.len() <= 5 {
            s.seed_names.clone_from(seeds);
        }
        let mut goal = GoalView::default();
        let mut quality = QualityView::default();
        for mut v in self.evaluate(r) {
            let role = v.target.role().to_owned();
            if role != ROLE_GOAL && v.status == STATUS_NARROW {
                STATUS_OK.clone_into(&mut v.status); // „knapp“ gibt es nur bei Zielen (B-648)
            }
            let pass = v.status == STATUS_OK || v.status == STATUS_NARROW;
            if role == ROLE_GOAL {
                goal.total += 1;
                goal.hit += i64::from(pass);
            } else if role == ROLE_GUARD {
                quality.total += 1;
                quality.passed += i64::from(pass);
            }
            s.targets.push(Entry { verdict: v, pass });
        }
        goal.reached = goal.hit == goal.total;
        s.pass = goal.reached;
        s.goal = Some(goal);
        if quality.total > 0 {
            s.quality = Some(quality);
        }
        if let Some(sc) = self.goal_scenario() {
            s.progress = progress_of(root, r, &sc, seeds);
        }
        s
    }

    /// Szenario des ersten Ziels mit Rolle goal und Welt (Insel-Lauf).
    fn goal_scenario(&self) -> Option<Scenario> {
        let g = self
            .targets
            .iter()
            .find(|g| !g.world.is_empty() && g.role() == ROLE_GOAL)?;
        Some(Scenario {
            players: g.players,
            bot: g.bot.clone(),
            days: g.days,
            grade: g.grade.clone(),
            world: g.world.clone(),
            helps: g.helps.clone(),
            ..Scenario::default()
        })
    }
}

/// Liest ein Summary-JSON (Baseline).
pub fn read_summary(raw: &[u8]) -> Result<Summary, String> {
    read_go(raw).map_err(|e| format!("baseline: {e}"))
}

/// Ziele, deren Status sich gegenüber der Baseline geändert hat, in der Reihenfolge von `cur` (entfallene am Ende).
pub fn compare(base: &Summary, cur: &Summary) -> Vec<Change> {
    let before: HashMap<String, &Entry> = base.targets.iter().map(|e| (e.target_key(), e)).collect();
    let mut seen = HashSet::new();
    let mut out = Vec::new();
    let change = |e: &Entry, before: &str, after: &str, tipped| Change {
        name: e.verdict.target.name.clone(),
        before: before.to_owned(),
        after: after.to_owned(),
        tipped,
    };
    for e in &cur.targets {
        let key = e.target_key();
        // Wie Go: ein zweites gleiches Ziel in `cur` gilt als neu, weil der Eintrag schon verbraucht ist.
        let old = before.get(&key).filter(|_| !seen.contains(&key));
        seen.insert(key);
        match old {
            None => out.push(change(e, "neu", &e.verdict.status, !e.pass)),
            Some(b) if b.verdict.status != e.verdict.status => {
                out.push(change(e, &b.verdict.status, &e.verdict.status, b.pass != e.pass));
            }
            Some(_) => {}
        }
    }
    for b in base.targets.iter().filter(|b| !seen.contains(&b.target_key())) {
        out.push(change(b, &b.verdict.status, "entfällt", b.pass));
    }
    out
}

impl Summary {
    /// Der Bericht eingerückt mit Zeilenende.
    pub fn json(&self) -> Result<String, String> {
        Ok(go_marshal_indent(self)? + "\n")
    }

    /// Auswertung, Tabelle je Ziel und Vergleich; `base` `None`: Hinweis „keine Baseline“.
    pub fn markdown(&self, base: Option<&Summary>) -> String {
        let meta = Meta {
            baseline: "Baseline".to_owned(),
            ..Meta::default()
        };
        let insights = self.insights(base, meta).ok();
        self.markdown_with(base, insights.as_ref())
    }

    /// Wie [`Summary::markdown`] mit einer fertigen Auswertung; `None`: ohne Auswertung.
    pub fn markdown_with(&self, base: Option<&Summary>, insights: Option<&Insights>) -> String {
        let mut verdict = if self.pass { "Pass" } else { "Fail" };
        if let Some(goal) = self.goal.as_ref().filter(|g| g.total > 0) {
            verdict = if goal.reached {
                "Ziel erreicht"
            } else {
                "Ziel nicht erreicht"
            };
        }
        let mut b = format!(
            "# Balancing-Bericht\n\nGesamt: **{verdict}**, {}, Ziele nach `data/balance-targets.json`.\n\n",
            self.seed_text()
        );
        if let Some(insights) = insights {
            b.push_str(&insights.markdown());
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
        self.comparison(base, &mut b);
        b
    }

    fn comparison(&self, base: Option<&Summary>, b: &mut String) {
        b.push_str("\n## Vergleich mit der Baseline\n\n");
        let Some(base) = base else {
            b.push_str(
                "Keine Baseline vorhanden (`testdata/balance/baseline.json`); `task balance:baseline` legt sie an.\n",
            );
            return;
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
        for c in changes {
            let mark = if c.tipped { " **gekippt**" } else { "" };
            let _ = writeln!(b, "- {}: {} → {}{mark}", c.name, c.before, c.after);
        }
    }
}

/// Go `strconv.FormatFloat(x, 'f', prec, 64)`; `prec` `None`: kürzeste Form. Anders als Rust schreibt Go `+Inf`.
pub(crate) fn go_fixed(x: f64, prec: Option<usize>) -> String {
    if x.is_infinite() {
        return if x > 0.0 { "+Inf" } else { "-Inf" }.to_owned();
    }
    match prec {
        Some(p) => format!("{x:.p$}"),
        None => format!("{x}"),
    }
}

/// Zahl mit Dezimalkomma in kürzester Form.
pub(crate) fn num(x: f64) -> String {
    go_fixed(x, None).replace('.', ",")
}

impl Entry {
    /// Unterscheidet Ziele auch bei gleichem Namen (Szenario gehört dazu).
    pub(crate) fn target_key(&self) -> String {
        let t = &self.verdict.target;
        format!(
            "{}|{}|{}|{}|{}|{}|{}|{}|{}",
            t.measure, t.name, t.players, t.bot, t.depth, t.days, t.grade, t.world, t.helps
        )
    }

    fn unit(&self) -> &'static str {
        if self.verdict.target.kind == KIND_SHARE {
            " %"
        } else {
            ""
        }
    }

    fn value_text(&self) -> String {
        if self.verdict.status == STATUS_INVALID {
            return "–".to_owned();
        }
        go_fixed(self.verdict.value, Some(1)).replace('.', ",") + self.unit()
    }

    /// Ohne jede Grenze (nur in kaputten Dateien) „–“; Go stürzt dort ab.
    fn bounds_text(&self) -> String {
        let unit = self.unit();
        match (self.verdict.target.lower, self.verdict.target.upper) {
            (Some(lo), Some(hi)) => format!("{}–{}{unit}", num(lo), num(hi)),
            (Some(lo), None) => format!("≥ {}{unit}", num(lo)),
            (None, Some(hi)) => format!("≤ {}{unit}", num(hi)),
            (None, None) => "–".to_owned(),
        }
    }

    fn status_text(&self) -> String {
        let word = if self.pass { "Pass" } else { "Fail" };
        format!("{word} ({})", self.verdict.status)
    }

    fn seeds_text(&self) -> String {
        let mut parts = Vec::new();
        if !self.verdict.fail_seeds.is_empty() {
            parts.push(self.verdict.fail_seeds.join(", "));
        }
        if !self.verdict.invalid_seeds.is_empty() {
            parts.push(format!("ungültig: {}", self.verdict.invalid_seeds.join(", ")));
        }
        if parts.is_empty() {
            return "–".to_owned();
        }
        parts.join("; ")
    }
}
