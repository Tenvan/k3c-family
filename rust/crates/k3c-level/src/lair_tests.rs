use super::check_lair_fit;
use crate::biome::Range;
use crate::camps_tests::{camp_behind_portal, check_camp};
use crate::planned_gen::Options;
use crate::testutil::{biome, biome_ids, count_entities, generate, generate_with, validate_with};

// Bau in der tiefsten Stufe jedes Bioms (B-981/AC-02): Options.Lair legt ihn fest, das Biom-Feld ist nur der Standard.

fn lair(on: bool) -> Options {
    Options {
        lair: Some(on),
        ..Options::default()
    }
}

/// Der Bau entsteht mit Options.Lair in jedem Biom genau einmal, ohne nie; Level und Prüfung sind sauber, der Abstand
/// des Camps zum Bau (und zu den Linien) gilt über 500 Seeds in jedem Biom.
#[test]
fn bau_in_jedem_biom_500_seeds() {
    for id in biome_ids() {
        let b = biome(&id);
        let mut behind = 0;
        for seed in 0..500_i64 {
            let s = seed.to_string();
            let l = generate_with(&b, &s, &lair(true)).unwrap_or_else(|e| panic!("{e}"));
            let n = count_entities(&l, "lair");
            assert_eq!(n, 1, "Biom {id}, Seed {s}: {n} Bau");
            let errs = validate_with(&l, &b, &lair(true));
            assert!(errs.is_empty(), "Biom {id}, Seed {s}: {errs:?}");
            if camp_behind_portal(&l, &id, seed) {
                behind += 1;
            }
            for e in l.entities.iter().filter(|e| e.kind == "recruitCamp") {
                check_camp(&l, e.x, &id, seed);
            }
            let off = generate_with(&b, &s, &lair(false));
            let ok = off
                .as_ref()
                .is_ok_and(|o| count_entities(o, "lair") == 0 && validate_with(o, &b, &lair(false)).is_empty());
            assert!(ok, "Biom {id}, Seed {s}: ohne Bau {off:?}");
        }
        // Der Abstand zu den Linien hat Vorrang (B-261): In kurzen Biomen bleibt außerhalb der Linien oft nur ein Platz
        // hinter einem Portal. Die 5 %-Grenze gilt nur für die Kristallhöhle (camps_test.go); sonst ist die Quote nur
        // Information.
        println!("Biom {id}: Camp hinter einem Portal in {behind} von 500 Seeds");
        assert!(
            id != "crystal" || behind <= 25,
            "Biom {id}: Camp in {behind} von 500 Seeds hinter einem Portal, erlaubt 25"
        );
    }
}

/// Ohne Options.Lair gilt das Feld des Bioms, das Level bleibt bytegleich zu Generate (Golden).
#[test]
fn bau_standard_ist_biom_feld() {
    for id in biome_ids() {
        let b = biome(&id);
        let l = generate(&b, "x");
        let has = count_entities(&l, "lair") == 1;
        assert_eq!(has, b.lair, "Biom {id}: Bau {has}, Feld lair {}", b.lair);
    }
}

/// CheckLairFit: jedes Biom trägt den Bau in seiner Länge und in jeder Inselgröße ab 16 Chunks; zu kleine Stufen und zu
/// viele Camps werden mit Grund abgelehnt.
#[test]
fn check_lair_fit_traegt_und_lehnt_ab() {
    for id in biome_ids() {
        let mut b = biome(&id);
        for n in [0, 16, 24, 160] {
            if let Err(e) = check_lair_fit(&b, n) {
                panic!("Biom {id}, {n} Chunks: {e}");
            }
        }
        let small = check_lair_fit(&b, 2);
        assert!(
            small.as_ref().is_err_and(|e| e.contains("zu wenig")),
            "Biom {id}, 2 Chunks: {small:?}"
        );
        b.event_chunks.clear();
        b.event_chunks.insert("recruitCamp".into(), Range { min: 99, max: 99 });
        let camps = check_lair_fit(&b, 16);
        assert!(
            camps.as_ref().is_err_and(|e| e.contains("Camp(s) brauchen Platz")),
            "Biom {id}, 99 Camps: {camps:?}"
        );
    }
}
