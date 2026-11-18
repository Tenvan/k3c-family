//! Planung und Chunk-Anzahl im Spielstand (Go: `island_save_planned_test.go`, WZ24.2, B-489/AC-02): Ein Stand lädt seine
//! Stufen mit der Planung und Chunk-Anzahl, mit denen sie gebaut wurden, auch wenn die Daten der Welt sich danach ändern.
//! Beide Go-Tests bauen eine Insel (`create_island`, RU14) und tragen `ignore`; `own_*` prüft, was schon läuft: den Weg von
//! Planung und Anzahl durch den Stand und `saved_create_options`.

use std::collections::BTreeMap;

use k3c_level::types::PlannedChunk;
use serde_json::Value;

use super::save::IslandSave;
use super::save_load::saved_create_options;
use super::testutil::*;
use super::{CreateOptions, create_island};
use crate::state::save::HubSave;

fn forest_plan() -> Vec<PlannedChunk> {
    vec![PlannedChunk {
        slot: 9,
        kind: "forest".into(),
        ..PlannedChunk::default()
    }]
}

/// Go: `TestStandLaedtMitGespeicherterPlanungUndAnzahl`.
#[test]
#[ignore = "wartet auf RU19 (decode_world_story)"]
fn stand_laedt_mit_gespeicherter_planung_und_anzahl() {
    let data = test_data();
    let world = standard_world(&data);
    let opts = CreateOptions {
        planned: Some(BTreeMap::from([(0, forest_plan())])),
        chunk_counts: Some(BTreeMap::from([(0, 24)])),
        ..CreateOptions::default()
    };
    let isl = create_island(&data, &world, "stand", &[0, 1], 1.0, opts).expect("Insel");
    let loaded = load_island(&data, &save_json(&isl, "2026-10-22T00:00:00Z"));
    for (i, (a, b)) in isl.stages.iter().zip(&loaded.stages).enumerate() {
        assert_eq!(
            a.width_units, b.width_units,
            "Stufe {i}: Breite nach dem Laden, gebaut mit {}",
            a.width_units
        );
    }
    assert_eq!(loaded.stages[0].chunk_count, 24, "Chunk-Anzahl nach dem Laden");
    assert!(
        loaded.stages[0].planned.len() == 1 && loaded.stages[0].planned[0].slot == 9,
        "Planung nach dem Laden {:?}",
        loaded.stages[0].planned
    );
    assert!(
        loaded.stages[1].planned.is_empty() && loaded.stages[1].chunk_count == 0,
        "Stufe 1 hatte keine Planung"
    );
}

/// Go: `TestStandOhneFelderLaedtReinenZufall`: Ein Stand ohne die Felder lädt in der Länge des Bioms ohne Planung.
#[test]
#[ignore = "wartet auf RU19 (decode_world_story)"]
fn stand_ohne_felder_laedt_reinen_zufall() {
    let data = test_data();
    let world = standard_world(&data);
    let isl = create_island(&data, &world, "alt", &[1], 1.0, CreateOptions::default()).expect("Insel");
    let generic: Value = serde_json::from_str(&save_json(&isl, "2026-10-22T00:00:00Z")).expect("JSON");
    for st in generic["stages"].as_array().expect("Stufen") {
        assert!(
            st.get("planned").is_none(),
            "planned ohne Planung geschrieben (omitempty)"
        );
    }
    let loaded = load_island(&data, &generic.to_string());
    assert_eq!(loaded.stages[0].width_units, isl.stages[0].width_units);
}

#[test]
fn own_planung_und_anzahl_gehen_durch_den_stand_und_die_optionen() {
    let hub = |depth: i64, planned: Vec<PlannedChunk>, chunk_count: i64| HubSave {
        depth,
        planned,
        chunk_count,
        ..HubSave::default()
    };
    let s = IslandSave {
        stages: vec![hub(0, forest_plan(), 24), hub(1, vec![], 0)],
        ..IslandSave::default()
    };
    // Geschrieben: `planned` und `chunkCount` nur bei Stufen mit Wert (omitempty).
    let v: Value = serde_json::to_value(&s).expect("schreiben");
    assert!(v["stages"][0].get("planned").is_some() && v["stages"][0]["chunkCount"] == 24);
    assert!(v["stages"][1].get("planned").is_none() && v["stages"][1].get("chunkCount").is_none());
    // Zurück: die Erzeugungs-Optionen tragen nur Stufen mit Wert (Go: leere Maps, nicht `nil`).
    let back: IslandSave = serde_json::from_value(v).expect("lesen");
    let o = saved_create_options(&back);
    assert_eq!(
        o.planned.as_ref().map(|m| m.keys().copied().collect::<Vec<_>>()),
        Some(vec![0])
    );
    assert_eq!(o.chunk_counts, Some(BTreeMap::from([(0, 24)])));
    assert_eq!(
        o.nature,
        Some(BTreeMap::new()),
        "ohne Natur-Werte: leere Map, keine Vorgabe der Welt"
    );
    let none = saved_create_options(&IslandSave::default());
    assert!(
        none.planned.as_ref().is_some_and(BTreeMap::is_empty)
            && none.chunk_counts.as_ref().is_some_and(BTreeMap::is_empty)
    );
}
