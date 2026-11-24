//! Spielstand nach dem Sieg (RU62.12, AC-06): Frieden und Fest sind aus `endboss_defeated` abgeleitet und überstehen Speichern
//! und Laden ohne neue Spielstand-Version und ohne zweites `festivalStart`.
use super::testutil::{count, enemies_total, festival_data, festival_island, put_enemy, run, tick};
use crate::island::testutil::ISLAND_SPEED;
use crate::island::{ISLAND_SAVE_VERSION, Island, SaveTables, WorldDef, from_island_save_in, parse_island_save};
use crate::state::links::values_of;
use k3c_data::Data;

fn reload(data: &Data, world: &WorldDef, isl: &Island) -> Island {
    let json = serde_json::to_string(&isl.to_save("2026-11-19T00:00:00Z").expect("Spielstand")).expect("json");
    let defaults = values_of(world).expect("Wertesatz");
    let t = SaveTables {
        data,
        defaults: &defaults,
        standard_world: world,
    };
    let s = parse_island_save(&json, &t).expect("lesen");
    from_island_save_in(&t, &s, ISLAND_SPEED, world).expect("laden")
}

/// AC-06: Speichern und Laden nach dem Sieg erhält Frieden und Fest (ohne neue Version, ohne neues Feld, ohne zweites
/// `festivalStart`); lebende Gegner fliehen auch nach dem Laden, und es kommen keine neuen.
#[test]
fn frieden_und_fest_ueberstehen_speichern_und_laden() {
    let data = festival_data();
    let (world, mut isl) = festival_island(&data, 2);
    run(&mut isl, &data, 1.0);
    let boss = isl.world.islands[0].end_boss_id(&data.bosses);
    isl.defeated_bosses.push(boss);
    isl.endboss_defeated = true;
    let ev = tick(&mut isl, &data);
    assert_eq!(count(&ev, "festivalStart"), 1);
    let s = isl.to_save("x").expect("Spielstand");
    assert_eq!(s.version, ISLAND_SAVE_VERSION);
    assert_eq!(ISLAND_SAVE_VERSION, 13, "keine eigene Spielstand-Version für das Fest");
    let json = serde_json::to_string(&s).expect("json");
    assert!(
        !json.contains("festival") && !json.contains("peace"),
        "kein Fest-Feld im Spielstand"
    );

    let mut loaded = reload(&data, &world, &isl);
    assert!(loaded.endboss_defeated, "Sieg bleibt");
    let hub = loaded.stages[0].hub_x;
    put_enemy(&mut loaded, 0, "goblin", hub + 20.0);
    let ev = run(&mut loaded, &data, 3.0);
    assert_eq!(
        count(&ev, "festivalStart"),
        0,
        "nach dem Laden kein zweites festivalStart"
    );
    assert!(loaded.festival_active());
    assert!(
        loaded
            .stages
            .iter()
            .all(|w| w.peace && w.festival.as_ref().is_some_and(|f| f.effect == "lanterns"))
    );
    let ev = run(&mut loaded, &data, 120.0);
    assert_eq!(count(&ev, "wave"), 0);
    assert_eq!(enemies_total(&loaded), 0, "der Gegner floh und ist verschwunden");
    assert!(loaded.stages.iter().all(|w| w.spawn_queue.is_empty()));
}
