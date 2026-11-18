//! Tests aus `preview_chunkcount_test.go` (WZ17.3, B-485/AC-04): Die Vorschau nimmt die Chunk-Anzahl als Option, ohne
//! globalen Zustand (wie WZ8.4). `count_opts` nutzen auch die übrigen Vorschau-Tests.
use k3c_level::level::generate;
use k3c_level::types::PlannedChunk;
use k3c_sim::world::base::base;

use super::PreviewOptions;
use super::preview_candidate_tests::embedded_hub;
use super::preview_planned_tests::planned_opts;
use super::preview_tests::{biome, json_of, must_preview, preview, preview_opts};
use super::testutil::test_data;
use crate::fehler::Fehler;

pub(super) fn count_opts(n: i64, plan: Vec<PlannedChunk>) -> PreviewOptions {
    let mut o = planned_opts(plan);
    o.chunk_count = n;
    o
}

fn err_text(r: Result<k3c_sim::state::World, Fehler>) -> String {
    match r {
        Err(e) => e.to_string(),
        Ok(_) => String::new(),
    }
}

#[test]
fn vorschau_mit_chunk_anzahl_hat_genau_diese_chunks() {
    for size in &test_data().island_sizes.sizes {
        let w = must_preview(&count_opts(size.chunk_count, Vec::new()));
        assert_eq!(
            w.level.chunks.len() as i64,
            size.chunk_count,
            "{}: {} Chunks, erwartet {}",
            size.name,
            w.level.chunks.len(),
            size.chunk_count
        );
        let want = (size.chunk_count / 2) as f64 * w.level.chunk_width_units;
        assert_eq!(
            w.hub_x, want,
            "{}: Hub bei {}, erwartet mittig bei {want}",
            size.name, w.hub_x
        );
    }
}

#[test]
fn vorschau_mit_chunk_anzahl_ist_deterministisch() {
    let a = must_preview(&count_opts(24, Vec::new()));
    let b = must_preview(&count_opts(24, Vec::new()));
    assert_eq!(
        json_of(&a),
        json_of(&b),
        "gleiche Anzahl und gleicher Seed, verschiedene Welt"
    );
}

#[test]
fn vorschau_chunk_anzahl_null_ist_heutiges_level() {
    let rules = &base().expect("Basis").rules;
    let want = generate(&biome("forest"), "abc", rules).expect("Level");
    let w = must_preview(&count_opts(0, Vec::new()));
    assert!(
        w.level.entities == want.entities && w.level.chunks.len() == want.chunks.len(),
        "Anzahl 0: das Level weicht von level.Generate ab"
    );
    assert_eq!(
        json_of(&w),
        json_of(&must_preview(&preview_opts())),
        "Anzahl 0: andere Welt als ohne Option"
    );
}

#[test]
fn vorschau_lehnt_anzahl_ohne_inselgroesse_ab() {
    let valid: Vec<String> = test_data()
        .island_sizes
        .sizes
        .iter()
        .map(|s| s.chunk_count.to_string())
        .collect();
    for n in [-16, 2, 18, 22, 50] {
        let text = err_text(preview(&count_opts(n, Vec::new())));
        assert!(
            text.contains("keine Inselgröße") && text.contains(&valid.join(", ")),
            "Anzahl {n}: Fehler {text:?}, erwartet „keine Inselgröße“ mit den gültigen Werten"
        );
    }
}

// Kann das Biom die Größe mit dieser Planung nicht erzeugen (kein Platz für die Portale), meldet die Erzeugung den
// Fehler; die Vorschau gerät nicht in Panik.
#[test]
fn vorschau_meldet_fehler_der_erzeugung() {
    let mut plan = Vec::new();
    for slot in 3..=6 {
        for s in [slot, -slot] {
            plan.push(PlannedChunk {
                slot: s,
                kind: "clearing".into(),
                ..PlannedChunk::default()
            });
        }
    }
    let r = preview(&count_opts(16, plan));
    let ok = r.is_ok();
    let text = err_text(r);
    assert!(
        !ok && text.contains("vorschau: Level nicht erzeugbar"),
        "Fehler {text:?}; erwartet der Fehler der Erzeugung"
    );
}

#[test]
fn vorschau_aufeinanderfolgende_aufrufe_mit_verschiedener_anzahl() {
    let first = json_of(&must_preview(&count_opts(24, Vec::new())));
    let other = must_preview(&count_opts(40, Vec::new()));
    let plain = must_preview(&count_opts(0, Vec::new()));
    let again = json_of(&must_preview(&count_opts(24, Vec::new())));
    assert_eq!(
        first, again,
        "zweiter Aufruf mit Anzahl 24 liefert einen anderen Snapshot"
    );
    assert!(
        other.level.chunks.len() == 40 && first != json_of(&other) && first != json_of(&plain),
        "Anzahl 40 hat {} Chunks oder gleicht der Welt mit Anzahl 24 bzw. 0",
        other.level.chunks.len()
    );
}

// B-490/AC-03: Die Chunk-Anzahl gilt auch mit Kandidaten-hub.json; eine Zahl, die keine Inselgröße ist, wird mit
// derselben Meldung abgelehnt (`check_chunk_count`).
#[test]
fn vorschau_chunk_anzahl_mit_kandidat() {
    let mut o = count_opts(24, Vec::new());
    o.hub_candidate = embedded_hub();
    let w = must_preview(&o);
    assert_eq!(
        w.level.chunks.len(),
        24,
        "Anzahl 24 mit Kandidat: {} Chunks",
        w.level.chunks.len()
    );
    o.chunk_count = 18;
    let text = err_text(preview(&o));
    assert!(
        text.contains("keine Inselgröße") && text.contains("gültig: 16, 24"),
        "Fehler {text:?}, erwartet die Ablehnung einer Zahl, die keine Inselgröße ist"
    );
    o.chunk_count = 0;
    must_preview(&o);
}

// Die Planung wird gegen die eingestellte Anzahl geprüft, nicht gegen die kürzeste Länge des Bioms (Wald: ±7).
#[test]
fn vorschau_prueft_planung_gegen_anzahl() {
    let far = PlannedChunk {
        slot: 8,
        kind: "clearing".into(),
        seed: "lichtung".into(),
        ..PlannedChunk::default()
    };
    let text = err_text(preview(&planned_opts(vec![far.clone()])));
    assert!(
        text.contains("Platz +8"),
        "ohne Anzahl: Fehler {text:?}, erwartet Platz +8 jenseits der kürzesten Länge"
    );
    let w = must_preview(&count_opts(24, vec![far]));
    assert_eq!(
        w.level.chunks.len(),
        24,
        "Anzahl 24 mit Planung: {} Chunks",
        w.level.chunks.len()
    );
    let near = PlannedChunk {
        slot: -7,
        kind: "clearing".into(),
        ..PlannedChunk::default()
    };
    let text = err_text(preview(&count_opts(16, vec![near])));
    assert!(
        text.contains("Platz -7") && text.contains("Chunk-Anzahl 16"),
        "Anzahl 16: Fehler {text:?}, erwartet Platz -7 jenseits der Chunk-Anzahl 16"
    );
}
