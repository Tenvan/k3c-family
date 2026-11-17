//! Vergleich mit Go: `testdata/rust/det-vectors.json.gz` aus `engine/internal/detvec` (RU3/AC-02).

use std::collections::BTreeMap;
use std::io::Read;

use serde_json::Value;

use crate::{exp, fmax, fmin, log, pow};

const PATH: &str = concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../../testdata/rust/det-vectors.json.gz"
);

fn hex(v: &Value) -> f64 {
    let s = v.as_str().expect("Hex-Text");
    f64::from_bits(u64::from_str_radix(s, 16).expect("16 Hex-Stellen"))
}

fn rows() -> Vec<Value> {
    let file = std::fs::File::open(PATH).expect("det-vectors.json.gz öffnen");
    let mut text = String::new();
    flate2::read::GzDecoder::new(file)
        .read_to_string(&mut text)
        .expect("det-vectors.json.gz entpacken");
    let v: Value = serde_json::from_str(&text).expect("det-vectors.json.gz parsen");
    v.as_array().expect("Liste").clone()
}

#[test]
fn vektoren_wie_go() {
    let mut count: BTreeMap<String, usize> = BTreeMap::new();
    let mut fails = Vec::new();
    for r in rows() {
        let f = r["f"].as_str().expect("f").to_string();
        let a = hex(&r["a"]);
        let want = hex(&r["r"]);
        let got = match f.as_str() {
            "pow" => pow(a, hex(&r["b"])),
            "exp" => exp(a),
            "log" => log(a),
            "fmax" => fmax(a, hex(&r["b"])),
            "fmin" => fmin(a, hex(&r["b"])),
            other => panic!("unbekannte Funktion {other}"),
        };
        if got.to_bits() != want.to_bits() && fails.len() < 10 {
            fails.push(format!(
                "{f}({}, {}) = {:016x}, Go {:016x}",
                r["a"],
                r["b"],
                got.to_bits(),
                want.to_bits()
            ));
        }
        *count.entry(f).or_default() += 1;
    }
    assert!(fails.is_empty(), "Abweichungen von Go:\n{}", fails.join("\n"));
    for f in ["pow", "exp", "log", "fmax", "fmin"] {
        let n = count.get(f).copied().unwrap_or(0);
        assert!(n >= 10_000, "{f}: nur {n} Vektoren");
    }
}

#[test]
fn nan_hat_go_bits() {
    assert_eq!(pow(f64::NAN, 2.0).to_bits(), 0x7FF8_0000_0000_0001);
    assert_eq!(fmax(f64::NAN, 1.0).to_bits(), 0x7FF8_0000_0000_0001);
    assert_eq!(fmax(f64::NAN, f64::INFINITY), f64::INFINITY);
    assert_eq!(fmin(0.0, -0.0).to_bits(), (-0.0_f64).to_bits());
    assert_eq!(fmax(-0.0, 0.0).to_bits(), 0.0_f64.to_bits());
}
