//! Parameter des automatischen Balancings (Port von `engine/balance/tuneparam.go`, WZ28.6), soweit `insights_next` sie
//! braucht (RU31.2): `datei.json:a.b.c=min..max` zeigt auf eine Zahl in data/; gelesen werden Ausgangswert und Ganzzahligkeit.
//! Wert-Variante und Rundung stehen hier, Diff-Vorschlag in `tune_diff.rs`, das Tuning in `tune.rs` (RU31.5).

use k3c_data::Source;
use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::fehler::Fehler;

/// Suchmodus des Abstimmens (Go: `TuneSearch`).
pub const TUNE_SEARCH: &str = "search";
/// Höchstzahl der Parameter je Abstimmung (Go: `MaxTuneParams`).
pub const MAX_TUNE_PARAMS: usize = 2;

/// Ein abgestimmter Wert: Datei, Pfad, Bereich und Ausgangswert in data/. `int`: Der Wert ist in der Datei ganzzahlig,
/// alle Punkte werden gerundet.
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct TuneParam {
    pub file: String,
    pub path: String,
    pub min: f64,
    pub max: f64,
    pub base: f64,
    pub int: bool,
}

impl TuneParam {
    /// Schreibt einen Wert des Parameters (ganzzahlig gerundet bzw. auf vier Nachkommastellen; Go: `format`).
    pub fn format(&self, v: f64) -> String {
        if self.int {
            return format!("{}", v.round() as i64);
        }
        format!("{}", (v * 1e4).round() / 1e4)
    }

    /// Rundet einen Wert wie `format`, als Zahl (Go: `norm`).
    pub fn norm(&self, v: f64) -> f64 {
        self.format(v).parse().unwrap_or_default()
    }

    /// Die Wert-Variante `datei.json:a.b.c=wert` (`variant.rs`; Go: `variant`).
    pub fn variant(&self, v: f64) -> String {
        format!("{}:{}={}", self.file, self.path, self.format(v))
    }
}

/// Liest `datei.json:a.b.c=min..max` und den Ausgangswert aus data/ (Go: `ParseTuneParam`).
pub fn parse_tune_param(spec: &str) -> Result<TuneParam, Fehler> {
    let fail = |why: &str| Fehler::Daten(format!("parameter {spec:?}: {why}"));
    let (path, range) = spec.split_once('=').unwrap_or((spec, ""));
    let parts = (path.split_once(':'), range.split_once(".."));
    let (Some((file, keys)), Some((lo, hi))) = parts else {
        return Err(fail("erwartet datei.json:a.b.c=min..max"));
    };
    if !spec.contains('=') || keys.is_empty() {
        return Err(fail("erwartet datei.json:a.b.c=min..max"));
    }
    let (Ok(min), Ok(max)) = (lo.parse::<f64>(), hi.parse::<f64>()) else {
        return Err(fail("min..max: zwei Zahlen mit min < max"));
    };
    if min >= max || (max - min).is_infinite() {
        return Err(fail("min..max: zwei Zahlen mit min < max"));
    }
    let raw = Source::from_env().read(file).map_err(|e| fail(&e.to_string()))?;
    let root: Value = serde_json::from_str(&raw).map_err(|e| fail(&e.to_string()))?;
    let mut at = &root;
    for k in keys.split('.') {
        let next = match at {
            Value::Object(o) => o.get(k),
            Value::Array(a) => k.parse::<usize>().ok().and_then(|i| a.get(i)),
            _ => None,
        };
        at = next.ok_or_else(|| fail(&format!("{keys:?} fehlt")))?;
    }
    let Value::Number(n) = at else {
        return Err(fail(&format!("{keys:?} ist keine Zahl")));
    };
    Ok(TuneParam {
        file: file.into(),
        path: keys.into(),
        min,
        max,
        base: n.as_f64().unwrap_or_default(),
        int: !n.is_f64(), // „1“ ist ganzzahlig, „1.0“ und „1e3“ nicht (Go: kein `.eE` im Text)
    })
}
