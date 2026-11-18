use k3c_rng::Rng;

use crate::biome::Biome;
use crate::planned::validate_plan;
use crate::planned_gen::Options;
use crate::testutil::{
    biome, biome_ids, data_dir, generate, generate_with, p, plan_ok, rules, test_gen, validate, validate_with,
};
use crate::types::Layout;

// WZ17.2 (B-485/AC-02, AC-03): Chunk-Anzahl je Stufe (docs/rules/geplante-chunks.md § 7).

/// Go: `sizeCounts`, die Chunk-Anzahlen der Inselgrößen aus data/island-sizes.json (einzige Quelle der Tabelle).
fn size_counts() -> Vec<i64> {
    let raw = std::fs::read_to_string(data_dir().join("island-sizes.json")).unwrap_or_default();
    let v: serde_json::Value = serde_json::from_str(&raw).unwrap_or_default();
    let out: Vec<i64> = v["sizes"]
        .as_array()
        .map(|a| a.iter().filter_map(|s| s["chunkCount"].as_i64()).collect())
        .unwrap_or_default();
    assert_eq!(
        out,
        [16, 24, 40, 64, 100, 160],
        "Inselgrößen {out:?}, laut Regel 16·24·40·64·100·160"
    );
    out
}

/// Mit gesetzter Anzahl würfelt frame() die Länge nicht: Der Haupt-Strom steht danach noch am Anfang.
#[test]
fn frame_mit_anzahl_wuerfelt_nicht() {
    let b = biome("forest");
    let mut g = test_gen(&b, "forest:x", 20);
    g.frame();
    let (got, want) = (g.r.next(), Rng::new("forest:x").next());
    assert_eq!(got, want, "frame() hat mit Anzahl gewürfelt");
    assert_eq!(g.kinds.len(), 20, "{} Chunks, erwartet 20", g.kinds.len());
}

/// Ohne Anzahl bleibt der Wurf: gleiches Level wie Generate (die Golden-Dateien prüfen die Bytes).
#[test]
fn ohne_anzahl_wie_bisher() {
    for id in biome_ids() {
        let b = biome(&id);
        for seed in 0..20 {
            let want = generate(&b, &seed.to_string());
            let got = generate_with(
                &b,
                &seed.to_string(),
                &Options {
                    chunk_count: 0,
                    ..Options::default()
                },
            );
            let same = got.is_ok_and(|g| g.chunks == want.chunks && g.entities == want.entities);
            assert!(same, "Biom {id}, Seed {seed}: ohne Anzahl anders als Generate");
        }
    }
}

/// Je Inselgröße und Biom ein Einzeltest (ein Seed): genau diese Chunks, gerade, Hub mittig, Validate ohne Befund.
/// Kein Nachweis über viele Seeds; geht eine Größe nicht, meldet generate_with einen Fehler.
#[test]
fn chunk_anzahl_je_groesse() {
    for id in biome_ids() {
        let b = biome(&id);
        for n in size_counts() {
            let o = Options {
                chunk_count: n,
                ..Options::default()
            };
            let l = generate_with(&b, "groesse", &o).unwrap_or_else(|e| panic!("Biom {id}, {n} Chunks: {e}"));
            check_frame(&l, &b, n);
            let errs = validate_with(&l, &b, &o);
            assert!(errs.is_empty(), "Biom {id}, {n} Chunks: {errs:?}");
        }
    }
}

/// Go: `checkFrame`: n Chunks, Breite n × Chunk-Breite, Hub-Chunks und Burg genau in der Mitte.
fn check_frame(l: &Layout, b: &Biome, n: i64) {
    let hub_chunks = 2.max(((b.hub_width_units / b.chunk_width_units / 2.0 + 0.5) as i64) * 2);
    let ok = l.chunks.len() as i64 == n
        && l.width_units == n as f64 * b.chunk_width_units
        && l.hub_center_units == l.width_units / 2.0;
    assert!(
        ok,
        "Biom {}: {} Chunks, Breite {}, Hub {}; erwartet {n} Chunks, Hub mittig",
        b.id,
        l.chunks.len(),
        l.width_units,
        l.hub_center_units
    );
    for (i, c) in l.chunks.iter().enumerate() {
        let i = i as i64;
        let in_hub = i >= n / 2 - hub_chunks / 2 && i < n / 2 + hub_chunks / 2;
        assert_eq!(
            in_hub,
            c.kind == "hub",
            "Biom {}, {n} Chunks: Chunk {i} ist {:?}",
            b.id,
            c.kind
        );
    }
}

