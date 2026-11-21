//! HB1.5 (B-1188): Hafen nah am Lager (1 bis 2 Chunks, nach Seed), Start am Steg, Hafen bewachsen.

use super::testutil::*;
use super::{CreateOptions, add_island_player, create_island};

fn island(seed: &str) -> super::Island {
    let data = test_data();
    create_island(&data, &standard_world(&data), seed, &[0], ISLAND_SPEED, CreateOptions::default()).expect("Insel")
}

/// Abstand Hafen-Mitte zur Burg in Chunks (Breite 50): 1 bis 2 Chunks zwischen Rand der Burg und Hafen-Mitte.
fn dist_chunks(w: &crate::state::World) -> f64 {
    let hx = super::harbor_start::harbor_x(w).expect("Hafen");
    (w.hub_x - hx).abs() / w.biome.chunk_width_units
}

#[test]
fn hafen_liegt_je_seed_ein_oder_zwei_chunks_vom_lager() {
    let mut seen = std::collections::BTreeSet::new();
    for i in 0..40 {
        let isl = island(&format!("hafen-{i}"));
        let w = &isl.stages[0];
        let d = (dist_chunks(w) * 2.0).round() as i64; // Halbchunks, um 1,5 und 2,5 zu trennen
        seen.insert(d);
        assert!((3..=5).contains(&d), "Seed {i}: Abstand {d} Halbchunks");
    }
    assert!(seen.len() > 1, "der Seed bestimmt den Abstand: {seen:?}");
}

#[test]
fn neue_spiele_starten_am_hafen() {
    let mut isl = island("start");
    let pi = add_island_player(&mut isl, 0);
    let w = &isl.stages[0];
    let hx = super::harbor_start::harbor_x(w).expect("Hafen");
    assert!((w.players[pi].x - hx).abs() <= 3.5, "Start {} Hafen {}", w.players[pi].x, hx);
}

#[test]
fn hafen_ist_bewachsen_bis_auf_den_startstreifen() {
    let isl = island("baeume");
    let w = &isl.stages[0];
    let hx = super::harbor_start::harbor_x(w).expect("Hafen");
    let near = |r: f64| w.nodes.iter().filter(|n| n.kind == "tree" && (n.x - hx).abs() <= r).count();
    assert!(near(w.biome.chunk_width_units / 2.0) as f64 >= w.biome.chunk_width_units * 0.5, "dicht bewachsen");
    assert_eq!(near(super::harbor_start::START_GAP - 0.1), 0, "Startstreifen frei");
}
