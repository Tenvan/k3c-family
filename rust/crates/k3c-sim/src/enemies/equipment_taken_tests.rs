//! Tests zu `equipment_taken.rs` (Go: `enemies_equipment_taken_test.go`, W7.1, B-312): Gegner tragen fallengelassene
//! Ausrüstung weg.

use super::equipment_taken::take_equipment;
use super::testutil::*;
use crate::state::ItemDrop;

#[test]
fn gegner_nimmt_drop_mit_und_flieht() {
    let mut w = quiet_world();
    add_player(&mut w);
    add_player(&mut w); // 2 Spieler gleichzeitig
    let id = w.new_id();
    w.drops = vec![ItemDrop {
        id,
        kind: "archer".into(),
        x: w.hub_x - 20.0,
        work_site: 0,
    }];
    let at = w.hub_x - 20.0;
    let e = spawn(&mut w, "skeleton", at);
    let eid = w.enemies[e].id;
    let mut taken = 0;
    for _ in 0..90 {
        step(&mut w, None, DT);
        taken += count_events(&w, "equipmentTaken");
    }
    // Go liest `e.Carried` und `e.Fleeing` am Zeiger, auch nachdem der Flüchtende das Portal erreicht hat; in Rust ist er
    // dann aus `w.enemies` entfernt: erwartet wird, dass er entweder fort ist oder trägt und flieht.
    let en = w.enemies.iter().find(|en| en.id == eid);
    assert!(
        taken == 1 && w.drops.is_empty() && en.is_none_or(|en| en.carried.is_some() && en.fleeing),
        "Ereignisse {taken}, am Boden {}",
        w.drops.len()
    );
}

#[test]
fn traeger_stirbt_drop_faellt_zu_boden() {
    let mut w = quiet_world();
    add_player(&mut w);
    let id = w.new_id();
    w.drops = vec![ItemDrop {
        id,
        kind: "warrior".into(),
        x: w.hub_x - 20.0,
        work_site: 0,
    }];
    let at = w.hub_x - 20.0;
    let e = spawn(&mut w, "skeleton", at);
    w.enemies[e].home_x = w.hub_x - 40.0; // sonst verschwindet er beim Greifen sofort im Portal
    step(&mut w, None, DT);
    w.enemies[e].x = w.hub_x - 18.0;
    w.enemies[e].hp = 0.0;
    step(&mut w, None, DT);
    assert!(w.enemies.is_empty() && w.drops.len() == 1 && w.drops[0].kind == "warrior");
    assert!((w.drops[0].x - (w.hub_x - 18.0)).abs() <= 0.5);
}

/// Passiver Spieler (Bot passive): ohne Eingaben fällt die Burg in mindestens einem Seed. Go: `-short` überspringt ihn
/// nicht, er rechnet aber eine ganze Insel durch.
#[test]
fn passiver_spieler_burg_faellt() {
    use crate::island::testutil::{standard_world, test_data};
    let data = test_data();
    let mut fell = false;
    for seed in ["1", "2", "3"] {
        let mut isl = crate::island::create_island(&data, &standard_world(&data), seed, &[0], 1.0, Default::default())
            .expect("Insel");
        crate::island::add_island_player(&mut isl, 0);
        let cmds = [crate::state::PlayerCommand::default()];
        let mut tick = 1;
        while isl.stages[0].cycle.day <= 5 && tick < 5 * 600 * 30 {
            crate::island::step_island(&mut isl, &cmds, &data.difficulty, DT);
            if count_events(&isl.stages[0], "castleFallen") > 0 {
                fell = true;
                break;
            }
            tick += 1;
        }
        if fell {
            break;
        }
    }
    assert!(fell, "Burg fällt in keinem Seed");
}

#[test]
fn take_equipment_nimmt_den_ersten_drop_ausser_hammer() {
    let mut w = bare_world();
    w.portals = vec![w.hub_x - 100.0];
    let v = w.vals.clone().expect("Wertesatz");
    let at = w.hub_x - 20.0;
    let e = spawn(&mut w, "skeleton", at);
    w.enemies[e].boss = false;
    let (h, a) = (w.new_id(), w.new_id());
    w.drops = vec![
        ItemDrop {
            id: h,
            kind: "hammer".into(),
            x: w.hub_x - 20.0,
            work_site: 0,
        },
        ItemDrop {
            id: a,
            kind: "archer".into(),
            x: w.hub_x - 20.0,
            work_site: 0,
        },
    ];
    take_equipment(&mut w, &v, e);
    let grab = v.enemies.get("skeleton").map_or(0.0, |d| d.grab_range);
    if grab > 0.0 {
        assert_eq!(w.enemies[e].carried.as_ref().map(|c| c.kind.as_str()), Some("archer"));
        assert_eq!(w.drops.len(), 1, "der Hammer bleibt liegen");
        assert!(w.enemies[e].fleeing);
    }
}

#[test]
fn take_equipment_ohne_portal_oder_mit_last_tut_nichts() {
    let mut w = bare_world();
    let v = w.vals.clone().expect("Wertesatz");
    let at = w.hub_x - 20.0;
    let e = spawn(&mut w, "skeleton", at);
    let id = w.new_id();
    w.drops = vec![ItemDrop {
        id,
        kind: "archer".into(),
        x: w.hub_x - 20.0,
        work_site: 0,
    }];
    take_equipment(&mut w, &v, e); // ohne Portal
    assert_eq!(w.drops.len(), 1);
    assert!(w.enemies[e].carried.is_none() && !w.enemies[e].fleeing);
}
