use crate::biome::Biome;
use crate::planned::geometry_of;
use crate::planned_gen::Options;
use crate::testutil::{biome, biome_ids, gen2, generate, generate_with, obj, p, plan_ok, rules, validate_with};
use crate::types::{Entity, Layout, PlannedChunk};

/// Eine gültige Planung für jedes Biom: ein Chunk mit Chunk-Seed, ein festes Portal, eine feste Truhe und, wo das
/// Biom Adern hat, eine feste Ader hinter dem äußersten Tor.
fn example_plan(b: &Biome) -> Vec<PlannedChunk> {
    let g = geometry_of(b, 0, rules().hub_sites.clone());
    let first = b.chunk_weights.0.first().map(|e| e.key.clone()).unwrap_or_default();
    let seeded = PlannedChunk {
        slot: -3,
        kind: first.clone(),
        seed: "lichtung".into(),
        ..PlannedChunk::default()
    };
    let mut plan = vec![seeded, p(4, "portal", &[]), p(-1, "chest", &[])];
    if b.veins.count > 0 {
        let mut slot = 1;
        while g.dist(slot, 2.0) < 136.0 {
            slot += 1;
        }
        plan.push(p(slot, &first, &[obj(&b.veins.kind, 10.0)]));
    }
    if let Err(e) = plan_ok(b, &plan) {
        panic!("Beispiel-Planung für {} ungültig: {e}", b.id);
    }
    plan
}

#[test]
fn generate_with_nullwert_ist_generate() {
    for id in biome_ids() {
        let b = biome(&id);
        for seed in ["a", "7", "Käse🧀"] {
            let want = generate(&b, seed);
            let got = generate_with(&b, seed, &Options::default());
            let same = got
                .as_ref()
                .is_ok_and(|g| g.entities == want.entities && g.sites == want.sites);
            assert!(
                same,
                "Biom {id}, Seed {seed}: generate_with(Options::default()) weicht von generate ab"
            );
        }
    }
}

#[test]
fn geplante_chunks_sind_deterministisch_und_gueltig() {
    for id in biome_ids() {
        let b = biome(&id);
        let plan = example_plan(&b);
        let o = Options {
            planned: plan.clone(),
            ..Options::default()
        };
        for seed in 0..500 {
            let s = seed.to_string();
            let l = gen2(&b, &s, &plan);
            let errs = validate_with(&l, &b, &o); // feste Adern zählen mit (B-891)
            assert!(errs.is_empty(), "Biom {id}, Seed {s}: {errs:?}");
            let again = gen2(&b, &s, &plan);
            assert!(
                l.entities == again.entities && l.chunks == again.chunks,
                "Biom {id}, Seed {s}: nicht deterministisch"
            );
        }
    }
}

#[test]
fn festes_portal_liegt_am_erwarteten_ort() {
    let b = biome("forest");
    let plan = [p(6, "portal", &[])]; // Hub + 300 … + 350, Mitte bei + 325
    for seed in 0..20 {
        let l = gen2(&b, &seed.to_string(), &plan);
        let mut xs: Vec<f64> = l
            .entities
            .iter()
            .filter(|e| e.kind == "portal")
            .map(|e| e.x - l.hub_center_units)
            .collect();
        xs.sort_by(f64::total_cmp);
        assert!(
            xs.len() == 2 && xs[1] == 325.0 && xs[0] < 0.0,
            "Seed {seed}: Portale bei {xs:?}, erwartet eines bei +325 und das zweite links"
        );
    }
}

/// Die Objekte innerhalb des Chunks auf Platz -3 (Hub-Abstand statt X), ohne Variante.
fn contents(b: &Biome, l: &Layout) -> Vec<Entity> {
    let start = l.hub_center_units - 50.0 - 3.0 * b.chunk_width_units;
    l.entities
        .iter()
        .filter(|e| e.x >= start && e.x < start + b.chunk_width_units)
        .map(|e| Entity {
            kind: e.kind.clone(),
            x: e.x - l.hub_center_units,
            ..Entity::default()
        }) // Variante folgt der Mischung der Welt (B-891)
        .collect()
}

/// AC-03: Der Chunk-Seed macht den Inhalt in jedem Level gleich, gemessen am Abstand zum Hub.
#[test]
fn chunk_seed_ist_ueber_level_seeds_stabil() {
    let b = biome("forest");
    let seeded = PlannedChunk {
        slot: -3,
        kind: "forest".into(),
        seed: "lichtung".into(),
        ..PlannedChunk::default()
    };
    let plan = [seeded];
    let want = contents(&b, &gen2(&b, "0", &plan));
    assert!(!want.is_empty(), "der Chunk enthält nichts");
    for seed in 1..100 {
        // Skill-Punkte würfeln im Haupt-Strom über freie Chunks und liegen nie im geplanten Chunk.
        let got = contents(&b, &gen2(&b, &seed.to_string(), &plan));
        assert_eq!(want, got, "Seed {seed}");
    }
}

/// AC-04: Der Inhalt eines geplanten Chunks würfelt nur aus seinem eigenen Strom; alle anderen Chunks bleiben gleich.
#[test]
fn inhalt_geplanter_chunks_stoert_den_haupt_strom_nicht() {
    let b = biome("forest");
    let seeded = |s: &str| PlannedChunk {
        slot: -3,
        kind: "forest".into(),
        seed: s.into(),
        ..PlannedChunk::default()
    };
    let variants = [
        vec![seeded("a")],
        vec![seeded("b")],
        vec![p(-3, "forest", &[obj("tree", 10.0)])],
        vec![p(-3, "forest", &[obj("rock", 20.0), obj("bush", 30.0)])],
    ];
    let outside = |l: &Layout| -> Vec<Entity> {
        let start = l.hub_center_units - 50.0 - 3.0 * b.chunk_width_units;
        l.entities
            .iter()
            .filter(|e| !(e.x >= start && e.x < start + b.chunk_width_units))
            .cloned()
            .collect()
    };
    for seed in 0..50 {
        let s = seed.to_string();
        let want = outside(&gen2(&b, &s, &variants[0]));
        for (i, plan) in variants[1..].iter().enumerate() {
            assert_eq!(
                want,
                outside(&gen2(&b, &s, plan)),
                "Seed {s}, Variante {}: ein geplanter Chunk verschiebt andere Objekte",
                i + 1
            );
        }
    }
}

#[test]
fn generate_with_lehnt_ungueltige_planung_ab() {
    let o = Options {
        planned: vec![p(0, "forest", &[])],
        ..Options::default()
    };
    assert!(
        generate_with(&biome("forest"), "x", &o).is_err(),
        "ungültige Planung muss abgelehnt werden"
    );
}
