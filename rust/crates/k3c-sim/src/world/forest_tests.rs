//! WD1.1 (B-1204): voller Baumwuchs, Hub-Streifen und Lichtungen auf der Oberfläche über viele Seeds.

use super::{CLEARING_RADIUS, clearing_centres, hub_strip_half};
use crate::state::World;
use crate::world::testutil::new_world;
use crate::world::{Options, vals};

fn surface(seed: &str) -> World {
    new_world(0, seed, Options::default())
}

fn trees(w: &World) -> Vec<f64> {
    w.nodes
        .iter()
        .filter(|n| n.kind == "tree" && !n.plant)
        .map(|n| n.x)
        .collect()
}

/// Jeder Chunk außer dem Hub hat mindestens die untere Dichte (Hafen-Chunk und Lichtungen ausgenommen).
#[test]
fn jeder_chunk_hat_volle_dichte() {
    for i in 0..40 {
        let w = surface(&format!("wald{i}"));
        assert!(vals(&w).native_rules, "native Regeln erwartet");
        let (cw, clearings, strip) = (w.biome.chunk_width_units, clearing_centres(&w), hub_strip_half(&w));
        let xs = trees(&w);
        for c in &w.level.chunks {
            if c.kind == "hub" {
                continue;
            }
            let (a, b) = (c.start_units, c.start_units + cw);
            let free: f64 = (0..50)
                .map(|k| a + (f64::from(k) + 0.5) * cw / 50.0)
                .filter(|x| {
                    (x - w.hub_x).abs() >= strip && clearings.iter().all(|cc| (x - cc).abs() >= CLEARING_RADIUS)
                })
                .count() as f64
                / 50.0;
            let n = xs.iter().filter(|x| **x >= a && **x < b).count() as f64;
            assert!(
                n >= 28.0 * free,
                "Seed {i}, Chunk {} ({}): nur {n} Bäume bei {free:.2} bebaubarem Anteil",
                c.index,
                c.kind
            );
        }
    }
}

/// Hub-Streifen und alle Lichtungen sind baumfrei, ein Streifen von der Burg bis kurz vor die erste Mauer.
#[test]
fn hub_streifen_und_lichtungen_sind_baumfrei() {
    for i in 0..40 {
        let w = surface(&format!("wald{i}"));
        let (clearings, strip) = (clearing_centres(&w), hub_strip_half(&w));
        assert!(strip > 8.0, "Seed {i}: Hub-Streifen {strip} zu schmal");
        for x in trees(&w) {
            assert!((x - w.hub_x).abs() >= strip, "Seed {i}: Baum {x} im Hub-Streifen");
            assert!(
                clearings.iter().all(|c| (x - c).abs() >= CLEARING_RADIUS - 0.01
                    || (x - c).abs() <= 4.0 && w.camps.iter().any(|k| (k.x - c).abs() < 0.1)),
                "Seed {i}: Baum {x} in einer Lichtung"
            );
        }
        assert!(!clearings.is_empty(), "Seed {i}: keine Ereignis-Orte");
    }
}

/// Ohne native Regeln bleibt das Level wie in Go (Wert der Welt).
#[test]
fn ohne_native_regeln_kein_voller_wald() {
    let mut v = (*crate::world::values_room::default_values()).clone();
    v.native_rules = false;
    let opts = Options {
        vals: Some(std::sync::Arc::new(v)),
        ..Options::default()
    };
    let legacy = new_world(0, "wald1", opts);
    assert!(
        trees(&legacy).len() < trees(&surface("wald1")).len(),
        "native Welt hat mehr Bäume"
    );
}
