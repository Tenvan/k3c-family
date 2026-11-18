use k3c_data::guards;

use crate::testutil::{biome, biome_ids, gen2, p, plan_ok, rules, with_ruin};
use crate::types::{PlannedChunk, PlannedEnemy};

fn with_guards(mut c: PlannedChunk, list: &[PlannedEnemy]) -> PlannedChunk {
    c.enemies = list.to_vec();
    c
}

fn guard(kind: &str, at: f64) -> PlannedEnemy {
    PlannedEnemy { kind: kind.into(), at }
}

#[test]
fn wachen_werte_aus_daten() {
    let r = &rules().guard;
    assert!(
        r.aggro_radius_units == 10.0 && r.max_per_chunk == 4 && r.min_gap_units == 1.5,
        "data/guards.json: {r:?}"
    );
}

#[test]
fn wachen_werte_lehnt_ab() {
    let cases = [
        ("Radius 0", (0, 4, 1), "aggroRadiusUnits"),
        ("keine Wache", (10, 0, 1), "maxPerChunk"),
        ("Abstand 0", (10, 4, 0), "minGapUnits"),
        ("gültig", (10, 4, 1), ""),
    ];
    for (name, (radius, max, gap), want) in cases {
        let raw = format!(r#"{{"aggroRadiusUnits":{radius},"maxPerChunk":{max},"minGapUnits":{gap}}}"#);
        let err = guards::load(&raw).map(|_| ()).map_err(|e| e.to_string());
        let ok = if want.is_empty() {
            err.is_ok()
        } else {
            err.as_ref().is_err_and(|e| e.contains(want))
        };
        assert!(ok, "{name}: Fehler {err:?}, erwartet {want:?}");
    }
}

#[test]
fn gegnerlager_gueltig() {
    let forest = biome("forest");
    let seed = PlannedChunk {
        slot: 4,
        kind: "forest".into(),
        seed: "lager".into(),
        ..PlannedChunk::default()
    };
    let four = [
        guard("greed", 10.0),
        guard("goblin", 20.0),
        guard("goblinArcher", 30.0),
        guard("goblin", 40.0),
    ];
    let cases: Vec<(&str, Vec<PlannedChunk>)> = vec![
        (
            "eine Wache",
            vec![with_guards(p(4, "forest", &[]), &[guard("goblin", 20.0)])],
        ),
        ("vier Wachen", vec![with_guards(p(4, "forest", &[]), &four)]),
        (
            "links",
            vec![with_guards(p(-4, "forest", &[]), &[guard("greed", 25.0)])],
        ),
        (
            "mit Ruine",
            vec![with_guards(
                with_ruin(p(4, "forest", &[]), "smithy", 10.0),
                &[guard("goblin", 14.0)],
            )],
        ),
        ("mit Chunk-Seed", vec![with_guards(seed, &[guard("goblin", 25.0)])]),
        (
            "Abstand genau 2",
            vec![with_guards(
                p(4, "forest", &[]),
                &[guard("goblin", 2.0), guard("goblin", 3.5)],
            )],
        ),
    ];
    for (name, plan) in cases {
        if let Err(e) = plan_ok(&forest, &plan) {
            panic!("{name}: {e}");
        }
    }
}

#[test]
fn gegnerlager_lehnt_ab() {
    let forest = biome("forest");
    let at4 = |list: &[PlannedEnemy]| vec![with_guards(p(4, "forest", &[]), list)];
    let five = [5.0, 10.0, 15.0, 20.0, 25.0].map(|at| guard("greed", at));
    let cases: Vec<(&str, Vec<PlannedChunk>, &str)> = vec![
        ("zu viele Wachen", at4(&five), "höchstens 4 Wachen"),
        ("Art nicht im Biom", at4(&[guard("skeleton", 20.0)]), "nicht erlaubt"),
        ("Nachtgegner", at4(&[guard("wolf", 20.0)]), "nicht erlaubt"),
        ("unbekannte Art", at4(&[guard("drache", 20.0)]), "nicht erlaubt"),
        (
            "zu nah an Grenze",
            at4(&[guard("goblin", 1.0)]),
            "Abstand zur Chunk-Grenze",
        ),
        (
            "zu nah an Grenze 2",
            at4(&[guard("goblin", 49.0)]),
            "Abstand zur Chunk-Grenze",
        ),
        (
            "innerhalb der Verteidigungszone",
            vec![with_guards(p(1, "forest", &[]), &[guard("goblin", 20.0)])],
            "innerhalb der Verteidigungszone",
        ),
        (
            "Wachen zu nah",
            at4(&[guard("goblin", 20.0), guard("greed", 21.0)]),
            "weniger als",
        ),
    ];
    for (name, plan, want) in cases {
        let err = plan_ok(&forest, &plan);
        let ok = err.as_ref().is_err_and(|e| e.contains(want) && e.contains("auf Platz"));
        assert!(ok, "{name}: Fehler {err:?}, erwartet mit Platz und {want:?}");
    }
}

/// Die Wachen liegen im Layout; sie würfeln nichts: Level mit und ohne Lager sind bis auf `Layout::guards` gleich.
#[test]
#[ignore = "wartet auf RU6.5"]
fn gegnerlager_im_level_ohne_wuerfe() {
    let forest = biome("forest");
    let plain = [p(3, "forest", &[])];
    let with = [with_guards(
        p(3, "forest", &[]),
        &[guard("goblin", 30.0), guard("greed", 35.0)],
    )];
    for seed in ["a", "lager", "Käse🧀"] {
        let (a, mut b) = (gen2(&forest, seed, &plain), gen2(&forest, seed, &with));
        let start = b.hub_center_units + 50.0 + 2.0 * 50.0;
        let want = vec![
            crate::types::Guard {
                kind: "goblin".into(),
                x: start + 30.0,
            },
            crate::types::Guard {
                kind: "greed".into(),
                x: start + 35.0,
            },
        ];
        assert_eq!(b.guards, want, "Seed {seed:?}: Wachen");
        assert!(a.guards.is_empty(), "Seed {seed:?}: ohne Lager {:?}", a.guards);
        b.guards.clear();
        assert_eq!(a, b, "Seed {seed:?}: das Lager ändert das Level");
    }
}

#[test]
#[ignore = "wartet auf RU6.5"]
fn gegnerlager_alle_biome_generiert_deterministisch() {
    for id in biome_ids() {
        let b = biome(&id);
        if b.enemies.portal.is_empty() {
            continue;
        }
        let first = b
            .chunk_weights
            .0
            .first()
            .map(|e| e.key.clone())
            .unwrap_or_else(|| panic!("Biom {id} ohne chunkWeights"));
        let plan = [with_guards(p(-3, &first, &[]), &[guard(&b.enemies.portal[0], 25.0)])];
        if plan_ok(&b, &plan).is_err() {
            continue; // Plätze nahe am Hub liegen in manchen Biomen innerhalb des Tors; dort gilt die Ablehnung
        }
        for seed in 0..20 {
            let s = seed.to_string();
            let (a, b2) = (gen2(&b, &s, &plan), gen2(&b, &s, &plan));
            assert!(
                a == b2 && a.guards.len() == 1,
                "Biom {id}, Seed {s}: {} Wachen oder nicht deterministisch",
                a.guards.len()
            );
        }
    }
}
