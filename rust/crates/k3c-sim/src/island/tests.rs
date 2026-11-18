//! Insel-Tests (Go: `island_test.go`, SP12.1): mehrere Stufen ticken gemeinsam, ein Vorrat je Insel, deterministisch.
//! Alle sechs Go-Tests bauen eine Insel (`create_island`, `step_island`) und warten auf RU14 (`create_world_with`, `step`).
//! Die Tests darunter (ohne Go-Gegenstück) prüfen, was schon läuft: die Fehlerwege der Erzeugung, `stage_ref` und
//! `put_stage_ref`.

use std::sync::Arc;

use k3c_data::hub::Stock;

use super::testutil::*;
use super::{
    CreateOptions, Island, IslandBuild, IslandDef, WorldDef, add_island_player, create_island_at, create_island_in,
};
use crate::boss::endboss::EndbossFight;
use crate::state::{Player, PlayerCommand, World, WorldValues};

/// Die Stufe `i` rechnet mit dem Vorrat der Insel (Zustandsmodell, Entscheidung 5): hinein, ändern, zurück.
fn with_stage_stock(isl: &mut Island, i: usize, f: impl FnOnce(&mut World)) {
    std::mem::swap(&mut isl.stock, &mut isl.stages[i].stock);
    f(&mut isl.stages[i]);
    std::mem::swap(&mut isl.stock, &mut isl.stages[i].stock);
}

#[test]
#[ignore = "wartet auf RU19 (decode_world_story)"]
fn island_stages_tick_together() {
    let data = test_data();
    let mut isl = must_island(&data, "insel-a", &[0, 1]);
    add_island_player(&mut isl, 0);
    add_island_player(&mut isl, 1);
    let events = run_island(
        &mut isl,
        &data,
        &[PlayerCommand::default(), PlayerCommand::default()],
        30.0,
    );

    assert!(
        isl.stages[0].time == isl.stages[1].time && isl.stages[0].time >= 29.0,
        "Zeit der Stufen weicht ab: {} und {}",
        isl.stages[0].time,
        isl.stages[1].time
    );
    assert!(
        count(&events[0], "night") > 0 && count(&events[0], "wave") > 0,
        "Wald: Nacht und Welle erwartet, Ereignisse: night={} wave={}",
        count(&events[0], "night"),
        count(&events[0], "wave")
    );
    assert!(isl.stages[0].aggression.is_none(), "Wald hat keinen Aggressionspool");
    let pool = isl.stages[1].aggression;
    assert!(
        pool.is_some_and(|a| !(a == 0.0 && count(&events[1], "wave") == 0)),
        "Höhle: Aggressionspool muss laufen (Anstieg oder Welle)"
    );
    assert!(
        isl.stages[0].players.len() == 1 && isl.stages[1].players.len() == 1,
        "je Stufe ein Spieler erwartet: {} und {}",
        isl.stages[0].players.len(),
        isl.stages[1].players.len()
    );
}

#[test]
#[ignore = "wartet auf RU19 (decode_world_story)"]
fn island_stage_without_player_keeps_running() {
    let data = test_data();
    let mut isl = must_island(&data, "insel-b", &[0, 1]);
    add_island_player(&mut isl, 0);
    let events = run_island(&mut isl, &data, &[PlayerCommand::default()], 20.0);
    assert!(
        isl.stages[1].time >= 19.0,
        "Stufe ohne Spieler steht still: {}",
        isl.stages[1].time
    );
    let pool = isl.stages[1].aggression.unwrap_or(0.0) as usize;
    assert!(
        count(&events[1], "wave") + pool > 0,
        "Stufe ohne Spieler: Aggressionspool muss weiterlaufen"
    );
}

