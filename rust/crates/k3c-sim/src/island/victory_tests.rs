//! Siegvarianten (Go: `island_victory_test.go`, K2.2a, B-102/AC-01): je Variante nicht erfüllt, knapp nicht, erfüllt und
//! danach kein zweites `victory`. Die Go-Tests bauen eine Insel (`create_world_with`, `step`) und tragen `ignore`;
//! `siege_werte_aus_daten` läuft. Die Tests ab `own_` prüfen `planted_trees`, `all_built`, `all_mined` und
//! `check_victory` an einer Insel von Hand.
use std::sync::Arc;

use super::switch_testutil::{cycle_length, default_values};
use super::testutil::*;
use super::victory::{all_built, all_mined, check_victory, planted_trees};
use super::{Island, add_island_player, set_options, step_island};
use crate::state::{Coin, Event, Plot, ResourceNode, Site, World, WorldValues};

/// Go: `goalIsland` ist eine Insel mit zwei Stufen, einem Spieler je Stufe und dem Ziel `goal`.
fn goal_island(data: &k3c_data::Data, goal: &str) -> Island {
    let mut isl = must_island(data, &format!("k2-sieg-{goal}"), &[0, 1]);
    add_island_player(&mut isl, 0);
    add_island_player(&mut isl, 1);
    let mut o = isl.options.clone();
    o.goal = goal.to_string();
    set_options(&mut isl, o, false).expect("Optionen");
    isl
}

/// Go: `victories` tickt die Insel einmal und liefert die `victory`-Ereignisse aller Stufen.
fn victories(isl: &mut Island, data: &k3c_data::Data) -> Vec<Event> {
    step_island(isl, &[], &data.difficulty, DT);
    let all = isl.stages.iter().flat_map(|w| w.events.iter());
    all.filter(|e| e.get("type").and_then(|t| t.as_str()) == Some("victory"))
        .cloned()
        .collect()
}

fn want_victories(isl: &mut Island, data: &k3c_data::Data, n: usize, step: &str) {
    let got = victories(isl, data);
    assert_eq!(got.len(), n, "{step}: {} victory, erwartet {n} ({got:?})", got.len());
}

/// Go: `wantOnce`: erfüllt, genau ein `victory` mit Variante, danach keines mehr, obwohl das Ziel erfüllt bleibt.
fn want_once(isl: &mut Island, data: &k3c_data::Data) {
    let got = victories(isl, data);
    let ok = got.len() == 1
        && got[0].get("goal").and_then(|g| g.as_str()) == Some(isl.options.goal.as_str())
        && got[0].get("stage").and_then(|s| s.as_i64()) == Some(0);
    assert!(
        ok,
        "erfüllt: victory {got:?}, erwartet eines mit goal {} in Stufe 0",
        isl.options.goal
    );
    assert!(
        got[0].get("day").and_then(|d| d.as_i64()) == Some(isl.stages[0].cycle.day) && isl.won,
        "victory: day {:?}, won {}",
        got[0].get("day"),
        isl.won
    );
    want_victories(isl, data, 0, "danach");
}

#[test]
fn sieg_endboss() {
    let data = test_data();
    let mut isl = goal_island(&data, "endboss");
    want_victories(&mut isl, &data, 0, "Endboss lebt");
    isl.endboss_defeated = true;
    want_once(&mut isl, &data);
}

/// Go: `coinAtPlayer` legt eine Münze vor den Spieler der Stufe.
fn coin_at_player(isl: &mut Island, stage: usize) {
    let w = &mut isl.stages[stage];
    w.players[0].gold = 0;
    let id = w.new_id();
    let x = w.players[0].x;
    w.coins.push(Coin {
        id,
        x,
        ..Coin::default()
    });
}

#[test]
fn sieg_gold_summe_aller_spieler() {
    let data = test_data();
    let goal = data.goals.gold;
    let mut isl = goal_island(&data, "gold");
    want_victories(&mut isl, &data, 0, "nichts gesammelt");
    isl.gold_collected = goal - 2;
    coin_at_player(&mut isl, 0); // Spieler 1 hebt auf
    want_victories(&mut isl, &data, 0, "knapp darunter");
    assert_eq!(isl.gold_collected, goal - 1, "Zähler");
    coin_at_player(&mut isl, 1); // Spieler 2 in der anderen Stufe: genug
    want_once(&mut isl, &data);
}

/// Go: `setDay` stellt die Uhr aller Stufen auf den Anfang von Tag `day` (der Zyklus rechnet den Tag aus der Zeit).
fn set_day(isl: &mut Island, day: i64) {
    let length = cycle_length();
    for w in &mut isl.stages {
        w.time = (day - 1) as f64 * length / w.cycle_speed + DT;
    }
}

