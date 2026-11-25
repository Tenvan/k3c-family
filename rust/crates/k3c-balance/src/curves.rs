//! Grad-Kurven (Port von `engine/balance/curves.go`, B-158, BAL3.3, Beschluss BAL3.1): je Schwierigkeitsgrad aus
//! data/difficulty.json eine Tabelle über die Nächte 1 bis `CURVE_NIGHTS` im Standardszenario (Wald, 2 Spieler, saver).

use std::cmp::Ordering;
use std::fmt::Write;

use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::economy_measures::b2f;
use crate::evaluate::median;
use crate::fehler::Fehler;
use crate::island_world::base_shared;
use crate::run::{Matrix, Report, run};

/// Nächte je Kurve (Beschluss BAL3.1; Go: `CurveNights`).
pub const CURVE_NIGHTS: i64 = 5;

/// Das Szenario der Kurven als Text (Go: `curveScenario`).
pub const CURVE_SCENARIO: &str = "Wald, 2 Spieler, `saver`";

/// Die Kurve eines Grads; `invalid` zählt abgebrochene Läufe (sie fehlen in allen Werten).
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct GradeCurve {
    pub grade: String,
    pub invalid: i64,
    pub nights: Vec<NightPoint>,
}

/// `castle_held` = Anteil der gültigen Läufe in %, deren Burg die Welle der Nacht hält; `destroyed` = Median der zerstörten
/// Gebäude dieser Welle; `gold_at_dusk` = Median des Golds aller Monarchen zur Dämmerung des Tags. `None` = kein Messwert.
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NightPoint {
    pub night: i64,
    pub castle_held: f64,
    pub destroyed: Option<f64>,
    pub gold_at_dusk: Option<f64>,
}

/// Die Grade aus data/difficulty.json, nach Wellengröße sortiert (dev, easy, normal, hard, ultra; Go: `Grades`).
pub fn grades() -> Result<Vec<String>, Fehler> {
    let fail = |e: String| Fehler::Daten(format!("data/difficulty.json: {e}"));
    let raw = base_shared()?
        .source
        .read("difficulty.json")
        .map_err(|e| fail(e.to_string()))?;
    let root: Value = serde_json::from_str(&raw).map_err(|e| fail(e.to_string()))?;
    let size = |g: &Value| g.get("waveSize").and_then(Value::as_f64).unwrap_or(0.0);
    let mut grades: Vec<(String, f64)> = match root.get("grades") {
        Some(Value::Object(m)) => m.iter().map(|(n, g)| (n.clone(), size(g))).collect(),
        _ => Vec::new(),
    };
    grades.sort_by(|a, b| {
        a.1.partial_cmp(&b.1)
            .unwrap_or(Ordering::Equal)
            .then_with(|| a.0.cmp(&b.0))
    });
    Ok(grades.into_iter().map(|(n, _)| n).collect())
}

/// Spielt das Standardszenario je Grad über die Seeds und `nights` Tage (Go: `Curves`).
pub fn curves(seeds: &[String], nights: i64) -> Result<Vec<GradeCurve>, Fehler> {
    let mut out = Vec::new();
    for g in grades()? {
        let r = run(&Matrix {
            seeds: seeds.to_vec(),
            players: vec![2],
            bots: vec!["saver".into()],
            depths: vec![0],
            days: nights,
            grade: g.clone(),
            ..Matrix::default()
        })?;
        out.push(curve(&g, &r, nights));
    }
    Ok(out)
}

/// Die Kurve eines Grads aus den Läufen (Go: `curve`).
fn curve(grade: &str, r: &Report, nights: i64) -> GradeCurve {
    let mut c = GradeCurve {
        grade: grade.to_string(),
        ..GradeCurve::default()
    };
    let valid_runs = || {
        r.results
            .iter()
            .filter_map(|res| res.metrics.as_ref().filter(|_| res.valid))
    };
    let valid = valid_runs().count();
    c.invalid = (r.results.len() - valid) as i64;
    for n in 1..=nights {
        let mut held = 0.0;
        let (mut destroyed, mut gold) = (Vec::new(), Vec::new());
        for m in valid_runs() {
            for w in m.waves.iter().filter(|w| w.wave == n) {
                held += b2f(w.survived);
                destroyed.push(w.buildings_destroyed as f64);
            }
            if let Some(g) = m.days.get((n - 1) as usize).and_then(|d| d.gold_at_dusk) {
                gold.push(g as f64);
            }
        }
        let mut p = NightPoint {
            night: n,
            destroyed: median_or_nil(&destroyed),
            gold_at_dusk: median_or_nil(&gold),
            ..NightPoint::default()
        };
        if valid > 0 {
            p.castle_held = 100.0 * held / valid as f64;
        }
        c.nights.push(p);
    }
    c
}

/// Der Median oder `None` ohne Werte (Go: `medianOrNil`).
pub fn median_or_nil(vals: &[f64]) -> Option<f64> {
    if vals.is_empty() { None } else { Some(median(vals)) }
}

impl GradeCurve {
    /// Hängt die Tabelle des Grads an (Go: `markdown`).
    pub fn markdown(&self, b: &mut String) {
        let _ = write!(b, "\n### {}\n\n", self.grade);
        if self.invalid > 0 {
            let _ = write!(b, "{} Läufe abgebrochen (nicht mitgezählt).\n\n", self.invalid);
        }
        b.push_str(
            "| Nacht | Burg hält | Zerstörte Gebäude (Median) | Gold zur Dämmerung (Median) |\n|---|---|---|---|\n",
        );
        for p in &self.nights {
            let _ = writeln!(
                b,
                "| {} | {} % | {} | {} |",
                p.night,
                dec1(p.castle_held),
                opt_dec1(p.destroyed),
                opt_dec1(p.gold_at_dusk)
            );
        }
    }
}

/// Eine Zahl mit einer Nachkommastelle und Komma (Go: `dec1`).
pub fn dec1(x: f64) -> String {
    format!("{x:.1}").replace('.', ",")
}

fn opt_dec1(x: Option<f64>) -> String {
    x.map_or_else(|| "–".to_string(), dec1)
}
