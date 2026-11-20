//! LA1.1 (B-1163): data/sizes.json, Größenstufen in Monarchenhöhen.

use super::*;
use crate::testutil::raw;

fn file() -> Sizes {
    load(&raw(FILE)).expect("sizes.json")
}

/// Richtwerte des Standards `docs/standards/bildaufbau.md` § 2 je Stufe (untere, obere Grenze in M).
const STANDARD: [(&str, f64, f64); 16] = [
    ("gegner-klein", 0.3, 0.5),
    ("buerger", 0.55, 0.65),
    ("truppe-elite", 0.55, 0.7),
    ("gegner-riese", 1.1, 1.4),
    ("busch", 0.4, 0.8),
    ("gebaeude-mittel", 1.0, 1.4),
    ("mauer", 0.8, 1.2),
    ("turm", 2.0, 2.5),
    ("burg", 3.5, 4.5),
    ("baum-niedrig", 2.5, 3.5),
    ("baum-mittel", 2.5, 3.5),
    ("baum-hoch", 2.5, 3.5),
    ("nadel-niedrig", 4.0, 6.0),
    ("nadel-mittel", 4.0, 6.0),
    ("nadel-hoch", 4.0, 6.0),
    ("boss", 1.0, 1.5),
];

#[test]
fn bezug_ist_der_berittene_monarch() {
    assert_eq!(file().reference_px, 121.5, "1 M = 121,5 Spiel-px (look::MONARCH_UNITS)");
    assert_eq!(file().reference_px / 32.0, crate::look::MONARCH_UNITS);
}

#[test]
fn stufen_liegen_in_den_richtwerten_des_standards() {
    let s = file();
    for (name, lo, hi) in STANDARD {
        let m = s.stages.get(name).unwrap_or_else(|| panic!("Stufe {name} fehlt")).m;
        assert!((lo..=hi).contains(&m), "{name}: {m} M liegt außerhalb {lo}–{hi} M");
    }
}

#[test]
fn verhaeltnisse_zum_monarchen() {
    let s = file();
    let px = |a: &str| s.target_px(a).unwrap_or_else(|| panic!("{a} ohne Stufe"));
    let m = s.reference_px;
    assert!(
        px("truppen/bauer") < 0.7 * m && px("truppen/bauer") > 0.5 * m,
        "Bürger etwa 0,6 M"
    );
    assert!(
        px("truppen/krieger") == px("truppen/bauer"),
        "Truppen gleicher Stufe gleich groß"
    );
    assert!(px("gegner/gier") < px("truppen/bauer"), "Gier kleiner als ein Bürger");
    assert!(px("gegner/hoehlentroll") > m, "Brecher größer als der Monarch");
    assert!(
        px("bosse/troll-koenig") > px("gegner/hoehlentroll"),
        "Boss größer als Gegner"
    );
    assert!(
        px("gebaeude-paper/burg") > px("gebaeude-paper/turm"),
        "Burg größer als Turm"
    );
    assert!(px("gebaeude-paper/turm") > 2.0 * m, "Turm mehr als 2 M");
    assert!(
        px("objekte/tanne-hoch") > px("objekte/birke-hoch"),
        "Nadelbaum höher als Laubbaum"
    );
}

#[test]
fn rechenwege() {
    let s = file();
    let (t, sicht) = (s.target_px("truppen/bauer").unwrap(), s.assets["truppen/bauer"].visible);
    assert!((s.frame_px("truppen/bauer").unwrap() - t / sicht).abs() < 1e-9);
    // Sheet 100 px hoch bei 2 Spiel-px je Bild-px: Faktor bringt die sichtbare Höhe auf das Soll.
    let f = s.sheet_factor("gebaeude-paper/burg", 100, 2.0).unwrap();
    let vis = s.assets["gebaeude-paper/burg"].visible * 100.0 * 2.0 * f;
    assert!((vis - s.target_px("gebaeude-paper/burg").unwrap()).abs() < 1e-9);
    assert!(s.target_px("gibt/es-nicht").is_none() && s.sheet_factor("gibt/es-nicht", 1, 1.0).is_none());
}

#[test]
fn pruefung_lehnt_fehler_ab() {
    let bad = |j: &str| load(j).is_err();
    assert!(bad(r#"{"bezugPx":0}"#));
    assert!(bad(r#"{"bezugPx":1,"stufen":{"a":{"m":0}}}"#));
    assert!(bad(
        r#"{"bezugPx":1,"stufen":{"a":{"m":1}},"assets":{"x/y":{"stufe":"b","sicht":0.5}}}"#
    ));
    assert!(bad(
        r#"{"bezugPx":1,"stufen":{"a":{"m":1}},"assets":{"x/y":{"stufe":"a","sicht":1.5}}}"#
    ));
    assert!(load(r#"{"bezugPx":1,"stufen":{"a":{"m":1}},"assets":{"x/y":{"stufe":"a","sicht":0.5}}}"#).is_ok());
}

#[test]
fn jede_stufe_wird_benutzt() {
    let s = file();
    for name in s.stages.keys() {
        assert!(s.assets.values().any(|a| &a.stage == name), "Stufe {name} ohne Asset");
    }
}
