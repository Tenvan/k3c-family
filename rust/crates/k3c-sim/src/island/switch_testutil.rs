//! Hilfen der Tests zu Wechsel, Zielen, Reise und Sieg (Go: `testIslands`, `gateIsland`, `stepSeconds`, `etappenWelt`,
//! `targetIDs`, `nordmeerIsland`, `fell`, `built` aus `island_switch_*_test.go` und Nachbarn). Go-Hilfen aus anderen Dateien
//! (`completeShip`, `cyclePhases`) kommen mit RU14; ihre Rümpfe melden den liefernden Sprint, Tests, die sie rufen, tragen
//! `ignore`.
#![allow(clippy::unimplemented)] // Rümpfe der Test-Hilfen: warten auf den genannten Sprint

use k3c_data::Data;

use super::testutil::{DT, run_island, standard_world};
use super::{CreateOptions, Island, IslandDef, WorldDef, add_island_player, create_island_in};
use crate::state::shared::{CROWN_GROUND, Crown};
use crate::state::{Event, Site, World};

/// Go: `testIslands`: Insel 1 der Standard-Welt und dahinter eine zweite Insel (Tiefen 0 und 1, eigene Tabelle).
pub fn test_islands(data: &Data) -> (WorldDef, IslandDef) {
    let mut two = IslandDef {
        id: "test-island2".into(),
        name: "Test-Insel".into(),
        depths: vec![0, 1],
        ..IslandDef::default()
    };
    two.depth_scaling.hp = 2.0;
    two.depth_scaling.damage = 2.0;
    two.depth_scaling.speed = 1.0;
    let std = standard_world(data);
    let world = WorldDef {
        id: "test".into(),
        name: "Test".into(),
        islands: vec![std.islands[0].clone(), two.clone()],
        ..WorldDef::default()
    };
    (world, two)
}

/// Go: `completeShip` (ship_test.go): baut das Schiff der Insel fertig und liefert die X-Position des Stegs. Der Rumpf
/// ruft `dev_build_site` (RU19).
pub fn complete_ship(isl: &mut Island) -> f64 {
    use crate::world::pending::dev_build_site;
    use crate::world::ship::{SHIP_KIND, harbor};
    let Some((wi, si)) = harbor(isl) else {
        panic!("kein Hafen auf der Insel");
    };
    let w = &mut isl.stages[wi];
    let stages = crate::world::vals(w)
        .buildings
        .get(SHIP_KIND)
        .map_or(0, |b| b.levels.len() as i64);
    dev_build_site(w, si, stages);
    w.sites[si].x
}

/// Go: `gateIsland`: erste Insel der Welt `w` mit Wald und Höhle, fertiges Schiff, `n` Spieler am Steg des Hafens (Wald),
/// Endboss besiegt. Die Plätze der Spieler in `stages[0].players` sind `0..n`.
pub fn gate_island(data: &Data, w: &WorldDef, n: usize) -> Island {
    let mut isl = create_island_in(
        data,
        w,
        "wechsel",
        &[0, 1],
        super::testutil::ISLAND_SPEED,
        CreateOptions::default(),
    )
    .expect("Insel bauen");
    isl.stock = k3c_data::hub::Stock::default();
    let steg = complete_ship(&mut isl);
    for _ in 0..n {
        let pi = add_island_player(&mut isl, 0);
        isl.stages[0].players[pi].x = steg;
    }
    isl.endboss_defeated = true;
    isl
}

/// Go: `stepSeconds` rechnet die Insel so viele Sekunden und sammelt die Ereignisse aller Stufen.
pub fn step_seconds(isl: &mut Island, data: &Data, seconds: f64) -> Vec<Event> {
    run_island(isl, data, &[], seconds).into_iter().flatten().collect()
}

/// Go: `fell`: die Krone des Spielers `pi` liegt am Boden.
pub fn fell(w: &mut World, pi: usize) {
    let p = &mut w.players[pi];
    p.crown = Some(Crown {
        owner: p.index,
        state: CROWN_GROUND.to_string(),
        x: p.x,
        ..Crown::default()
    });
}

