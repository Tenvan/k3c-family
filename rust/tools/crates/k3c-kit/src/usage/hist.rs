//! Laufzeit-Histogramm und Normalisierung der Argumente (Port von `hist.go`).

use std::collections::BTreeMap;

use serde::{Deserialize, Deserializer, Serialize, de::Error};
use serde_json::Value;

use super::nullable;

/// Faktor zwischen zwei Bucket-Grenzen; das geometrische Mittel eines Buckets liegt höchstens um den Faktor √1,25
/// (rund 12 %) neben jedem Wert darin.
const HIST_BASE: f64 = 1.25;
/// Genauigkeit der Perzentile in Prozent (√HIST_BASE − 1, gerundet); die Oberfläche nennt sie in der Erklärzeile.
pub const PERCENTILE_ERROR_PCT: i64 = 12;
/// Verschiedene Argumente bzw. Fehlermeldungen je Tool; der Rest zählt unter [`OVERFLOW_KEY`].
pub(super) const MAX_VALUES: usize = 50;
pub(super) const OVERFLOW_KEY: &str = "(weitere)";
/// Länge, auf die Argumente und Fehlermeldungen gekürzt werden.
const VALUE_RUNES: usize = 120;

/// Laufzeiten: Anzahl, Summe, Maximum und Histogramm. Die JSON-Namen gelten auch für die Datei.
#[derive(Debug, Default, Serialize, Deserialize)]
#[serde(default)]
pub(super) struct DurAgg {
    #[serde(deserialize_with = "nullable")]
    pub calls: i64,
    #[serde(rename = "sumMs", deserialize_with = "nullable")]
    pub sum_ms: f64,
    #[serde(rename = "maxMs", deserialize_with = "nullable")]
    pub max_ms: f64,
    #[serde(deserialize_with = "hist_map")]
    pub hist: BTreeMap<i64, i64>,
}

/// Bucket-Nummern stehen als Text in der Datei. Selbst geparst, weil `serde(flatten)` in `ToolAgg` die Schlüssel
/// puffert und sie dann nicht mehr als Zahl liest.
fn hist_map<'de, D: Deserializer<'de>>(de: D) -> Result<BTreeMap<i64, i64>, D::Error> {
    let raw: Option<BTreeMap<String, Option<i64>>> = Option::deserialize(de)?;
    raw.unwrap_or_default()
        .into_iter()
        .map(|(k, v)| Ok((k.parse().map_err(D::Error::custom)?, v.unwrap_or_default())))
        .collect()
}

impl DurAgg {
    pub(super) fn add(&mut self, ms: f64) {
        self.calls += 1;
        self.sum_ms += ms;
        self.max_ms = self.max_ms.max(ms);
        *self.hist.entry(hist_index(ms)).or_default() += 1;
    }

    pub(super) fn merge(&mut self, other: &DurAgg) {
        self.calls += other.calls;
        self.sum_ms += other.sum_ms;
        self.max_ms = self.max_ms.max(other.max_ms);
        for (k, v) in &other.hist {
            *self.hist.entry(*k).or_default() += v;
        }
    }

    pub(super) fn avg(&self) -> f64 {
        if self.calls == 0 {
            return 0.0;
        }
        self.sum_ms / self.calls as f64
    }

    /// Perzentil `q` (0..1) aus dem Histogramm, nie über dem echten Maximum.
    pub(super) fn p(&self, q: f64) -> f64 {
        if self.calls == 0 {
            return 0.0;
        }
        let rank = (q * self.calls as f64).ceil() as i64;
        let mut seen = 0;
        for (k, n) in &self.hist {
            seen += n;
            if seen >= rank {
                return hist_value(*k).min(self.max_ms);
            }
        }
        self.max_ms
    }
}

/// Bucket einer Dauer: (1,25^(i-1), 1,25^i]; alles bis 1 ms liegt in Bucket 0.
fn hist_index(ms: f64) -> i64 {
    if ms <= 1.0 {
        return 0;
    }
    (ms.ln() / HIST_BASE.ln()).ceil() as i64
}

/// Schätzwert eines Buckets: das geometrische Mittel seiner Grenzen.
fn hist_value(i: i64) -> f64 {
    if i <= 0 {
        return 0.5;
    }
    HIST_BASE.powf(i as f64 - 0.5)
}

/// Macht gleiche Argumente gleich: Schlüssel sortiert, kompakt, gekürzt. Kein JSON bleibt Text.
pub(super) fn norm_args(raw: &str) -> String {
    if raw.is_empty() || raw == "null" {
        return "{}".to_owned();
    }
    match serde_json::from_str::<Value>(raw) {
        Ok(v) => {
            let mut out = String::new();
            write_go_json(&v, &mut out);
            clip(&out)
        }
        Err(_) => clip(raw),
    }
}

/// Schreibt wie Go `json.Marshal` eines `any`, damit die Schlüssel aus einer Go-Datei weiterzählen: Zahlen als f64,
/// Objekte sortiert, `<`, `>`, `&`, U+2028 und U+2029 maskiert.
fn write_go_json(v: &Value, out: &mut String) {
    match v {
        Value::Null => out.push_str("null"),
        Value::Bool(b) => out.push_str(if *b { "true" } else { "false" }),
        Value::Number(n) => out.push_str(&go_number(n.as_f64().unwrap_or_default())),
        Value::String(s) => write_go_string(s, out),
        Value::Array(list) => {
            out.push('[');
            for (i, item) in list.iter().enumerate() {
                if i > 0 {
                    out.push(',');
                }
                write_go_json(item, out);
            }
            out.push(']');
        }
        Value::Object(map) => {
            // Selbst sortiert: ein anderes Crate kann `preserve_order` von serde_json einschalten.
            let mut entries: Vec<_> = map.iter().collect();
            entries.sort_by(|a, b| a.0.cmp(b.0));
            out.push('{');
            for (i, (key, item)) in entries.into_iter().enumerate() {
                if i > 0 {
                    out.push(',');
                }
                write_go_string(key, out);
                out.push(':');
                write_go_json(item, out);
            }
            out.push('}');
        }
    }
}

fn write_go_string(s: &str, out: &mut String) {
    let quoted = Value::from(s).to_string();
    for c in quoted.chars() {
        match c {
            '<' | '>' | '&' | '\u{2028}' | '\u{2029}' => out.push_str(&format!("\\u{:04x}", u32::from(c))),
            _ => out.push(c),
        }
    }
}

fn go_number(f: f64) -> String {
    let abs = f.abs();
    if abs != 0.0 && !(1e-6..1e21).contains(&abs) {
        let e = format!("{f:e}");
        return if e.contains("e-") { e } else { e.replace('e', "e+") };
    }
    format!("{f}")
}

pub(super) fn clip(s: &str) -> String {
    if s.chars().count() <= VALUE_RUNES {
        return s.to_owned();
    }
    let mut out: String = s.chars().take(VALUE_RUNES - 1).collect();
    out.push('…');
    out
}

/// Zählt einen Wert und liefert den Schlüssel, unter dem er zählt: ab [`MAX_VALUES`] verschiedenen Werten
/// [`OVERFLOW_KEY`].
pub(super) fn bump(map: &mut BTreeMap<String, i64>, value: &str) -> String {
    let key = if !map.contains_key(value) && map.len() >= MAX_VALUES {
        OVERFLOW_KEY
    } else {
        value
    };
    *map.entry(key.to_owned()).or_default() += 1;
    key.to_owned()
}
