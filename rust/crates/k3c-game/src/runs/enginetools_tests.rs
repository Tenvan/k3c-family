//! Port von `engine/enginetools/enginetools_test.go` (alle fünf Tests, RU33.1).

use k3c_data::Source;

use super::RunError;
use super::enginetools_level::{BIOMES, level};
use super::enginetools_run::{MAX_SEGMENTS, MAX_TICKS, Segment, run};
use super::stop::Stop;

fn source() -> Source {
    Source::Dir(format!("{}/../../../data", env!("CARGO_MANIFEST_DIR")).into())
}

fn text(r: Result<String, RunError>) -> String {
    r.expect("Lauf")
}

fn message(r: Result<String, RunError>) -> String {
    match r {
        Err(RunError::Failed(m) | RunError::Usage(m)) => m,
        other => panic!("kein Fehler: {other:?}"),
    }
}

/// B-047/AC-03: `level` liefert für die Golden-Seeds Breite und Abschnitte wie `testdata/golden/`.
#[test]
fn level_stimmt_mit_golden_ueberein() {
    for biome in BIOMES {
        let path = format!(
            "{}/../../../testdata/golden/level-{biome}.json",
            env!("CARGO_MANIFEST_DIR")
        );
        let g: serde_json::Value =
            serde_json::from_str(&std::fs::read_to_string(&path).expect("Golden")).expect("JSON");
        let levels = g["levels"].as_array().expect("levels");
        assert!(!levels.is_empty(), "{biome}");
        for want in levels {
            let seed = want["seed"].as_str().expect("seed");
            let t = text(level(&source(), biome, seed));
            let width = k3c_det::golden::go_float(want["widthUnits"].as_f64().expect("widthUnits"));
            assert!(
                t.contains(&format!("Breite {width} Units")),
                "{biome}/{seed}: Breite {width} fehlt"
            );
            let got: Vec<Vec<&str>> = t
                .lines()
                .filter_map(|l| {
                    let p: Vec<&str> = l.split_whitespace().collect();
                    (p.len() == 3 && p[0].parse::<i64>().is_ok() && p[2].starts_with('@')).then_some(p)
                })
                .collect();
            let chunks = want["chunks"].as_array().expect("chunks");
            assert_eq!(got.len(), chunks.len(), "{biome}/{seed}: Abschnitte");
            for (g, c) in got.iter().zip(chunks) {
                let start: f64 = g[2][1..].parse().expect("Start");
                assert_eq!(g[0], c["index"].to_string());
                assert_eq!(g[1], c["kind"].as_str().expect("kind"));
                assert_eq!(start, c["startUnits"].as_f64().expect("startUnits"), "{biome}/{seed}");
            }
        }
    }
}

/// B-047/AC-04: gleicher Lauf zweimal gibt gleichen Text, anderer Seed einen anderen.
#[test]
fn run_ist_deterministisch() {
    let input = [
        Segment {
            from_tick: 0,
            to_tick: 600,
            move_x: 1.0,
            sprint: true,
            ..Segment::default()
        },
        Segment {
            player: 1,
            from_tick: 100,
            to_tick: 900,
            pay: true,
            ..Segment::default()
        },
    ];
    let go = |seed: &str| text(run(&source(), "forest", seed, 3000, &input, &Stop::never()));
    let a = go("abc");
    assert_eq!(a, go("abc"));
    for want in ["3000 Ticks", "Gold je Monarch:", "Ende: "] {
        assert!(a.contains(want), "{want} fehlt in\n{a}");
    }
    assert_ne!(go("xyz"), a);
}

#[test]
fn run_lehnt_ticks_ueber_der_grenze_ab() {
    for ticks in [MAX_TICKS + 1, 0, -5] {
        let m = message(run(&source(), "forest", "abc", ticks, &[], &Stop::never()));
        assert!(m.contains("100000"), "ticks {ticks}: {m}");
    }
}

/// Go läuft hier 100000 Ticks; in Rust nur auf Wunsch (Sim-Sperre, lange Läufe).
#[test]
#[ignore = "lang"]
fn run_genau_die_grenze_geht() {
    assert!(run(&source(), "forest", "abc", MAX_TICKS, &[], &Stop::never()).is_ok());
}

#[test]
fn eingaben_und_biom_werden_geprueft() {
    assert!(message(level(&source(), "hoelle", "0")).contains("forest, cave, mine"));
    let bad = [
        Segment {
            player: 4,
            to_tick: 1,
            ..Segment::default()
        },
        Segment {
            player: -1,
            to_tick: 1,
            ..Segment::default()
        },
        Segment {
            from_tick: 5,
            to_tick: 2,
            ..Segment::default()
        },
        Segment {
            from_tick: -1,
            to_tick: 2,
            ..Segment::default()
        },
    ];
    for b in bad {
        assert!(
            run(&source(), "forest", "0", 10, std::slice::from_ref(&b), &Stop::never()).is_err(),
            "{b:?} akzeptiert"
        );
    }
}

#[test]
fn zu_viele_segmente_werden_abgelehnt() {
    let m = message(run(
        &source(),
        "forest",
        "0",
        10,
        &vec![Segment::default(); MAX_SEGMENTS + 1],
        &Stop::never(),
    ));
    assert!(m.contains("höchstens 100"), "{m}");
    assert!(
        run(
            &source(),
            "forest",
            "0",
            10,
            &vec![Segment::default(); MAX_SEGMENTS],
            &Stop::never()
        )
        .is_ok()
    );
}
