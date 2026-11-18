//! Tests zu `waves.rs` (Go hat dafür keine eigene Testdatei; die Wellen-Tests stehen in `pools_tests.rs`).

use k3c_rng::Rng;

use super::testutil::*;
use super::waves::*;
use crate::state::{Projectile, Troop};

#[test]
fn scaled_rundet_und_haelt_mindestens_eins() {
    assert_eq!(scaled(0, 2.0), 0);
    assert_eq!(scaled(-3, 2.0), -3);
    assert_eq!(scaled(7, 1.0), 7);
    assert_eq!(scaled(5, 1.5), 8); // 7,5 rundet auf
    assert_eq!(scaled(1, 0.1), 1);
}

#[test]
fn plan_wave_ohne_portale_ist_leer() {
    let v = test_values();
    let b = super::pools_tests::biome_for_depth(1);
    let mut r = Rng::new("leer");
    assert!(plan_wave(&v, &b, 1, &mut r, &[], false, 1.0).is_empty());
}

#[test]
fn plan_wave_verteilt_auf_die_portale_und_ist_deterministisch() {
    let v = test_values();
    let b = super::pools_tests::biome_for_depth(1);
    let portals = [10.0, 50.0];
    let mut r = Rng::new("reihum");
    let plan = plan_wave(&v, &b, 2, &mut r, &portals, false, 1.0);
    assert!(!plan.is_empty());
    let spread = v.waves.spawn_spread_seconds;
    for s in &plan {
        assert!(portals.contains(&s.x));
        assert!(s.delay >= 0.0 && s.delay < spread + 2.0, "Verzögerung {}", s.delay);
    }
    // Gleicher Seed, gleicher Plan (rng-Reihenfolge fest).
    let mut r2 = Rng::new("reihum");
    assert_eq!(plan, plan_wave(&v, &b, 2, &mut r2, &portals, false, 1.0));
}

#[test]
fn queue_wave_stellt_mit_faktoren_in_die_warteschlange() {
    let mut w = bare_world();
    w.time = 10.0;
    let plan = vec![SpawnOrder {
        kind: "skeleton".into(),
        x: 5.0,
        delay: 2.5,
    }];
    queue_wave(&mut w, &plan, 1.5, 1.25);
    let q = &w.spawn_queue[0];
    assert_eq!(
        (q.kind.as_str(), q.x, q.at, q.hp_factor, q.damage_factor),
        ("skeleton", 5.0, 12.5, 1.5, 1.25)
    );
}

#[test]
fn step_spawns_laesst_faellige_entstehen_und_behaelt_den_rest() {
    let mut w = bare_world();
    w.time = 10.0;
    let plan = [
        SpawnOrder {
            kind: "skeleton".into(),
            x: 5.0,
            delay: -1.0,
        },
        SpawnOrder {
            kind: "skeleton".into(),
            x: 6.0,
            delay: 4.0,
        },
    ];
    queue_wave(&mut w, &plan, 1.0, 1.0);
    let v = w.vals.clone().expect("Wertesatz");
    super::step_spawns(&mut w, &v, None);
    assert_eq!(w.enemies.len(), 1);
    assert_eq!(w.enemies[0].x, 5.0);
    assert_eq!(w.spawn_queue.len(), 1);
}

#[test]
fn target_x_findet_gegner_truppe_platz_und_burg() {
    let mut w = bare_world();
    let e = spawn(&mut w, "skeleton", 33.0);
    let tid = w.new_id();
    w.troops.push(Troop {
        id: tid,
        x: 44.0,
        ..Troop::default()
    });
    let s = built_site(&mut w, "workshop", 55.0);
    assert_eq!(target_x(&w, w.enemies[e].id), Some(33.0));
    assert_eq!(target_x(&w, tid), Some(44.0));
    assert_eq!(target_x(&w, w.sites[s].id), Some(55.0));
    assert_eq!(target_x(&w, w.castle.id), Some(w.hub_x));
    assert_eq!(target_x(&w, 9999), None);
}

#[test]
fn step_projectiles_bewegt_und_entfernt_geschosse_ohne_ziel() {
    let mut w = bare_world();
    let tid = w.new_id();
    w.troops.push(Troop {
        id: tid,
        x: 50.0,
        ..Troop::default()
    });
    let lost = w.new_id();
    let (a, b) = (w.new_id(), w.new_id());
    w.projectiles = vec![
        Projectile {
            id: a,
            x: 10.0,
            target_id: tid,
            speed: 20.0,
            ..Projectile::default()
        },
        Projectile {
            id: b,
            x: 10.0,
            target_id: lost,
            speed: 20.0,
            ..Projectile::default()
        },
    ];
    step_projectiles(&mut w, None, 0.5);
    assert_eq!(w.projectiles.len(), 1, "das Geschoss ohne Ziel fällt weg");
    assert_eq!(w.projectiles[0].id, a);
    assert_eq!(w.projectiles[0].x, 20.0);
}

#[test]
fn remove_dead_enemies_ohne_tote_aendert_nichts() {
    let mut w = bare_world();
    spawn(&mut w, "skeleton", 33.0);
    super::waves::remove_dead_enemies(&mut w, None);
    assert_eq!(w.enemies.len(), 1);
}
