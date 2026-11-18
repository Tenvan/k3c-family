//! Tests aus `preview_chunk_test.go` (WZ17.3, B-485/AC-06 Teil Server): Seed-Inhalt eines Chunks aus dem Vorschau-Level,
//! für die Umwandlung eines Seed-Chunks in feste Objekte (geplante-chunks.md § 7).
use k3c_level::types::{Entity, PlannedChunk, PlannedObject};
use k3c_sim::state::World;

use super::preview_chunkcount_tests::count_opts;
use super::preview_planned_tests::{plan_b, planned_opts};
use super::preview_tests::{biome, json_of, must_preview, preview, preview_opts};
use super::testutil::test_data;
use super::{PreviewOptions, PreviewSource, preview_chunk_objects};
use crate::fehler::Fehler;

fn objects(o: &PreviewOptions, slot: i64) -> Result<Vec<PlannedObject>, Fehler> {
    preview_chunk_objects(o, slot, &test_data())
}

fn must_chunk_objects(o: &PreviewOptions, slot: i64) -> Vec<PlannedObject> {
    match objects(o, slot) {
        Ok(v) => v,
        Err(e) => panic!("{e}"),
    }
}

/// Der Anfang des Platzes `slot` im Level der Welt, gerechnet wie in geplante-chunks.md § 1 (Hub ± 50, alle Biome haben
/// 100 Units Hub-Breite) und damit unabhängig vom Code, dazu die Objekte darin.
fn chunk_start(o: &PreviewOptions, slot: i64) -> (f64, Vec<Entity>) {
    let w: World = must_preview(o);
    let lv = &w.level;
    let start = if slot > 0 {
        lv.hub_center_units + 50.0 + (slot - 1) as f64 * lv.chunk_width_units
    } else {
        lv.hub_center_units - 50.0 + slot as f64 * lv.chunk_width_units
    };
    let inside = lv
        .entities
        .iter()
        .filter(|e| e.x >= start && e.x < start + lv.chunk_width_units)
        .cloned()
        .collect();
    (start, inside)
}

// Der gelieferte Inhalt sind genau die Objekte des Vorschau-Levels in diesem Chunk, `at` ab Chunk-Anfang.
#[test]
fn chunk_inhalt_entspricht_den_objekten_im_level() {
    let lichtung = PlannedChunk {
        slot: 8,
        kind: "clearing".into(),
        seed: "lichtung".into(),
        ..PlannedChunk::default()
    };
    let cases: Vec<(&str, PreviewOptions, i64)> = vec![
        ("Seed-Chunk", planned_opts(plan_b()), -3),
        ("feste Objekte", planned_opts(plan_b()), 2),
        ("ungeplant", planned_opts(plan_b()), 5),
        ("Seed-Chunk, Anzahl 24", count_opts(24, vec![lichtung]), 8),
        (
            "Kristallhöhle links",
            PreviewOptions {
                seed: "abc".into(),
                source: PreviewSource {
                    biome: "crystal".into(),
                },
                ..PreviewOptions::default()
            },
            -2,
        ),
    ];
    for (name, o, slot) in cases {
        let (start, want) = chunk_start(&o, slot);
        let got = must_chunk_objects(&o, slot);
        if want.is_empty() || got.len() != want.len() {
            panic!("{name}: {} Objekte, im Level {}", got.len(), want.len());
        }
        for (i, obj) in got.iter().enumerate() {
            assert!(
                obj.kind == want[i].kind && start + obj.at == want[i].x,
                "{name}: Objekt {i} ist {}@{} ab {start}, im Level {}@{}",
                obj.kind,
                obj.at,
                want[i].kind,
                want[i].x
            );
        }
    }
    let got = must_chunk_objects(&planned_opts(plan_b()), 2);
    assert_eq!(
        got,
        plan_b()[1].objects,
        "Chunk mit Objektliste liefert {got:?}, geplant {:?}",
        plan_b()[1].objects
    );
}

