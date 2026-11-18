//! Hilfen der Tests der Gruppe `island`. Die Hilfen aus den Go-Testdateien dieser Session (`mustIsland`, `newIsland`,
//! `runIsland`, `count`, `islandJSON`, `neutralNature`, `defeatIsland`, `builtSites` …) stehen hier. Go-Hilfen aus anderen
//! Dateien (`CreateWorld`, `Step`, `ParseWorld`, `SetOptions`, `buildSite` …) kommen mit RU14 und RU10.2/RU10.5: Ihre Rümpfe
//! melden den liefernden Sprint, Tests, die sie rufen, tragen `ignore`.
#![allow(clippy::unimplemented, clippy::panic)] // Rümpfe der Test-Hilfen: warten auf den genannten Sprint

use std::collections::BTreeMap;

use k3c_data::biome::Biome;
use k3c_data::hub::Stock;
use k3c_data::{Data, Source, load_all};
use k3c_level::nature::neutral_nature as level_neutral_nature;
use k3c_level::rules::Rules;
use k3c_level::types::NatureValues;

use super::{
    CreateOptions, Island, SaveTables, WorldDef, create_island, from_island_save, parse_island_save, step_island,
};
use crate::state::pending::values_of;
use crate::state::{Event, IslandRef, PlayerCommand, World};

/// Go: `islandSpeed`: schneller Zyklus, Tag, Dämmerung und Nacht in Sekunden statt Minuten.
pub const ISLAND_SPEED: f64 = 60.0;

/// Go: `dt` der Tests.
pub const DT: f64 = 1.0 / 30.0;

/// Die eingebetteten Spiel-Daten (Go: die Paket-Variablen).
pub fn test_data() -> Data {
    load_all(&Source::Embedded).expect("eingebettete Daten")
}

/// Go: `neutralNature`, die neutralen Natur-Werte je Tiefe (`level.NeutralNature`).
pub fn neutral_nature(biomes: &[Biome], depths: &[i64]) -> BTreeMap<i64, NatureValues> {
    let mut out = BTreeMap::new();
    for &d in depths {
        if let Some(b) = super::data::biome_for_depth(biomes, d) {
            out.insert(d, level_neutral_nature(b));
        }
    }
    out
}

/// Die Regeln des Level-Generators aus den eingebetteten Daten (Go: die Paket-Variablen, `Rules::from_data`).
pub fn test_rules(data: &Data) -> Rules {
    let hub = Source::Embedded.read("hub.json").expect("hub.json");
    Rules::from_data(data, &hub).expect("Regeln")
}

/// Go: `standardWorld` (island_worlds.go): die Standard-Welt als `WorldDef`. Sie hat eine Geschichte (`story`), deren
/// Lesen und Prüfen mit RU19 kommt (`decode_world_story`, `check_story`): bis dahin melden die Platzhalter dort.
pub fn standard_world(data: &Data) -> WorldDef {
    super::worlds::standard_world(data, &test_rules(data)).expect("Standard-Welt")
}

/// Go: `parseTestWorld` (island_worlds_test.go): liest eine Test-Welt mit einer Insel „a“; `fields` wie bei `test_island`.
pub fn parse_test_world(fields: &str) -> Result<WorldDef, String> {
    use super::switch_testutil::{test_island, test_world};
    parse_world_in(&test_world(&test_island("a", fields)), "test")
}

/// Go: `ParseWorld` (island_worlds.go) mit den eingebetteten Daten.
pub fn parse_world_in(raw: &str, id: &str) -> Result<WorldDef, String> {
    let data = test_data();
    super::worlds::parse_world(raw, id, &data, &test_rules(&data))
}

/// Go: `newIsland`: `create_island` mit Startvorrat, wie ein neues Spiel, mit neutraler Natur (B-891): Die Tests prüfen
/// Mechanik, nicht die gewürfelte Dichte des Seeds.
pub fn new_island(data: &Data, seed: &str, depths: &[i64]) -> Island {
    let opts = CreateOptions {
        nature: Some(neutral_nature(&data.biomes, depths)),
        ..CreateOptions::default()
    };
    match create_island(data, &standard_world(data), seed, depths, ISLAND_SPEED, opts) {
        Ok(isl) => isl,
        Err(e) => panic!("{e}"),
    }
}

/// Go: `mustIsland`: wie `new_island`, mit leerem Vorrat; den Startvorrat prüft `start_tests.rs`.
pub fn must_island(data: &Data, seed: &str, depths: &[i64]) -> Island {
    let mut isl = new_island(data, seed, depths);
    isl.stock = Stock::default();
    isl
}

/// Go: `runIsland` tickt die Insel `seconds` lang und liefert alle Ereignisse je Stufe.
pub fn run_island(isl: &mut Island, data: &Data, cmds: &[PlayerCommand], seconds: f64) -> Vec<Vec<Event>> {
    let mut events: Vec<Vec<Event>> = vec![Vec::new(); isl.stages.len()];
    let mut tm = 0.0;
    while tm < seconds {
        step_island(isl, cmds, &data.difficulty, DT);
        for (i, w) in isl.stages.iter().enumerate() {
            events[i].extend(w.events.iter().cloned());
        }
        tm += DT;
    }
    events
}

/// Go: `count`, die Zahl der Ereignisse einer Art.
pub fn count(events: &[Event], typ: &str) -> usize {
    events
        .iter()
        .filter(|e| e.get("type").and_then(|t| t.as_str()) == Some(typ))
        .count()
}

