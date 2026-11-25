//! Sensitivitäts-Lauf (Port von `engine/balance/sensitivity.go`, B-158, BAL3.3): Je Pfad und Stufe die Ziele aus
//! data/balance-targets.json neu rechnen und die gekippten nennen, also die, deren Bewertung (ok, knapp, verletzt, ungültig)
//! sich gegenüber dem unveränderten Lauf ändert. Dazu Grad-Kurven (`curves.rs`). Nichts wird automatisch geändert, nur
//! berichtet.

use std::fmt::Write;

use k3c_data::balance_targets::{Target, Targets};
use serde::{Deserialize, Serialize};

use crate::curves::{CURVE_SCENARIO, GradeCurve, curves, dec1};
use crate::fehler::Fehler;
use crate::run::Tick;
use crate::seedmode::SeedModes;
use crate::summary::{Summarize, Summary};
use crate::summary_text::num;
use crate::targets::KIND_SHARE;
use crate::vary::{STEPS, varied_source, with_data};

/// `paths` leer = keine Variation, `steps` leer = `STEPS`, `seeds` 0 = alle Seeds der Datei, `nights` 0 = keine Grad-Kurven
/// (Beschluss BAL3.1: `CURVE_NIGHTS`). `tick` (Go: `Targets.Tick`) erfährt jeden Lauf und jede Variation.
#[derive(Debug, Clone, Default)]
pub struct SensitivityOptions {
    pub paths: Vec<String>,
    pub steps: Vec<i64>,
    pub seeds: i64,
    pub nights: i64,
    pub tick: Option<Tick>,
}

/// Der Bericht; feste Feldreihenfolge, keine Wanduhr, damit gleiche Eingaben byte-gleich sind.
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
#[serde(default)]
pub struct SensitivityReport {
    pub version: i64,
    pub seeds: i64,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub variations: Vec<Variation>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub curves: Vec<GradeCurve>,
}

/// Ein Pfad auf einer Stufe: Wert in data/ vorher und nachher, gekippte Ziele.
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct Variation {
    pub path: String,
    pub percent: i64,
    pub before: f64,
    pub after: f64,
    pub flips: Vec<Flip>,
}

/// Ein gekipptes Ziel mit Bewertung und Wert (Anteil in %, sonst Median) vorher und nachher.
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Flip {
    pub name: String,
    pub scenario: String,
    pub kind: String,
    pub before: String,
    pub after: String,
    pub value_before: f64,
    pub value_after: f64,
}

/// Variationen und Kurven (Go: `Targets.Sensitivity`).
pub trait Sensitivity {
    /// Rechnet Variationen und Kurven. Alle Pfade werden vor dem ersten Lauf geprüft.
    fn sensitivity(&self, o: &SensitivityOptions) -> Result<SensitivityReport, Fehler>;
}

impl Sensitivity for Targets {
    fn sensitivity(&self, o: &SensitivityOptions) -> Result<SensitivityReport, Fehler> {
        // Sensitivität und Kurven brauchen viele Seeds; der Seed-Modus dev gilt hier nicht (B-634)
        let t = self.all_seeds();
        let steps: &[i64] = if o.steps.is_empty() { &STEPS } else { &o.steps };
        let mut jobs = Vec::new();
        for p in &o.paths {
            for &pct in steps {
                let (src, before, after) = varied_source(p, pct)?;
                let v = Variation {
                    path: p.clone(),
                    percent: pct,
                    before,
                    after,
                    flips: Vec::new(),
                };
                jobs.push((v, src));
            }
        }
        let mut rep = SensitivityReport {
            version: 1,
            seeds: t.seed_list(o.seeds).len() as i64,
            ..SensitivityReport::default()
        };
        if !jobs.is_empty() {
            let base = summary(&t, o)?;
            for (mut v, src) in jobs {
                tick(o);
                v.flips = with_data(src, || Ok(flips(&base, &summary(&t, o)?)))?;
                rep.variations.push(v);
            }
        }
        if o.nights > 0 {
            tick(o);
            rep.curves = curves(t.seed_list(o.seeds), o.nights)?;
        }
        Ok(rep)
    }
}

