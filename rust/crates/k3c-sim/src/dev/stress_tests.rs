//! Go: `dev_stress_test.go` (B-920, B-921): Gegner in der ersten Nacht, Dev-Welle am ersten Tag, reiner Stressmodus.

use super::spawn::{dev_spawn_at_castle, dev_spawn_units};
use super::testutil::island_with_player;
use super::{dev_set_pure, dev_start_wave};
use crate::island::Island;
use crate::island::step_island;
use crate::state::PlayerCommand;
use crate::state::pending::values;

const DT: f64 = 1.0 / 30.0;

/// Go: `enemiesAfter`: tickt die Insel `seconds` lang und liefert die größte Zahl gleichzeitiger Gegner (ohne Wachen).
fn enemies_after(isl: &mut Island, data: &k3c_data::Data, cmds: &[PlayerCommand], seconds: f64) -> usize {
    let (mut most, mut tm) = (0, 0.0);
    while tm < seconds {
        step_island(isl, cmds, &data.difficulty, DT);
        most = most.max(isl.stages[0].enemies.iter().filter(|e| !e.guard).count());
        tm += DT;
    }
    most
}

#[test]
#[ignore = "wartet auf RU19 (decode_world_story) und RU18 (step_players)"]
fn erste_nacht_bringt_gegner_und_dev_welle_schon_am_tag() {
    let (data, mut isl, _) = island_with_player("stress-nacht");
    let cmds = [PlayerCommand::default()];
    assert_eq!(enemies_after(&mut isl, &data, &cmds, 5.0), 0, "Gegner vor der Nacht");
    assert!(enemies_after(&mut isl, &data, &cmds, 25.0) > 0 && isl.stages[0].wave > 0);

    let (data, mut dev, index) = island_with_player("stress-dev-welle");
    assert!(dev_start_wave(&mut dev, &data.difficulty, index), "Dev-Welle abgelehnt");
    assert!(
        enemies_after(&mut dev, &data, &cmds, 10.0) > 0,
        "keine Gegner nach der Dev-Welle"
    );
}

/// Go: `pureRun`: im reinen Modus 7 Truppen und 12 Gegner an die Burg, tickt über die erste Nacht; liefert die X der
/// gesetzten Gegner und die Zahl der Truppen am Ende.
fn pure_run() -> (Vec<f64>, usize) {
    let (data, mut isl, index) = island_with_player("stress-rein");
    let d = &data.difficulty;
    dev_set_pure(&mut isl, true);
    dev_spawn_units(&mut isl, d, index, "troop", "archer", 7).expect("Truppen");
    assert_eq!(dev_spawn_at_castle(&mut isl, d, index, "enemy", "goblin", 12), Ok(12));
    let radius = values(&isl.stages[0]).hub.castle_radius_units;
    let hub_x = isl.stages[0].hub_x;
    let xs: Vec<f64> = isl.stages[0].enemies.iter().map(|e| e.x).collect();
    assert!(
        xs.iter().all(|x| (x - hub_x).abs() <= radius + 1e-9),
        "Gegner nicht an der Burg"
    );
    for _ in 0..40 * 30 {
        step_island(&mut isl, &[PlayerCommand::default()], d, DT);
    }
    let w = &isl.stages[0];
    assert!(
        w.wave == 0 && w.spawn_queue.is_empty(),
        "Welle {}, wartend {}",
        w.wave,
        w.spawn_queue.len()
    );
    assert!(
        w.troops.iter().all(|t| t.kind != "vagrant"),
        "Landstreicher im reinen Modus"
    );
    (xs, w.troops.len())
}

#[test]
#[ignore = "wartet auf RU19 (decode_world_story) und RU18 (step_players)"]
fn reiner_stressmodus_ist_reproduzierbar() {
    let (x1, t1) = pure_run();
    let (x2, t2) = pure_run();
    assert!(
        x1.len() == 12 && t1 >= 7 && t1 == t2 && x1 == x2,
        "Gegner {}/{}, Truppen {t1}/{t2}",
        x1.len(),
        x2.len()
    );
}