/// Go: `islandJSON`, der Zustand jeder Stufe als JSON-Text.
pub fn island_json(isl: &Island) -> Vec<String> {
    isl.stages
        .iter()
        .map(|w| serde_json::to_string(w).expect("Stufe als JSON"))
        .collect()
}

/// Go: `islandState`, alle Stufen als ein Text.
pub fn island_state(isl: &Island) -> String {
    island_json(isl).join("\n")
}

/// Go: `builtSites`, die Zahl der gebauten Plätze der Stufe.
pub fn built_sites(w: &World) -> usize {
    w.sites.iter().filter(|s| s.state == "built").count()
}

/// Go: `vagrants`, die Zahl der Landstreicher der Stufe.
pub fn vagrants(w: &World) -> usize {
    w.troops.iter().filter(|t| t.kind == "vagrant").count()
}

/// Go: `defeatIsland` (island_defeat_test.go): Insel mit Wald und Höhle, je ein Spieler (Gold 40 bzw. 30), Vorrat 20 Holz
/// und 10 Stein, im Wald ein gebauter Turm und ein Bogenschütze.
pub fn defeat_island(data: &Data, defeat: &str) -> Island {
    let mut isl = must_island(data, "niederlage", &[0, 1]);
    let o = super::options::IslandOptions {
        grade: "normal".into(),
        goal: "endboss".into(),
        defeat: defeat.into(),
    };
    super::options::set_options(&mut isl, o, false).expect("Optionen");
    for (stage, gold) in [(0, 40), (1, 30)] {
        let pi = super::add_island_player(&mut isl, stage);
        isl.stages[stage].players[pi].gold = gold;
    }
    isl.stock = Stock {
        wood: 20,
        stone: 10,
        ..Stock::default()
    };
    let w = &mut isl.stages[0];
    crate::enemies::testutil::build_site(w, "tower");
    let (id, x) = (w.new_id(), w.hub_x);
    w.troops.push(crate::state::Troop {
        id,
        kind: "archer".into(),
        x,
        hp: 10.0,
        max_hp: 10.0,
        ..crate::state::Troop::default()
    });
    isl
}

/// Go: `saveJSON`: der Spielstand der Insel als JSON-Text.
pub fn save_json(isl: &Island, saved_at: &str) -> String {
    serde_json::to_string(&isl.to_save(saved_at).expect("Spielstand")).expect("Spielstand als JSON")
}

/// Go: `loadIsland`: liest einen Spielstand (`parse_island_save`) und baut die Insel (`from_island_save`). Braucht
/// `standard_world` (RU10.5) und `create_world_with` (RU14).
pub fn load_island(data: &Data, raw: &str) -> Island {
    let world = standard_world(data);
    let defaults = values_of(&world).expect("Basis-Wertesatz");
    let t = SaveTables {
        data,
        defaults: &defaults,
        standard_world: &world,
    };
    let s = parse_island_save(raw, &t).expect("Spielstand lesen");
    from_island_save(&t, &s, ISLAND_SPEED).expect("Insel laden")
}

/// Go: `storageSite` (island_storage_test.go): Index des Lager-Bauplatzes der Stufe.
pub fn storage_site(w: &World) -> Option<usize> {
    w.sites.iter().position(|s| s.kind == "storage")
}

/// Ruft `f` auf der Stufe `i` wie `step_island` (Entscheidung 4 und 5): der Vorrat der Insel liegt per `swap` in der Stufe,
/// der `IslandRef` kommt aus der Insel; danach geht beides zurück.
pub fn on_stage<R>(isl: &mut Island, data: &Data, i: usize, f: impl FnOnce(&mut World, &IslandRef) -> R) -> R {
    let r = isl.stage_ref(i, &data.difficulty);
    std::mem::swap(&mut isl.stock, &mut isl.stages[i].stock);
    let out = f(&mut isl.stages[i], &r);
    std::mem::swap(&mut isl.stock, &mut isl.stages[i].stock);
    isl.put_stage_ref(r);
    out
}

/// Go: `mustLearn` (monarch_test.go): lässt Spieler `pi` die Skills lernen; kommt mit RU18 (Monarch).
pub fn must_learn(_w: &mut World, _pi: usize, _ids: &[&str]) {
    unimplemented!("RU18: must_learn")
}

/// Go: `addPoolPoints` (monarch_test.go): legt Punkte in den Fund-Pool; kommt mit RU18 (Monarch).
pub fn add_pool_points(_w: &mut World, _n: i64) {
    unimplemented!("RU18: add_pool_points")
}

/// Go: `AvailablePoints` (monarch.go): freie Skill-Punkte des Spielers `pi`; kommt mit RU18 (Monarch).
pub fn available_points(_w: &World, _pi: usize) -> i64 {
    unimplemented!("RU18: available_points")
}

/// Go: `nordmeer` (island_save_v8_test.go): die Test-Welt mit zwei Inseln, aus dem Text gelesen (nicht aus data/worlds/).
pub fn nordmeer_loaded(data: &Data) -> WorldDef {
    let raw = concat!(
        r#"{"id":"nordmeer","name":"Nordmeer","islands":["#,
        r#"{"id":"island1","name":"Insel 1","depths":[0,4],"depthScaling":{"hp":1.5,"damage":1.3,"speed":1.1},"plannedChunks":{"0":[{"slot":-7,"kind":"harbor"}]}},"#,
        r#"{"id":"eisinsel","name":"Eisinsel","depths":[0,4],"depthScaling":{"hp":1.6,"damage":1.4,"speed":1.15}}]}"#
    );
    super::worlds::parse_world(raw, "nordmeer", data, &test_rules(data)).expect("Welt nordmeer")
}