// Mit Seed liefert ein geplanter Chunk ohne Objektliste den Inhalt seines Seeds: gleich für jeden Level-Seed.
#[test]
fn chunk_inhalt_eines_seed_chunks_haengt_nur_am_seed() {
    let a = must_chunk_objects(&planned_opts(plan_b()), -3);
    let mut o = planned_opts(plan_b());
    o.seed = "anderer".into();
    let b = must_chunk_objects(&o, -3);
    assert_eq!(
        a, b,
        "Seed-Chunk „lichtung“: {a:?} bei Seed abc, {b:?} bei Seed anderer"
    );
}

// Umwandlung (AC-06): Der Seed-Chunk mit seinem Inhalt als feste Objekte und ohne Seed ergibt dasselbe Level.
#[test]
fn umgewandelter_seed_chunk_gibt_dasselbe_level() {
    for n in [0, 24] {
        let before = count_opts(n, plan_b());
        let mut converted = plan_b();
        converted[0] = PlannedChunk {
            slot: -3,
            kind: "clearing".into(),
            objects: must_chunk_objects(&before, -3),
            ..PlannedChunk::default()
        };
        let after = count_opts(n, converted);
        assert_eq!(
            json_of(&must_preview(&before).level),
            json_of(&must_preview(&after).level),
            "Anzahl {n}: nach der Umwandlung ein anderes Level"
        );
    }
}

#[test]
fn chunk_inhalt_lehnt_ungueltigen_platz_ab() {
    let lv = must_preview(&preview_opts()).level;
    let hub_chunks = lv.chunks.iter().filter(|c| c.kind == "hub").count() as i64;
    let outer = (lv.chunks.len() as i64 - hub_chunks) / 2 - 1; // Hub-Chunks, Ausgang und Rand abgezogen
    let cases = [
        (0, "Platz 0"),
        (outer + 1, "Ausgang oder Rand"),
        (-outer - 1, "Ausgang oder Rand"),
        (100, "außerhalb"),
        (-100, "außerhalb"),
    ];
    for (slot, want) in cases {
        match objects(&preview_opts(), slot) {
            Err(e) if e.to_string().contains(want) => {}
            other => panic!("Platz {slot:+}: Fehler {other:?}, erwartet „{want}“"),
        }
    }
    assert!(objects(&preview_opts(), outer).is_ok(), "äußerster Platz {outer:+}");
    match objects(&count_opts(18, Vec::new()), 1) {
        Err(e) if e.to_string().contains("keine Inselgröße") => {}
        other => panic!("ungültige Optionen: Fehler {other:?}, erwartet den Fehler der Vorschau"),
    }
}

// WZ24.3 (B-497/AC-05): Die Umwandlung eines Seed-Chunks in feste Objekte ergibt in jedem Biom (Tiefe 0 bis 4), mit jeder
// Art des Bioms und auf mehreren Plätzen beider Seiten dasselbe Level, auch mit Adern und Biom-Summen.
#[test]
fn umwandlung_in_allen_biomen_und_stufen() {
    for id in ["forest", "cave", "mine", "ironhold", "crystal"] {
        let b = biome(id);
        for entry in &b.chunk_weights.0 {
            let kind = entry.key.as_str();
            if kind == "hub" || kind == "exit" || kind == "edge" {
                continue;
            }
            for slot in [-3, -1, 1, 3] {
                let base = PreviewOptions {
                    seed: "umwandlung".into(),
                    source: PreviewSource { biome: id.into() },
                    ..PreviewOptions::default()
                };
                let mut before = base.clone();
                before.planned = vec![PlannedChunk {
                    slot,
                    kind: kind.into(),
                    seed: "inhalt".into(),
                    ..PlannedChunk::default()
                }];
                let objs = match objects(&before, slot) {
                    Ok(o) => o,
                    Err(e) => panic!("{id}/{kind}/{slot:+}: Inhalt: {e}"),
                };
                let mut after = base;
                after.planned = vec![PlannedChunk {
                    slot,
                    kind: kind.into(),
                    objects: objs,
                    ..PlannedChunk::default()
                }];
                let (w1, w2) = (preview(&before), preview(&after));
                let (Ok(w1), Ok(w2)) = (w1, w2) else {
                    panic!("{id}/{kind}/{slot:+}: Vorschau schlug fehl");
                };
                assert_eq!(
                    json_of(&w1.level),
                    json_of(&w2.level),
                    "{id}/{kind}/{slot:+}: nach der Umwandlung ein anderes Level"
                );
            }
        }
    }
}