/// Go: `TestIslandSharedStock`. In Rust besitzt die Insel den Vorrat (Entscheidung 5); jede Stufe sieht ihn, solange sie
/// rechnet (`with_stage_stock`), und keine Stufe hat einen eigenen.
#[test]
#[ignore = "wartet auf RU19 (decode_world_story)"]
fn island_shared_stock() {
    let data = test_data();
    let mut isl = must_island(&data, "insel-c", &[0, 1, 2]);
    for (i, w) in isl.stages.iter().enumerate() {
        assert_eq!(w.stock, Stock::default(), "Stufe {i} hat einen eigenen Vorrat");
    }
    with_stage_stock(&mut isl, 0, |w| w.stock.wood += 30); // Go: addStock(Stages[0], "wood", 30)
    with_stage_stock(&mut isl, 2, |w| w.stock.stone += 12);
    with_stage_stock(&mut isl, 1, |w| {
        assert!(
            w.stock.wood == 30 && w.stock.stone == 12,
            "Vorrat nicht gemeinsam: {:?}",
            w.stock
        );
        w.stock.wood -= 10; // Go: spend(Stages[1].Stock, Cost{Wood: 10})
    });
    with_stage_stock(&mut isl, 0, |w| {
        assert_eq!(w.stock.wood, 20, "Ausgabe in Stufe 1 muss Stufe 0 sehen");
    });
    // Gold gehört dem Spieler, nicht der Insel.
    let a = add_island_player(&mut isl, 0);
    let b = add_island_player(&mut isl, 1);
    isl.stages[0].players[a].gold = 7;
    assert_eq!(
        isl.stages[1].players[b].gold, data.economy.purse.start_gold,
        "Gold ist nicht je Spieler"
    );
}

#[test]
#[ignore = "wartet auf RU19 (decode_world_story)"]
fn island_player_index_is_unique() {
    let data = test_data();
    let mut isl = must_island(&data, "insel-d", &[0, 1, 2]);
    let mut seen = std::collections::BTreeSet::new();
    for (i, st) in [0usize, 1, 0, 2].into_iter().enumerate() {
        let pi = add_island_player(&mut isl, st);
        let idx = isl.stages[st].players[pi].index;
        assert!(idx == i as i64 && seen.insert(idx), "Spieler {i} hat Index {idx}");
        assert_eq!(pi, isl.stages[st].players.len() - 1, "Spieler {i} nicht in Stufe {st}");
    }
}

#[test]
#[ignore = "wartet auf RU19 (decode_world_story)"]
fn island_deterministic() {
    let data = test_data();
    let run = |seed: &str| -> Vec<String> {
        let mut isl = must_island(&data, seed, &[0, 1, 2]);
        for st in [0usize, 1, 2, 0] {
            add_island_player(&mut isl, st);
        }
        let cmds = [
            PlayerCommand {
                move_x: 1.0,
                sprint: true,
                ..Default::default()
            },
            PlayerCommand {
                move_x: -1.0,
                ..Default::default()
            },
            PlayerCommand {
                pay: true,
                ..Default::default()
            },
            PlayerCommand::default(),
        ];
        run_island(&mut isl, &data, &cmds, 40.0);
        island_json(&isl)
    };
    let (a, b) = (run("insel-e"), run("insel-e"));
    for i in 0..a.len() {
        assert_eq!(
            a[i], b[i],
            "Stufe {i}: gleicher Seed und gleiche Eingaben ergeben verschiedene Zustände"
        );
    }
    assert_ne!(
        run("insel-f")[0],
        a[0],
        "anderer Seed muss einen anderen Zustand ergeben"
    );
}

#[test]
#[ignore = "wartet auf RU19 (decode_world_story)"]
fn create_island_rejects_bad_input() {
    let data = test_data();
    let world = standard_world(&data);
    assert!(
        create_island_in(&data, &world, "x", &[], 1.0, CreateOptions::default()).is_err(),
        "ohne Stufen muss ein Fehler kommen"
    );
    assert!(
        create_island_in(&data, &world, "x", &[0, 9], 1.0, CreateOptions::default()).is_err(),
        "unbekannte Tiefe muss ein Fehler sein"
    );
}

// Ab hier: Tests ohne Go-Gegenstück, sie laufen schon.

/// Eine Welt mit einer Insel, ohne die Platzhalter zu brauchen.
fn mini_world(islands: usize) -> WorldDef {
    WorldDef {
        id: "test".into(),
        islands: vec![
            IslandDef {
                id: "a".into(),
                name: "Insel".into(),
                ..IslandDef::default()
            };
            islands
        ],
        ..WorldDef::default()
    }
}