fn tick(o: &SensitivityOptions) {
    if let Some(t) = &o.tick {
        (t.0)();
    }
}

fn summary(t: &Targets, o: &SensitivityOptions) -> Result<Summary, Fehler> {
    let r = t.run_targets_progress(o.seeds, o.tick.clone(), None)?;
    Ok(t.summarize(&r))
}

/// Ziele mit anderer Bewertung; beide Summaries stammen aus denselben Zielen (gleiche Reihenfolge; Go: `flips`).
fn flips(base: &Summary, cur: &Summary) -> Vec<Flip> {
    let mut out = Vec::new();
    for (b, c) in base.targets.iter().zip(&cur.targets) {
        if b.verdict.status != c.verdict.status {
            out.push(Flip {
                name: b.verdict.target.name.clone(),
                scenario: scenario_text(&b.verdict.target),
                kind: b.verdict.target.kind.clone(),
                before: b.verdict.status.clone(),
                after: c.verdict.status.clone(),
                value_before: b.verdict.value,
                value_after: c.verdict.value,
            });
        }
    }
    out
}

/// Nennt das Szenario eines Ziels; der Grad nur, wenn er gesetzt ist (leer = normal; Go: `scenarioText`).
pub fn scenario_text(g: &Target) -> String {
    let mut s = format!("{} Spieler, {}, Tiefe {}, {} Tage", g.players, g.bot, g.depth, g.days);
    if !g.grade.is_empty() {
        s += &format!(", Grad {}", g.grade);
    }
    s
}

impl SensitivityReport {
    /// Schreibt den Bericht eingerückt mit Zeilenende (Go: `JSON`).
    pub fn json(&self) -> Result<String, Fehler> {
        let mut s = serde_json::to_string_pretty(self).map_err(|e| Fehler::Daten(format!("balance: {e}")))?;
        s.push('\n');
        Ok(s)
    }

    /// Tabelle der Variationen mit gekippten Zielen, dann je Grad die Kurve (Go: `Markdown`).
    pub fn markdown(&self) -> String {
        let mut b = String::new();
        let _ = write!(
            b,
            "# Sensitivitäts-Bericht\n\n{} Seeds, Ziele nach `data/balance-targets.json`. Gekippt: Die Bewertung ändert sich \
             gegenüber dem Lauf mit unverändertem Wert. `data/*.json` bleiben unverändert.\n",
            self.seeds
        );
        if !self.variations.is_empty() {
            b.push_str("\n## Variationen\n\n| Pfad | Stufe | Wert | Gekippte Ziele |\n|---|---|---|---|\n");
            for v in &self.variations {
                let _ = writeln!(
                    b,
                    "| `{}` | {:+} % | {} → {} | {} |",
                    v.path,
                    v.percent,
                    num(v.before),
                    num(v.after),
                    flips_text(&v.flips)
                );
            }
        }
        if !self.curves.is_empty() {
            let _ = write!(
                b,
                "\n## Grad-Kurven\n\n{CURVE_SCENARIO}; je Nacht: Anteil der Seeds, in denen die Burg hält, Median zerstörter \
                 Gebäude der Welle, Median des Golds aller Monarchen zur Dämmerung.\n"
            );
            for c in &self.curves {
                c.markdown(&mut b);
            }
        }
        b
    }
}

fn flips_text(flips: &[Flip]) -> String {
    if flips.is_empty() {
        return "–".into();
    }
    let parts: Vec<String> = flips
        .iter()
        .map(|f| {
            let unit = if f.kind == KIND_SHARE { " %" } else { "" };
            format!(
                "{} ({}): {} {}{unit} → {} {}{unit}",
                f.name,
                f.scenario,
                f.before,
                dec1(f.value_before),
                f.after,
                dec1(f.value_after)
            )
        })
        .collect();
    parts.join("; ")
}
