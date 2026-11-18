//! Go: `dev_spawn_test.go` (B-766): Die Dev-Aktion spawn setzt Gegner und Truppen gleichmäßig um den Spieler und lehnt
//! Ungültiges ab.

use std::collections::BTreeSet;

use super::spawn::{DEV_MAX_SPAWN, dev_spawn_units, dev_spawn_x};
use super::testutil::island_with_player;

const SPAN: f64 = 40.0;

#[test]
fn spawn_x_verteilt_gleichmaessig() {
    assert_eq!(dev_spawn_x(10.0, 40.0, 0, 1), 10.0);
    assert_eq!(dev_spawn_x(10.0, 40.0, 0, 3), -10.0);
    assert_eq!(dev_spawn_x(10.0, 40.0, 1, 3), 10.0);
    assert_eq!(dev_spawn_x(10.0, 40.0, 2, 3), 30.0);
}

#[test]
fn gegner_stehen_um_den_spieler() {
    let (data, mut isl, index) = island_with_player("dev-spawn");
    let px = isl.stages[0].players[0].x;
    let before = isl.stages[0].enemies.len();
    let n = dev_spawn_units(&mut isl, &data.difficulty, index, "enemy", "wolf", 20).expect("spawn");
    assert_eq!(n, 20);
    let w = &isl.stages[0];
    assert_eq!(w.enemies.len() - before, 20);
    for e in &w.enemies[before..] {
        assert!(
            e.kind == "wolf" && (e.x - px).abs() <= SPAN / 2.0 + 1e-9 && e.hp > 0.0,
            "Gegner {e:?}"
        );
    }
}

#[test]
fn truppen_haben_hp_und_eigene_ids() {
    let (data, mut isl, index) = island_with_player("dev-spawn-t");
    let before = isl.stages[0].troops.len();
    dev_spawn_units(&mut isl, &data.difficulty, index, "troop", "archer", 5).expect("spawn");
    let w = &isl.stages[0];
    assert_eq!(w.troops.len() - before, 5);
    let last = w.troops.last().expect("Truppe");
    assert!(
        last.kind == "archer" && last.hp > 0.0 && last.max_hp == last.hp,
        "Truppe {last:?}"
    );
    let ids: BTreeSet<i64> = w.troops.iter().map(|t| t.id).collect();
    assert_eq!(ids.len(), w.troops.len(), "doppelte ID");
}

#[test]
fn ungueltiges_wird_abgelehnt() {
    let (data, mut isl, index) = island_with_player("dev-spawn-bad");
    let cases: [(&str, i64, &str, &str, i64); 6] = [
        ("Spieler unbekannt", 7, "enemy", "wolf", 1),
        ("Art unbekannt", index, "boss", "wolf", 1),
        ("Sorte unbekannt", index, "enemy", "drache", 1),
        ("Truppe unbekannt", index, "troop", "wolf", 1),
        ("null", index, "enemy", "wolf", 0),
        ("zu viele", index, "enemy", "wolf", DEV_MAX_SPAWN + 1),
    ];
    for (name, idx, kind, unit, count) in cases {
        assert!(
            dev_spawn_units(&mut isl, &data.difficulty, idx, kind, unit, count).is_err(),
            "{name} angenommen"
        );
    }
    assert!(isl.stages[0].enemies.is_empty(), "abgelehnte Aktionen setzten Gegner");
}

#[test]
fn einzelne_figur_steht_am_spieler() {
    let (data, mut isl, index) = island_with_player("dev-spawn-1");
    let px = isl.stages[0].players[0].x;
    dev_spawn_units(&mut isl, &data.difficulty, index, "enemy", "greed", 1).expect("spawn");
    let e = isl.stages[0].enemies.last().expect("Gegner");
    assert_eq!(e.x, px);
}
