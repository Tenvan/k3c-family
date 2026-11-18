//! Tests zu `hub_candidate_test.go` (WZ15.3, B-462): Bau einer Insel aus einer Kandidaten-hub.json über `CreateOptions`,
//! ohne globalen Zustand.

use std::sync::Arc;

use super::level::site_hub_level;
use super::testutil::{hub, hub_copy, make_island, read_hub7, site_of_kind, without_kind};
use super::validate::{RULE_REQUIRED, validate_planned};
use crate::island::CreateOptions;
use crate::island::data::biome_for_depth;
use crate::island::testutil::{parse_test_world, test_data};
use crate::state::World;
use crate::world::base::base;
use crate::world::preview::preview_hub_level;
use crate::world::{Options, create_world, create_world_with, hub_of};

fn candidate(h: k3c_data::hub::HubData) -> CreateOptions {
    CreateOptions {
        candidate: Some(h),
        ..CreateOptions::default()
    }
}

/// Go: `siteSpecsOf`: Art und Position aller Plätze der Stufe (Vergleich zweier Bauten).
fn site_specs_of(w: &World) -> Vec<(String, f64)> {
    w.sites.iter().map(|s| (s.kind.clone(), s.x)).collect()
}

#[test]
fn create_island_aus_sieben_stufen_kandidat() {
    let h = read_hub7();
    let isl = make_island("kandidat", &[0, 1], candidate(h.clone())).unwrap_or_else(|e| panic!("{e}"));
    let w = &isl.stages[0];
    assert!(!w.level.sites.is_empty(), "keine Mauer- und Turm-Plätze im Level");
    let smithy = site_of_kind(w, "smithy");
    assert!(
        smithy.is_some_and(|s| site_hub_level(w, s) == 6),
        "Schmiede aus dem Kandidaten mit Hub-Stufe 6 erwartet: {smithy:?}"
    );
    assert!(
        site_of_kind(w, "storage").is_some() && site_of_kind(&isl.stages[1], "stairsUp").is_some(),
        "Insel-Platz oder Treppe hoch fehlt"
    );
    assert_eq!(isl.stock, h.island_start_stock, "Startvorrat");
    let smithy = site_of_kind(w, "smithy").expect("smithy");
    let bare = World {
        hub_x: w.hub_x,
        ..World::default()
    };
    assert_ne!(
        site_hub_level(&bare, smithy),
        6,
        "eine Welt ohne Kandidat liest den Kandidaten (globaler Zustand?)"
    );
}

/// Der Vorschau-Teil von `TestCreateIslandAusSiebenStufenKandidat` (Go): `preview_hub_level` baut die Plätze über
/// `dev_build_site`.
#[test]
fn create_island_kandidat_vorschau() {
    let h = read_hub7();
    let mut isl = make_island("kandidat", &[0, 1], candidate(h.clone())).unwrap_or_else(|e| panic!("{e}"));
    let top = h.levels.len() as i64;
    if let Err(e) = preview_hub_level(&mut isl.stages[0], top) {
        panic!("Vorschau auf Stufe {top}: {e}");
    }
    assert!(
        preview_hub_level(&mut isl.stages[1], top + 1).is_err(),
        "Vorschau über der höchsten Stufe des Kandidaten ohne Fehler"
    );
}

#[test]
fn create_island_kandidat_gleich_eingebettet() {
    let plain = make_island("gleich", &[0, 1], CreateOptions::default()).unwrap_or_else(|e| panic!("{e}"));
    let cand = make_island("gleich", &[0, 1], candidate(hub_copy())).unwrap_or_else(|e| panic!("{e}"));
    for i in 0..plain.stages.len() {
        let (a, b) = (&plain.stages[i], &cand.stages[i]);
        assert!(
            site_specs_of(a) == site_specs_of(b) && a.level.sites == b.level.sites,
            "Stufe {i}: Kandidat = eingebettete hub.json baut anders"
        );
    }
}

#[test]
fn create_island_ohne_optionalen_platz() {
    let mut h = hub_copy();
    without_kind(&mut h, "healer");
    let isl = make_island("ohne-heiler", &[0], candidate(h)).unwrap_or_else(|e| panic!("{e}"));
    let s = site_of_kind(&isl.stages[0], "healer");
    assert!(s.is_none(), "Heilplatz trotz Entfernen: {s:?}");
}

