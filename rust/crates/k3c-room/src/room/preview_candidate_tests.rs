//! Tests aus `preview_candidate_test.go` (WZ9.1, B-462/AC-04, AC-08, AC-09): Vorschau aus einer Kandidaten-hub.json. Alle
//! Kandidaten sind Kopien im Speicher. Abweichung: Gos `sim.EconomyOf` rechnet hier `economy_of` mit dem Wertesatz der Welt.
use std::collections::BTreeMap;

use k3c_data::source::EMBEDDED;
use k3c_level::types::{PlannedChunk, PlannedObject};
use k3c_sim::economy::view::economy_of;
use k3c_sim::world::vals;

use super::PreviewOptions;
use super::preview_tests::{built_of, hub_levels, json_of, must_preview, preview, preview_opts};
use crate::fehler::Fehler;

/// Eine Kopie der eingebetteten data/hub.json.
pub(super) fn embedded_hub() -> String {
    EMBEDDED
        .iter()
        .find(|(name, _)| *name == "hub.json")
        .map(|(_, raw)| (*raw).to_string())
        .expect("hub.json")
}

/// Die eingebettete hub.json, Rüstkammer ab Hub-Stufe 3 statt 4.
fn armory_from3() -> String {
    let (from, to) = (
        r#"{ "kind": "armory", "offsetUnits": 40, "hubLevel": 4 }"#,
        r#"{ "kind": "armory", "offsetUnits": 40, "hubLevel": 3 }"#,
    );
    let raw = embedded_hub();
    assert_eq!(
        raw.matches(from).count(),
        1,
        "data/hub.json enthält {from} nicht genau einmal; Test an die Datei anpassen"
    );
    raw.replacen(from, to, 1)
}

#[test]
fn build_preview_kandidat_gleich_eingebettet() {
    for biome in ["forest", "cave", "crystal"] {
        for level in 0..=hub_levels() {
            let mut o = preview_opts();
            o.source.biome = biome.into();
            o.hub_level = level;
            let pw = must_preview(&o);
            o.hub_candidate = embedded_hub();
            let cw = must_preview(&o);
            assert_eq!(
                json_of(&cw),
                json_of(&pw),
                "{biome}, Hub {level}: Kandidat = eingebettete hub.json ergibt eine andere Vorschau"
            );
            // `economy_of` hängt an der Welt (Wertesatz, Hub-Stufe): muss ebenfalls gleich sein
            let (ec, ep) = (economy_of(&cw, &vals(&cw), None), economy_of(&pw, &vals(&pw), None));
            assert_eq!(
                json_of(&ec),
                json_of(&ep),
                "{biome}, Hub {level}: Wirtschaft mit Kandidat {ec:?}, ohne {ep:?}"
            );
        }
    }
}

#[test]
fn build_preview_ruestkammer_ab_stufe3_nur_mit_kandidat() {
    let mut o = preview_opts();
    o.hub_level = 3;
    let n = built_of(&must_preview(&o), "armory");
    assert_eq!(
        n, 0,
        "ohne Kandidat: Rüstkammer in Hub-Stufe 3 gebaut ({n}), erwartet erst ab 4"
    );
    o.hub_candidate = armory_from3();
    let n = built_of(&must_preview(&o), "armory");
    assert_eq!(
        n, 1,
        "mit Kandidat: Rüstkammer in Hub-Stufe 3 {n}-mal gebaut, erwartet 1"
    );
    o.hub_level = 2;
    let n = built_of(&must_preview(&o), "armory");
    assert_eq!(n, 0, "mit Kandidat: Rüstkammer schon in Hub-Stufe 2 gebaut ({n})");
}

