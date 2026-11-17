//! Golden lesen und als JSON-Baum vergleichen: Port der lesenden Teile von engine/internal/golden (B-043).
//!
//! Golden schreibt nur Go (`task go:golden`); hier gibt es kein `-update`. Bewusste Abweichung von Go
//! (Leitfaden § 3): `null` und die leere Liste gelten als gleich, weil Go eine `nil`-Liste als `null` schreibt.

use std::collections::BTreeSet;
use std::io::Read;
use std::path::{Path, PathBuf};

use serde::Serialize;
use serde_json::Value;

/// `testdata` ist der Ordner `testdata/` des Repos (Pfad ab diesem Crate, alle Crates liegen nebeneinander).
pub fn testdata() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("../../../testdata")
}

/// `read` liest eine Golden-Datei als JSON-Baum; endet der Name auf `.gz`, entpackt es sie (GD1, B-624).
/// Wie Gos `t.Fatalf` bricht ein Fehler mit Pfad und Grund ab.
pub fn read(path: impl AsRef<Path>) -> Value {
    let path = path.as_ref();
    let text = read_text(path).unwrap_or_else(|e| panic!("{}: {e}", path.display()));
    serde_json::from_str(&text).unwrap_or_else(|e| panic!("{}: {e}", path.display()))
}

fn read_text(path: &Path) -> std::io::Result<String> {
    let file = std::fs::File::open(path)?;
    let mut text = String::new();
    if path.extension().is_some_and(|e| e == "gz") {
        flate2::read::GzDecoder::new(file).read_to_string(&mut text)?;
    } else {
        std::io::BufReader::new(file).read_to_string(&mut text)?;
    }
    Ok(text)
}

/// `tree` wandelt v über JSON in einen generischen Baum, wie ihn Gos `golden.Tree` liefert.
pub fn tree<T: Serialize + ?Sized>(v: &T) -> Value {
    serde_json::to_value(v).unwrap_or_else(|e| panic!("JSON-Baum: {e}"))
}

/// `diff` liefert den Pfad der ersten Abweichung zwischen zwei JSON-Bäumen ("" = gleich). Zahlen als `f64`
/// mit denselben Bits; Meldungen wie Gos `golden.Diff`.
pub fn diff(at: &str, want: &Value, got: &Value) -> String {
    match want {
        Value::Object(w) => {
            let Value::Object(g) = got else {
                return at.to_string();
            };
            let keys: BTreeSet<&String> = w.keys().chain(g.keys()).collect();
            for k in keys {
                let (Some(wv), Some(gv)) = (w.get(k), g.get(k)) else {
                    return format!("{at}.{k} (Schlüssel nur auf einer Seite)");
                };
                let d = diff(&format!("{at}.{k}"), wv, gv);
                if !d.is_empty() {
                    return d;
                }
            }
            String::new()
        }
        Value::Array(w) => {
            let g = match got {
                Value::Array(g) if g.len() == w.len() => g,
                Value::Null if w.is_empty() => return String::new(),
                _ => return format!("{at} (Länge)"),
            };
            for (i, (wv, gv)) in w.iter().zip(g).enumerate() {
                let d = diff(&format!("{at}[{i}]"), wv, gv);
                if !d.is_empty() {
                    return d;
                }
            }
            String::new()
        }
        Value::Null if matches!(got, Value::Array(g) if g.is_empty()) => String::new(),
        _ if same_scalar(want, got) => String::new(),
        _ => format!("{at} = {}, erwartet {}", go_text(got), go_text(want)),
    }
}

fn same_scalar(want: &Value, got: &Value) -> bool {
    match (want, got) {
        (Value::Number(w), Value::Number(g)) => number(w).to_bits() == number(g).to_bits(),
        _ => want == got,
    }
}

fn number(n: &serde_json::Number) -> f64 {
    n.as_f64().unwrap_or_else(|| panic!("Zahl {n} passt nicht in f64"))
}

/// `go_text` schreibt einen Wert wie Gos `%v` auf dem Baum von `json.Unmarshal`.
fn go_text(v: &Value) -> String {
    match v {
        Value::Null => "<nil>".to_string(),
        Value::Bool(b) => b.to_string(),
        Value::Number(n) => go_float(number(n)),
        Value::String(s) => s.clone(),
        Value::Array(_) | Value::Object(_) => v.to_string(),
    }
}

/// `go_float` schreibt eine Zahl wie Gos `%v` (`%g` mit kürzester Darstellung, Exponent ab 1e+06 und unter 1e-04).
pub(crate) fn go_float(x: f64) -> String {
    if x == 0.0 {
        return if x.is_sign_negative() { "-0" } else { "0" }.to_string();
    }
    let sign = if x < 0.0 { "-" } else { "" };
    let e = format!("{:e}", x.abs()); // kürzeste Ziffern, z. B. 1.5e-7
    let (mant, exp) = e.split_once('e').expect("Exponent");
    let exp: i32 = exp.parse().expect("Exponent als Zahl");
    if !(-4..6).contains(&exp) {
        let es = if exp < 0 { '-' } else { '+' };
        return format!("{sign}{mant}e{es}{:02}", exp.abs());
    }
    let digits = mant.replace('.', "");
    if exp < 0 {
        let zeros = "0".repeat((-exp - 1) as usize);
        return format!("{sign}0.{zeros}{digits}");
    }
    let dp = (exp + 1) as usize;
    if digits.len() <= dp {
        return format!("{sign}{digits}{}", "0".repeat(dp - digits.len()));
    }
    format!("{sign}{}.{}", &digits[..dp], &digits[dp..])
}

#[cfg(test)]
#[path = "golden_tests.rs"]
mod golden_tests;