#[test]
fn validate_mit_anzahl_meldet_falsche_laenge() {
    let b = biome("forest");
    let o = Options {
        chunk_count: 16,
        ..Options::default()
    };
    let l = generate_with(&b, "laenge", &o).unwrap_or_else(|e| panic!("{e}"));
    let errs = validate(&l, &b);
    assert!(
        errs.iter().any(|e| e == "Level zu kurz"),
        "Validate ohne Anzahl = {errs:?}, erwartet „Level zu kurz“"
    );
    let got = validate_with(
        &l,
        &b,
        &Options {
            chunk_count: 24,
            ..Options::default()
        },
    );
    assert_eq!(got, ["Level hat 16 Chunks, erwartet 24"]);
}

#[test]
fn generate_lehnt_ungueltige_anzahl_ab() {
    for n in [-2, 2, 4, 17] {
        let err = generate_with(
            &biome("forest"),
            "x",
            &Options {
                chunk_count: n,
                ..Options::default()
            },
        );
        assert!(
            err.is_err_and(|e| e.contains("Chunk-Anzahl")),
            "Anzahl {n}: kein Fehler"
        );
    }
}

/// AC-03: Ein geplanter Platz außerhalb der Anzahl wird mit Platz abgelehnt; die Prüfung rechnet mit der Anzahl.
#[test]
fn planung_gegen_die_anzahl() {
    let cases = [
        ("forest", 16, 7),
        ("forest", 16, -7),
        ("forest", 20, 9),
        ("ironhold", 16, -6),
        ("crystal", 16, 6),
    ];
    for (id, count, slot) in cases {
        let err = crate::planned::validate_plan_for(&biome(id), &[p(slot, "chest", &[])], count, rules());
        let want = if slot > 0 {
            format!("auf Platz +{slot}")
        } else {
            format!("auf Platz {slot}")
        };
        let ok = err
            .as_ref()
            .is_err_and(|e| e.to_string().contains(&want) && e.to_string().contains(&format!("Chunk-Anzahl {count}")));
        assert!(ok, "{id}, {count} Chunks, Platz {slot}: Fehler {err:?}");
    }
    // Jenseits der kürzesten Länge (Wald ±7), aber innerhalb von 48 Chunks: gültig und im Level an seinem Platz.
    let b = biome("forest");
    let plan = [p(20, "clearing", &[]), p(-6, "chest", &[])];
    assert!(
        validate_plan(&b, &plan, rules()).is_err(),
        "Platz +20 ohne Anzahl angenommen"
    );
    assert!(plan_ok(&b, &plan).is_err());
    let o = Options {
        chunk_count: 48,
        planned: plan.to_vec(),
        ..Options::default()
    };
    let l = generate_with(&b, "weit", &o).unwrap_or_else(|e| panic!("{e}"));
    assert_eq!(l.chunks[24 + 20].kind, "clearing", "Platz +20 bei 48 Chunks");
    let errs = validate_with(&l, &b, &o);
    assert!(errs.is_empty(), "Validate: {errs:?}");
}

/// Mit Planung und Anzahl deterministisch: gleicher Seed, gleiches Level.
#[test]
fn anzahl_und_planung_deterministisch() {
    let b = biome("cave");
    let o = Options {
        chunk_count: 32,
        planned: vec![p(-10, "cavern", &[]), p(8, "chest", &[])],
        ..Options::default()
    };
    let first = generate_with(&b, "Käse🧀", &o).unwrap_or_else(|e| panic!("{e}"));
    let again = generate_with(&b, "Käse🧀", &o).unwrap_or_else(|e| panic!("{e}"));
    assert!(
        first.chunks == again.chunks && first.entities == again.entities,
        "gleicher Seed, anderes Level"
    );
}
