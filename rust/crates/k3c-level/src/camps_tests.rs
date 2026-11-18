use crate::level::LAIR_CAMP_GAP_UNITS;
use crate::testutil::{biome, biome_ids, generate, rules};
use crate::types::Layout;

/// Jedes Camp liegt ≥ `pay_gap_units` von Mauer, Turm und Tor jeder Linie entfernt, im Level selbst und bei jeder
/// möglichen Streuung (B-261/AC-01, W0 › AC-08).
#[test]
fn camp_abstand_zu_linien_500_seeds() {
    for id in biome_ids() {
        let b = biome(&id);
        for seed in 0..500 {
            let l = generate(&b, &seed.to_string());
            for e in l.entities.iter().filter(|e| e.kind == "recruitCamp") {
                check_camp(&l, e.x, &id, seed);
            }
        }
    }
}

/// Go: `checkCamp`.
pub(crate) fn check_camp(l: &Layout, x: f64, id: &str, seed: i64) {
    for site in &l.sites {
        assert!(
            (x - site.x).abs() >= rules().pay_gap_units,
            "Biom {id}, Seed {seed}: Camp {x}, Platz {} bei {}",
            site.kind,
            site.x
        );
    }
}

/// B-645: In Biomen mit Bau liegt das Camp nie näher als `LAIR_CAMP_GAP_UNITS` am Bau und höchstens in 5 % der Seeds
/// hinter einem Portal (wo der Abstand zu den Linien keinen anderen Platz lässt, hat er Vorrang, B-261).
#[test]
fn camp_fern_vom_bau_500_seeds() {
    for id in biome_ids() {
        let b = biome(&id);
        if !b.lair {
            continue;
        }
        let mut behind = 0;
        for seed in 0..500 {
            let l = generate(&b, &seed.to_string());
            if camp_behind_portal(&l, &id, seed) {
                behind += 1;
            }
        }
        assert!(
            behind <= 25,
            "Biom {id}: Camp in {behind} von 500 Seeds hinter einem Portal, erlaubt 25"
        );
    }
}

/// Go: `campBehindPortal`.
pub(crate) fn camp_behind_portal(l: &Layout, id: &str, seed: i64) -> bool {
    let (mut camp, mut lair) = (0.0, 0.0);
    let mut portals = Vec::new();
    for e in &l.entities {
        match e.kind.as_str() {
            "recruitCamp" => camp = e.x,
            "lair" => lair = e.x,
            "portal" => portals.push(e.x),
            _ => {}
        }
    }
    let gap = LAIR_CAMP_GAP_UNITS as f64;
    assert!(
        (camp - lair).abs() >= gap,
        "Biom {id}, Seed {seed}: Camp {camp} nur {} vom Bau",
        (camp - lair).abs()
    );
    let hub = l.hub_center_units;
    portals
        .iter()
        .any(|p| (p - hub) * (camp - hub) > 0.0 && (p - hub).abs() < (camp - hub).abs())
}