/// Go: `built` (world_lines_test.go): der Platz steht.
pub fn built(s: &mut Site) {
    s.state = "built".to_string();
    s.hp = s.max_hp;
}

/// Go: `etappenWelt`: drei Inseln mit den Etappen 1, 2, 2 (Insel 1 der Standard-Welt, dahinter „zwei“ und „drei“).
pub fn etappen_welt(data: &Data) -> (WorldDef, IslandDef, IslandDef) {
    let (mut w, mut two) = test_islands(data);
    let mut three = two.clone();
    three.id = "test-island3".into();
    three.name = "Test-Insel 3".into();
    two.etappe = 2;
    three.etappe = 2;
    w.islands = vec![w.islands[0].clone(), two.clone(), three.clone()];
    (w, two, three)
}

/// Go: `targetIDs`, die IDs der Ziele am Hafen.
pub fn target_ids(isl: &Island) -> Vec<String> {
    isl.targets().into_iter().map(|t| t.id).collect()
}

/// Go: `openGate` rechnet einen Takt, damit der Wechselpunkt offen ist (noch nicht bereit).
pub fn open_gate(isl: &mut Island, data: &Data) {
    super::step_island(isl, &[], &data.difficulty, DT);
    assert!(
        isl.gate_open && !isl.switch_ready,
        "Wechselpunkt: offen {}, bereit {}",
        isl.gate_open,
        isl.switch_ready
    );
}

/// Go: `nordmeerIsland` (island_save_v8_test.go): Insel 1 der Test-Welt mit `n` Spielern am Wechselpunkt (tiefste Stufe),
/// Endboss besiegt.
pub fn nordmeer_island(data: &Data, w: &WorldDef, n: usize) -> Island {
    let mut isl = create_island_in(
        data,
        w,
        "nordmeer",
        &[0, 4],
        super::testutil::ISLAND_SPEED,
        CreateOptions::default(),
    )
    .expect("Insel bauen");
    let steg = complete_ship(&mut isl);
    for _ in 0..n {
        let pi = add_island_player(&mut isl, 0);
        isl.stages[0].players[pi].x = steg;
    }
    isl.endboss_defeated = true;
    isl
}

/// Go: `cyclePhases(globalDayNight)` (world_cycle.go): Länge eines Tages in Sekunden.
pub fn cycle_length() -> f64 {
    crate::world::cycle::cycle_phases(&crate::world::base::base().expect("Daten").day_night).1
}

/// Go: `testIsland` (island_worlds_test.go): eine Insel `id` der Test-Welt als JSON; `fields` (beginnt mit `,`) ersetzt
/// einzelne Felder.
pub fn test_island(id: &str, fields: &str) -> String {
    let mut v: serde_json::Map<String, serde_json::Value> =
        serde_json::from_str(r#"{"name":"Insel","depths":[0,1,2,3,4],"depthScaling":{"hp":1,"damage":1,"speed":1}}"#)
            .expect("Vorgabe");
    v.insert("id".to_string(), serde_json::Value::from(id));
    if !fields.is_empty() {
        let over: serde_json::Map<String, serde_json::Value> =
            serde_json::from_str(&format!("{{{}}}", &fields[1..])).expect("Felder");
        v.extend(over);
    }
    serde_json::to_string(&v).expect("Insel als JSON")
}

/// Go: `testWorld` (island_worlds_test.go): eine Welt-Datei mit den Inseln `islands`.
pub fn test_world(islands: &str) -> String {
    format!(r#"{{"id":"test","name":"Test","islands":[{islands}]}}"#)
}

/// Go: `ParseWorld` (island_worlds.go): liest und prüft eine Welt-Datei mit den eingebetteten Daten.
pub fn parse_world(raw: &str, name: &str) -> Result<WorldDef, String> {
    super::testutil::parse_world_in(raw, name)
}

/// Go: `defaultValues()` (world_values.go): der Basis-Wertesatz; kommt mit RU14 (`values_of`).
pub fn default_values(data: &Data) -> std::sync::Arc<crate::state::WorldValues> {
    crate::state::pending::values_of(&standard_world(data)).expect("Basis-Wertesatz")
}
