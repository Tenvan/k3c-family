//! Tests aus `preview_ruin_test.go` (WZ14.5, B-468/AC-05): Die Vorschau liefert die Ruinen eines Chunks
//! (`preview_chunk_ruins`) in der Form von `plannedChunks.buildings`; Grundlage ist `testdata/planned/forest-ruin.json`.
use k3c_level::types::PlannedChunk;
use serde::Deserialize;

use super::preview_tests::{must_preview, preview_opts};
use super::testutil::test_data;
use super::{PreviewOptions, PreviewSource, preview_chunk_ruins};

#[derive(Deserialize)]
struct PlannedFile {
    #[serde(rename = "planned", alias = "Planned")]
    planned: Vec<PlannedChunk>,
}

fn ruin_preview_opts() -> (PreviewOptions, Vec<PlannedChunk>) {
    let path = concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../../testdata/planned/forest-ruin.json"
    );
    let raw = std::fs::read_to_string(path).expect("forest-ruin.json");
    let f: PlannedFile = serde_json::from_str(&raw).expect("JSON");
    let o = PreviewOptions {
        seed: "ruine-1".into(),
        source: PreviewSource { biome: "forest".into() },
        planned: f.planned.clone(),
        ..preview_opts()
    };
    (o, f.planned)
}

#[test]
fn vorschau_liefert_ruinen_je_chunk() {
    let (mut o, plan) = ruin_preview_opts();
    let data = test_data();
    for c in &plan {
        match preview_chunk_ruins(&o, c.slot, &data) {
            Ok(got) => assert_eq!(
                got, c.buildings,
                "Platz {:+}: Ruinen {got:?}, geplant {:?}",
                c.slot, c.buildings
            ),
            Err(e) => panic!("{e}"),
        }
    }
    match preview_chunk_ruins(&o, 1, &data) {
        Ok(got) if got.is_empty() => {}
        other => panic!("Platz +1 ohne Ruine: {other:?}"),
    }
    o.planned = Vec::new();
    match preview_chunk_ruins(&o, 2, &data) {
        Ok(got) if got.is_empty() => {}
        other => panic!("ohne Planung: {other:?}"),
    }
    assert!(preview_chunk_ruins(&o, 0, &data).is_err(), "Platz 0 (Hub) angenommen");
}

// Die Vorschau-Welt trägt die Ruinen als Plätze mit Kennung (Zustand wie snap, Feld sites[].ruin).
#[test]
fn vorschau_welt_hat_ruinen_plaetze() {
    let (o, _) = ruin_preview_opts();
    let w = must_preview(&o);
    let n = w.sites.iter().filter(|s| s.ruin).count();
    assert!(
        n == w.level.ruins.len() && n == 3,
        "{n} Ruinen-Plätze, Level hat {}, erwartet 3",
        w.level.ruins.len()
    );
}