#[test]
fn sieg_tage_ueberleben() {
    let data = test_data();
    let days = data.goals.days;
    let mut isl = goal_island(&data, "days");
    want_victories(&mut isl, &data, 0, "Tag 1");
    set_day(&mut isl, days);
    want_victories(&mut isl, &data, 0, "letzter Tag");
    assert_eq!(isl.stages[0].cycle.day, days, "Tag");
    set_day(&mut isl, days + 1);
    want_once(&mut isl, &data);
    assert_eq!(isl.stages[0].cycle.day, days + 1, "Tag");
}

/// Go: `veinKind`, die erste Ader in `data/economy.json`.
fn vein_kind(data: &k3c_data::Data) -> String {
    data.economy
        .veins
        .keys()
        .next()
        .cloned()
        .expect("keine Ader in data/economy.json")
}

/// Go: `onlyVeins`, die Adern unter den Knoten.
fn only_veins(v: &WorldValues, nodes: &[ResourceNode]) -> Vec<ResourceNode> {
    nodes
        .iter()
        .filter(|n| crate::state::pending::is_vein(v, n))
        .cloned()
        .collect()
}

#[test]
fn sieg_alles_abbauen() {
    let data = test_data();
    let vals = default_values(&data);
    let mut isl = goal_island(&data, "mineAll");
    let vein = vein_kind(&data);
    for w in &mut isl.stages {
        // jede Stufe: eine Ader, die nie zählt
        let (id, x) = (w.new_id(), w.hub_x);
        w.nodes.push(ResourceNode {
            id,
            kind: vein.clone(),
            x,
            ..ResourceNode::default()
        });
    }
    want_victories(&mut isl, &data, 0, "volles Level");
    let last = isl.stages[1]
        .nodes
        .iter()
        .position(|n| !crate::state::pending::is_vein(&vals, n))
        .expect("Höhle ohne endliches Ressourcenobjekt");
    isl.stages[0].nodes = only_veins(&vals, &isl.stages[0].nodes);
    let keep = isl.stages[1].nodes[last].clone();
    let mut cave = only_veins(&vals, &isl.stages[1].nodes);
    cave.push(keep);
    isl.stages[1].nodes = cave;
    want_victories(&mut isl, &data, 0, "ein Objekt übrig");
    isl.stages[1].nodes = only_veins(&vals, &isl.stages[1].nodes);
    want_once(&mut isl, &data);
}

#[test]
fn sieg_alles_abbauen_ohne_plantage() {
    let data = test_data();
    let mut isl = goal_island(&data, "mineAll");
    want_victories(&mut isl, &data, 0, "volles Level");
    for w in &mut isl.stages {
        w.nodes.clear();
    }
    let w = &mut isl.stages[0];
    let (nid, tid) = (w.new_id(), w.new_id());
    let tree = ResourceNode {
        id: tid,
        kind: "tree".into(),
        marked: true,
        ..ResourceNode::default()
    };
    let nursery = Site {
        id: nid,
        kind: "nursery".into(),
        state: "built".into(),
        plots: vec![Plot {
            tree: Some(tid),
            ..Plot::default()
        }],
        ..Site::default()
    };
    w.sites.push(nursery);
    w.nodes = vec![tree];
    want_once(&mut isl, &data);
}

#[test]
fn sieg_alles_abbauen_ohne_endliche_objekte_nie() {
    let data = test_data();
    let vein = vein_kind(&data);
    let mut isl = goal_island(&data, "mineAll");
    for w in &mut isl.stages {
        let (id, x) = (w.new_id(), w.hub_x);
        w.nodes = vec![ResourceNode {
            id,
            kind: vein.clone(),
            x,
            ..ResourceNode::default()
        }];
    }
    for _ in 0..30 {
        want_victories(&mut isl, &data, 0, "ohne endliche Objekte");
    }
}

#[test]
fn sieg_alles_ausbauen_ueber_alle_stufen() {
    let data = test_data();
    let mut isl = goal_island(&data, "buildAll");
    want_victories(&mut isl, &data, 0, "nichts gebaut");
    for w in &mut isl.stages {
        for s in &mut w.sites {
            s.state = "built".into();
            s.hp = s.max_hp;
            s.build_progress = 1.0;
        }
    }
    assert!(
        !isl.stages[1].sites.is_empty() && !isl.stages[0].sites.is_empty(),
        "Stufe ohne Bauplatz"
    );
    let open = isl.stages[1].sites.len() - 1;
    isl.stages[1].sites[open].state = "unpaid".into();
    want_victories(&mut isl, &data, 0, "ein Bauplatz offen in der Höhle");
    let s = &mut isl.stages[1].sites[open];
    s.state = "built".into();
    s.hp = s.max_hp;
    s.build_progress = 1.0;
    want_once(&mut isl, &data);
}