#[test]
fn create_island_ohne_stufen_oder_ohne_insel_wird_abgelehnt() {
    let data = test_data();
    let err = create_island_in(&data, &mini_world(1), "x", &[], 1.0, CreateOptions::default()).err();
    assert_eq!(err.as_deref(), Some("insel: keine Stufen"));
    let err = create_island_in(&data, &mini_world(0), "x", &[0], 1.0, CreateOptions::default()).err();
    assert_eq!(err.as_deref(), Some("insel: Welt \"test\" ohne Insel"));
    let world = mini_world(1);
    let build = IslandBuild {
        data: &data,
        world: &world,
        first: &world.islands[0],
        hub: None,
    };
    let err = create_island_at(&build, "x", &[], 1.0, 0.0, &CreateOptions::default()).err();
    assert_eq!(err.as_deref(), Some("insel: keine Stufen"));
}

/// Eine Insel mit zwei Stufen und Spielern (Gold 40 und 30), ohne `create_island`.
fn bare_island() -> Island {
    let mut isl = Island {
        stages: vec![World::default(), World::default()],
        stock: Stock {
            wood: 20,
            stone: 10,
            ..Stock::default()
        },
        ..Island::default()
    };
    isl.options.grade = "normal".into();
    isl.options.defeat = "resources".into();
    isl.stages[0].players.push(Player {
        gold: 40,
        ..Player::default()
    });
    isl.stages[1].players.push(Player {
        gold: 30,
        ..Player::default()
    });
    isl.stages[0].vals = Some(Arc::new(WorldValues::default()));
    isl
}

#[test]
fn stage_ref_und_put_stage_ref_geben_vorrat_und_bosse_unveraendert_zurueck() {
    let data = test_data();
    let mut isl = bare_island();
    isl.defeated_bosses = vec!["wald".into(), "hoehle".into()];
    isl.endboss = Some(EndbossFight::default());
    isl.endboss_defeated = true;
    isl.dev.cheap_build = true;
    isl.stages[0].vals = Some(Arc::new(WorldValues {
        waves: k3c_data::waves::WavesData {
            per_extra_player: 0.5,
            ..Default::default()
        },
        ..WorldValues::default()
    }));
    let before = isl.clone();

    let r = isl.stage_ref(1, &data.difficulty);
    assert_eq!(r.stage, 1);
    assert_eq!(r.players, 2, "Spieler aller Stufen der Insel");
    assert!(r.cheap_build && r.endboss_defeated && r.defeat_resources);
    assert_eq!(r.per_extra_player, 0.5);
    assert_eq!(r.grade, data.difficulty.grades["normal"], "Grad aus der Tabelle");
    assert_eq!(r.defeated_bosses, vec!["wald".to_string(), "hoehle".to_string()]);
    assert!(
        isl.defeated_bosses.is_empty() && isl.endboss.is_none(),
        "herausgenommen"
    );

    // Vorrat tauschen wie `step_island` (Entscheidung 5): hinein, ändern, zurück.
    std::mem::swap(&mut isl.stock, &mut isl.stages[1].stock);
    assert_eq!(isl.stages[1].stock.wood, 20);
    std::mem::swap(&mut isl.stock, &mut isl.stages[1].stock);

    isl.put_stage_ref(r);
    assert_eq!(isl, before, "Vorrat und besiegte Bosse kommen unverändert wieder an");
}

#[test]
fn put_stage_ref_uebernimmt_over_und_halbiert_gold_der_anderen_stufen() {
    let mut isl = bare_island();
    let mut r = isl.stage_ref(0, &test_data().difficulty);
    super::defeat::resource_losses_in_stage(&mut isl.stages[0], &mut r);
    r.over = true;
    assert_eq!(
        isl.stages[1].players[0].gold, 30,
        "die andere Stufe erst in `put_stage_ref`"
    );
    isl.put_stage_ref(r);
    assert!(isl.over);
    assert_eq!(isl.stages[0].players[0].gold, 20);
    assert_eq!(isl.stages[1].players[0].gold, 15);
}

#[test]
fn player_count_zaehlt_die_spieler_aller_stufen() {
    // `add_island_player` braucht `add_player_at` (RU14); geprüft wird die Zählung an einer fertigen Insel.
    let isl = bare_island();
    assert_eq!(isl.next_player, 0);
    assert_eq!(isl.player_count(), 2);
}
