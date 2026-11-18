//! Tests aus `preview_planned_test.go` (WZ8.4, B-461/AC-08): Die Vorschau nimmt die Planung als Option „geplante
//! Chunks“, ohne globalen Zustand. Die Hilfen (`planned_opts`, `plan_a`, `plan_b`) nutzen auch die übrigen Vorschau-Tests.
use k3c_level::level::generate;
use k3c_level::types::{PlannedChunk, PlannedObject};
use k3c_sim::world::base::base;

use super::PreviewOptions;
use super::preview_tests::{biome, json_of, must_preview, preview, preview_opts};

pub(super) fn planned_opts(plan: Vec<PlannedChunk>) -> PreviewOptions {
    let mut o = preview_opts();
    o.planned = plan;
    o
}

pub(super) fn plan_a() -> Vec<PlannedChunk> {
    vec![PlannedChunk {
        slot: 6,
        kind: "portal".into(),
        ..PlannedChunk::default()
    }]
}

pub(super) fn plan_b() -> Vec<PlannedChunk> {
    vec![
        PlannedChunk {
            slot: -3,
            kind: "clearing".into(),
            seed: "lichtung".into(),
            ..PlannedChunk::default()
        },
        PlannedChunk {
            slot: 2,
            kind: "rocks".into(),
            objects: vec![PlannedObject {
                kind: "rock".into(),
                at: 10.0,
            }],
            ..PlannedChunk::default()
        },
    ]
}

/// Die Portale der Vorschau, sortiert und relativ zum Hub.
fn portals_of(o: &PreviewOptions) -> Vec<f64> {
    let w = must_preview(o);
    let mut out = w.portals.clone();
    out.sort_by(f64::total_cmp);
    out.iter().map(|p| p - w.hub_x).collect()
}

#[test]
fn vorschau_mit_planung_ist_deterministisch() {
    let a = must_preview(&planned_opts(plan_b()));
    let b = must_preview(&planned_opts(plan_b()));
    assert_eq!(json_of(&a), json_of(&b), "gleiche Planung, verschiedene Welt");
}

#[test]
fn vorschau_ohne_planung_bleibt_zufall() {
    let rules = &base().expect("Basis").rules;
    let want = generate(&biome("forest"), "abc", rules).expect("Level");
    // Go prüft auch `nil`; Rust kennt nur die leere Liste
    let got = must_preview(&planned_opts(Vec::new())).level;
    assert_eq!(got.entities, want.entities, "das Level weicht von level.Generate ab");
}

#[test]
fn vorschau_lehnt_ungueltige_planung_ab() {
    let plan = vec![PlannedChunk {
        slot: 0,
        kind: "forest".into(),
        ..PlannedChunk::default()
    }];
    match preview(&planned_opts(plan)) {
        Err(e) if e.to_string().contains("Platz 0") => {}
        other => panic!(
            "Fehler {other:?}, erwartet mit dem Grund der Prüfung (Platz 0)",
            other = other.map(|_| ())
        ),
    }
}

#[test]
fn vorschau_aufeinanderfolgende_aufrufe_mit_verschiedener_planung() {
    let first = portals_of(&planned_opts(plan_a()));
    let other = portals_of(&planned_opts(plan_b()));
    let again = portals_of(&planned_opts(plan_a()));
    assert!(
        first == again && first.contains(&325.0) && !other.contains(&325.0),
        "Portale: erst {first:?}, andere Planung {other:?}, dann wieder {again:?}"
    );
    let snap_b1 = json_of(&must_preview(&planned_opts(plan_b())));
    let _ = must_preview(&planned_opts(plan_a())); // dazwischen eine andere Planung
    assert_eq!(
        snap_b1,
        json_of(&must_preview(&planned_opts(plan_b()))),
        "zweiter Aufruf mit Planung B liefert einen anderen Snapshot"
    );
}