/// Mehrere Bedingungen im selben Tick und ein späterer Wechsel des Ziels geben zusammen genau ein `victory`.
#[test]
fn sieg_zwei_bedingungen_ein_victory() {
    let data = test_data();
    let mut isl = goal_island(&data, "gold");
    isl.gold_collected = data.goals.gold;
    isl.endboss_defeated = true;
    set_day(&mut isl, data.goals.days + 1);
    want_once(&mut isl, &data);
    let mut o = isl.options.clone();
    o.goal = "days".into();
    set_options(&mut isl, o, false).expect("Optionen");
    want_victories(&mut isl, &data, 0, "zweites Ziel erfüllt");
}

#[test]
fn sieg_werte_aus_daten() {
    let data = test_data();
    // Startwerte: 66 Gold, 20 Tage (data/goals.json)
    assert_eq!((data.goals.gold, data.goals.days), (66, 20), "goals {:?}", data.goals);
}

#[test]
fn sieg_deterministisch() {
    let data = test_data();
    let play = || {
        let mut isl = goal_island(&data, "gold");
        isl.gold_collected = data.goals.gold - 1;
        let ev = run_island(&mut isl, &data, &[], 20.0);
        (ev, island_json(&isl), isl.gold_collected)
    };
    let (a, b) = (play(), play());
    assert!(a == b, "gleicher Seed, unterschiedlicher Verlauf");
}

/// Eine Insel von Hand mit zwei Stufen, dem Ziel `goal` und dem Wertesatz in jeder Stufe (ohne `values`-Platzhalter).
fn hand_goal_island(goal: &str) -> Island {
    let mut isl = Island::default();
    isl.options.goal = goal.to_string();
    isl.world = super::save_tests::world_of(&["island1"]);
    for _ in 0..2 {
        let w = World {
            vals: Some(Arc::new(WorldValues::default())),
            ..World::default()
        };
        isl.stages.push(w);
    }
    isl
}

#[test]
fn own_check_victory_wartet_bis_das_ziel_erreicht_ist() {
    let mut endboss = hand_goal_island("endboss");
    check_victory(&mut endboss);
    assert!(!endboss.won, "Endboss lebt");
    let mut gold = hand_goal_island("gold");
    gold.stages[0].vals = Some(Arc::new(WorldValues {
        goals: k3c_data::goals::Goals { gold: 10, days: 3 },
        ..WorldValues::default()
    }));
    gold.gold_collected = 9;
    check_victory(&mut gold);
    assert!(!gold.won, "knapp darunter");
    let mut days = hand_goal_island("days");
    days.stages[0].vals = gold.stages[0].vals.clone();
    days.stages[0].cycle.day = 3;
    check_victory(&mut days);
    assert!(!days.won, "Tag 3 von 3 reicht nicht");
    let mut unknown = hand_goal_island("gibtsnicht");
    check_victory(&mut unknown);
    assert!(!unknown.won, "unbekanntes Ziel");
}

#[test]
fn own_check_victory_gilt_nur_auf_der_letzten_insel() {
    let mut isl = hand_goal_island("endboss");
    isl.endboss_defeated = true;
    isl.world = super::save_tests::world_of(&["island1", "island2"]);
    check_victory(&mut isl);
    assert!(!isl.won, "es gibt ein Ziel: kein Kampagnensieg");
    let mut empty = Island {
        endboss_defeated: true,
        ..Island::default()
    };
    empty.options.goal = "endboss".into();
    check_victory(&mut empty);
    assert!(!empty.won, "ohne Stufen kein Sieg");
}

#[test]
fn own_planted_trees_und_all_built() {
    let mut isl = hand_goal_island("buildAll");
    assert!(!all_built(&isl), "eine Insel ohne Bauplatz gewinnt nie");
    let site = |kind: &str, state: &str| Site {
        kind: kind.into(),
        state: state.into(),
        ..Site::default()
    };
    isl.stages[0].sites = vec![site("house", "built"), site("ship", "unpaid")];
    isl.stages[1].sites = vec![site("tower", "built")];
    assert!(all_built(&isl), "das Schiff zählt nicht");
    isl.stages[1].sites.push(site("wall", "unpaid"));
    assert!(!all_built(&isl), "ein Platz offen");
    let mut nursery = site("nursery", "built");
    nursery.plots = vec![
        Plot {
            tree: Some(7),
            ..Plot::default()
        },
        Plot::default(),
        Plot {
            tree: Some(3),
            ..Plot::default()
        },
    ];
    isl.stages[0].sites.push(nursery);
    let trees: Vec<i64> = planted_trees(&isl.stages[0]).into_iter().collect();
    assert_eq!(trees, [3, 7], "Plätze ohne Baum zählen nicht");
}

#[test]
fn own_all_mined_ohne_knoten_gewinnt_nie() {
    let mut isl = hand_goal_island("mineAll");
    assert!(!all_mined(&mut isl), "nie ein endliches Objekt: nie");
    assert!(!isl.had_finite);
    isl.had_finite = true; // ein Objekt stand einmal
    assert!(all_mined(&mut isl), "danach alles abgebaut");
}