#[test]
fn build_preview_lehnt_ungueltigen_kandidaten_ab() {
    let no_workshop = embedded_hub().replacen(r#""kind": "workshop""#, r#""kind": "nirgendwo""#, 1);
    let too_high = armory_from3().replacen(r#""hubLevel": 3 }"#, r#""hubLevel": 99 }"#, 1);
    let cases: BTreeMap<&str, String> = BTreeMap::from([
        ("kein JSON", r#"{"levels": ["#.to_string()),
        ("leere Stufen", r#"{"levels": []}"#.to_string()),
        ("unbekannte Art", no_workshop),
        ("Stufe zu hoch", too_high),
    ]);
    for (name, raw) in cases {
        let mut o = preview_opts();
        o.hub_candidate = raw;
        match preview(&o) {
            Err(Fehler::Candidate(text))
                if text.starts_with("vorschau: Kandidaten-hub.json mit ")
                    && !text.starts_with("vorschau: Kandidaten-hub.json mit 0 Befunden") => {}
            other => panic!(
                "{name}: Welt {}, Fehler {other:?}; erwartet Candidate mit Befunden",
                other.is_ok(),
                other = other.as_ref().err()
            ),
        }
    }
}

#[test]
fn build_preview_kandidat_deterministisch_ohne_globalen_zustand() {
    let mut plain = preview_opts();
    plain.hub_level = 3;
    let before = json_of(&must_preview(&plain));
    let mut cand = plain.clone();
    cand.hub_candidate = armory_from3();
    cand.phase = "dusk".into();
    cand.progress = 0.25;
    let first = json_of(&must_preview(&cand));
    assert_eq!(
        first,
        json_of(&must_preview(&cand)),
        "zweimal derselbe Aufruf mit Kandidat liefert verschiedene Welten"
    );
    assert_eq!(
        before,
        json_of(&must_preview(&plain)),
        "ein Aufruf mit Kandidat ändert die Vorschau ohne Kandidat (globaler Zustand?)"
    );
    let embedded = PreviewOptions {
        seed: cand.seed.clone(),
        source: cand.source.clone(),
        hub_level: 3,
        phase: "dusk".into(),
        progress: 0.25,
        ..PreviewOptions::default()
    };
    assert_ne!(
        first,
        json_of(&must_preview(&embedded)),
        "Kandidat mit Rüstkammer ab 3 und eingebettete hub.json liefern dieselbe Welt"
    );
}

#[test]
fn build_preview_sieben_stufen_kandidat() {
    let path = concat!(env!("CARGO_MANIFEST_DIR"), "/../../../testdata/hub/hub7.json");
    let raw = std::fs::read_to_string(path).expect("hub7.json");
    let mut o = preview_opts();
    o.hub_candidate = raw;
    let w = must_preview(&o);
    assert_eq!(
        w.hub_level, 7,
        "Standard-Stufe mit 7-Stufen-Kandidat: {}, erwartet 7",
        w.hub_level
    );
    for level in 1..=7 {
        o.hub_level = level;
        let w = must_preview(&o);
        let smithy = usize::from(level >= 6); // Schmiede steht in hub7.json ab Hub-Stufe 6
        assert!(
            w.hub_level == level && built_of(&w, "smithy") == smithy,
            "Hub {level}: HubLevel {}, Schmiede {} (erwartet {smithy})",
            w.hub_level,
            built_of(&w, "smithy")
        );
    }
    o.hub_level = 8;
    assert!(preview(&o).is_err(), "Hub-Stufe 8 mit 7-Stufen-Kandidat ohne Fehler");
    o.hub_candidate = String::new();
    o.hub_level = hub_levels() + 1;
    assert!(
        preview(&o).is_err(),
        "Hub-Stufe {} ohne Kandidat ohne Fehler (über der eingebetteten hub.json)",
        o.hub_level
    );
}

// WZ24.3 (B-479, B-462/AC-09): Sind geplante Chunks und Kandidat zugleich gesetzt, wird die Planung mitgeprüft; mit den
// eingebetteten Daten ist sie gültig.
#[test]
fn vorschau_planung_gegen_kandidaten() {
    let plan = vec![PlannedChunk {
        slot: 2,
        kind: "cavern".into(),
        objects: vec![PlannedObject {
            kind: "stoneVein".into(),
            at: 36.0,
        }],
        ..PlannedChunk::default()
    }];
    let mut o = preview_opts();
    o.source.biome = "cave".into();
    o.planned = plan;
    o.hub_candidate = embedded_hub();
    must_preview(&o);
}