#[test]
fn create_island_lehnt_ungueltigen_kandidaten_ab() {
    let mut h = hub_copy();
    without_kind(&mut h, "workshop");
    let err = make_island("ohne-werkstatt", &[0], candidate(h)).err();
    assert!(
        err.as_deref().is_some_and(|e| e.contains(RULE_REQUIRED)),
        "Fehler mit {RULE_REQUIRED:?} erwartet, bekommen {err:?}"
    );
}

/// Der Bau behält eine Kopie: Änderungen des Aufrufers am Kandidaten nach `create_island` ändern die Welt nicht. In Rust
/// übernimmt `CreateOptions` den Kandidaten (Besitz); die Kopie des Aufrufers ist unabhängig vom Klon der Welt.
#[test]
fn create_island_kopiert_den_kandidaten() {
    let mut h = hub_copy();
    let isl = make_island("kopie", &[0], candidate(h.clone())).unwrap_or_else(|e| panic!("{e}"));
    let before = isl.stages[0].sites.len();
    h.sites.clear();
    h.site_placement.zone1_wall_units = 1.0;
    assert_eq!(isl.stages[0].sites.len(), before, "Plätze der Welt");
    let hw = hub_of(&isl.stages[0]);
    assert!(
        hw.site_placement.zone1_wall_units != 1.0 && !hw.sites.is_empty(),
        "die Welt liest den Kandidaten des Aufrufers"
    );
}

/// WZ9.1: `create_world_with` baut eine Stufe ohne Insel aus dem Kandidaten (Vorschau des Gebäude-Planers).
#[test]
#[ignore = "wartet auf RU19 (decode_world_story: create_world lädt die Standard-Welt)"]
fn create_world_with_kandidat() {
    let data = test_data();
    let b = biome_for_depth(&data.biomes, 0).expect("Biom Tiefe 0").clone();
    let mut bad = hub_copy();
    bad.levels.clear();
    let res = create_world_with(&b, "with", &candidate(bad), &[], 0);
    assert!(
        res.is_err(),
        "ungültiger Kandidat: Welt {}, Fehler {:?}",
        res.is_ok(),
        res.as_ref().err()
    );
    let plain = create_world(
        &b,
        "with",
        &Options {
            cycle_speed: 1.0,
            ..Options::default()
        },
    )
    .unwrap_or_else(|e| panic!("{e}"));
    let mut h = hub_copy();
    let cand = create_world_with(&b, "with", &candidate(h.clone()), &[], 0).unwrap_or_else(|e| panic!("{e}"));
    let want = serde_json::to_string(&plain).expect("json");
    let got = serde_json::to_string(&cand).expect("json");
    assert_eq!(
        got, want,
        "Kandidat = eingebettete hub.json: Welt ungleich create_world"
    );
    h.levels.clear();
    h.sites[0].hub_level = 99; // Aufrufer ändert den Kandidaten nach dem Bau
    assert!(
        !hub_of(&cand).levels.is_empty() && hub_of(&cand).sites[0].hub_level != 99,
        "Änderung am Kandidaten nach dem Bau wirkt auf die Welt (keine Kopie)"
    );
    let _ = Arc::new(h);
}

/// WZ24.3 (B-479/AC-04): Die geplanten Chunks der Welt werden bei der Hub-Prüfung mitgeprüft; die eingebetteten Daten sind
/// gültig.
#[test]
fn hub_pruefung_schliesst_geplante_chunks_ein() {
    let world = parse_test_world(
        r#","depths":[0,1,4],"plannedChunks":{"1":[{"slot":2,"kind":"cavern","objects":[{"kind":"stoneVein","at":36}]}]}"#,
    )
    .unwrap_or_else(|e| panic!("{e}"));
    let b = base().unwrap_or_else(|e| panic!("{e}"));
    let f = validate_planned(&world, &hub(), &b.data.biomes, &b.rules);
    assert!(f.is_empty(), "gültige Planung abgelehnt: {f:?}");
}
