//! Hilfen der Tests dieser Gruppe. Die Hilfen aus den Go-Testdateien (`guardPlan`, `guardsOf`, `guardSteps`) stehen
//! hier; Go-Hilfen aus anderen Dateien (`quietWorld`, `Step`, `CreateIsland` …) brauchen `create_world`/`step`
//! (RU14) und `Island` (RU10): ihre Rümpfe melden den liefernden Sprint, Tests, die sie rufen, tragen `ignore`.

use std::sync::Arc;

use k3c_data::{Source, load_all};
use k3c_level::types::{PlannedChunk, PlannedEnemy};

use crate::state::{PlayerCommand, Site, World, WorldValues};

/// Wertesatz aus den eingebetteten Daten (Go: `w.values()` ohne eigene Werte).
pub fn test_values() -> WorldValues {
    let d = load_all(&Source::Embedded).expect("eingebettete Daten");
    WorldValues {
        buildings: d.buildings,
        troops: d.troops,
        economy: d.economy,
        hub: d.hub,
        monarch: d.monarch,
        enemies: d.enemies,
        waves: d.waves,
        goals: d.goals,
        ..Default::default()
    }
}

/// Eine leere Welt mit eigenem Wertesatz: Hub bei 200, Burg dort, IDs ab 1.
pub fn bare_world() -> World {
    let mut w = World {
        hub_x: 200.0,
        next_id: 1,
        vals: Some(Arc::new(test_values())),
        ..World::default()
    };
    w.castle.id = w.new_id();
    w.castle.x = w.hub_x;
    w.castle.hp = 100.0;
    w.castle.max_hp = 100.0;
    w
}

/// Ein gebautes Gebäude (oder Tor) an `x`.
pub fn built_site(w: &mut World, kind: &str, x: f64) -> usize {
    let id = w.new_id();
    w.sites.push(Site {
        id,
        kind: kind.into(),
        x,
        state: "built".into(),
        hp: 50.0,
        max_hp: 50.0,
        ..Site::default()
    });
    w.sites.len() - 1
}

/// Ein Gegner nach Art, über den Wertesatz der Welt gebaut.
pub fn spawn(w: &mut World, kind: &str, x: f64) -> usize {
    let v = w.vals.clone().expect("Wertesatz");
    crate::enemies::spawn_with(w, &v, kind, x, 1.0, 1.0, None)
}

/// Go: `guardPlan`: Wald, Platz +4 (Hub + 200 + 250), Wachen bei +20 (Goblin) und +40 (Greed, Gold-Dieb).
pub fn guard_plan() -> Vec<PlannedChunk> {
    vec![PlannedChunk {
        slot: 4,
        kind: "forest".into(),
        enemies: vec![
            PlannedEnemy {
                kind: "goblin".into(),
                at: 20.0,
            },
            PlannedEnemy {
                kind: "greed".into(),
                at: 40.0,
            },
        ],
        ..PlannedChunk::default()
    }]
}

/// Go: `guardsOf`: die Indizes der Wachen.
pub fn guards_of(w: &World) -> Vec<usize> {
    (0..w.enemies.len()).filter(|&i| w.enemies[i].guard).collect()
}

/// Go: `guardSteps`: so viele Takte, wie `seconds` Sekunden ergeben.
pub fn guard_steps(w: &mut World, seconds: f64) {
    let mut tm = 0.0;
    while tm < seconds {
        step(w, None, DT);
        tm += DT;
    }
}

/// Go: `dt` der Tests.
pub const DT: f64 = 1.0 / 30.0;

/// Die Anzahl der Ereignisse einer Art.
pub fn count_events(w: &World, typ: &str) -> usize {
    super::events_of_type(w, typ).len()
}

/// Eine Insel aus Stufen (Platzhalter für Gos `Island`, kommt mit RU10).
pub struct Island {
    pub stages: Vec<World>,
}

// Hilfen aus anderen Go-Dateien, die `CreateWorld`/`Step` oder Inseln (RU10) brauchen.

/// Go: `quietWorld` (rules_test.go); seit RU14.1 über `create_world`, die Teilschritte späterer Sprints fehlen noch.
pub fn quiet_world() -> World {
    crate::world::testutil::quiet_world()
}

/// Go: `dayWorld` (hub_level_test.go).
pub fn day_world() -> World {
    crate::world::testutil::day_world()
}

/// Go: `AddPlayer`.
pub fn add_player(w: &mut World) -> usize {
    crate::world::add_player(w)
}

/// Go: `Step(w, cmds, dt)` ohne Insel; `None` = keine Eingaben.
pub fn step(w: &mut World, cmds: Option<&[PlayerCommand]>, dt: f64) {
    crate::world::step(w, cmds.unwrap_or(&[]), None, dt);
}

/// Go: `stepFor` (enemies_traits_test.go): nur `stepEnemies`, `seconds` lang.
pub fn step_for(w: &mut World, seconds: f64) {
    let dt = crate::world::testutil::DT;
    for _ in 0..(seconds / dt).round() as i64 {
        super::step_enemies(w, None, dt);
        w.time += dt;
    }
}

/// Go: `warriorAt` (enemies_traits_test.go): ein Krieger mit 1000 HP an `x`.
pub fn warrior_at(w: &mut World, x: f64) -> usize {
    let id = w.new_id();
    w.troops.push(crate::state::Troop {
        id,
        kind: "warrior".into(),
        x,
        hp: 1000.0,
        max_hp: 1000.0,
        ..crate::state::Troop::default()
    });
    w.troops.len() - 1
}

/// Go: `buildSite` (rules_test.go): der erste Platz `kind` ist gebaut, mit voller HP (Index in `w.sites`).
pub fn build_site(w: &mut World, kind: &str) -> usize {
    let Some(si) = w.sites.iter().position(|s| s.kind == kind) else {
        panic!("kein Bauplatz {kind}");
    };
    let s = &mut w.sites[si];
    s.state = "built".into();
    s.hp = s.max_hp;
    s.build_progress = 1.0;
    si
}

/// Go: `lineSite` (world_lines_test.go): der erste Platz `kind` der Zone `n` auf Seite `side` (Index in `w.sites`).
pub fn line_site(w: &mut World, kind: &str, side: i64, n: i64) -> usize {
    let found = w.sites.iter().position(|s| {
        s.kind == kind && crate::world::lines::level_site(w, s).is_some_and(|l| l.side == side && l.zone == n)
    });
    match found {
        Some(si) => si,
        None => panic!("kein Platz {kind} Linie {n} Seite {side}"),
    }
}

/// Go: `probeGold` (crown_helpers_test.go).
pub const PROBE_GOLD: i64 = 20;

/// Go: `probe` (crown_helpers_test.go): gibt den Spielern `players` genau `PROBE_GOLD`.
pub fn probe(w: &mut World, players: &[usize]) {
    for &pi in players {
        w.players[pi].gold = PROBE_GOLD;
    }
}

/// Go: `wasHit` (crown_helpers_test.go): Der Monarch wurde seit `probe` getroffen (Gold weg oder Krone gefallen).
pub fn was_hit(w: &World, pi: usize) -> bool {
    let p = &w.players[pi];
    p.gold < PROBE_GOLD || p.crown.is_some()
}

pub fn ruin_island(_plan: Vec<PlannedChunk>) -> Island {
    panic!("RU10: ruin_island")
}

pub fn island_with(_grade: &str, _players: i64) -> Island {
    panic!("RU10: island_with")
}

pub fn remove_dead_enemies(w: &mut World) {
    super::waves::remove_dead_enemies(w, None)
}

// Hilfen für die Tests der Truppen (`unit/`), aus anderen Go-Testdateien.

/// Go: `run` (rules_test.go): rechnet ohne Eingaben, bis `seconds` um sind.
pub fn run(w: &mut World, seconds: f64) {
    let dt = crate::world::testutil::DT;
    let mut t = 0.0;
    while t < seconds {
        step(w, None, dt);
        t += dt;
    }
}

/// Go: `runUntil` (rules_test.go), siehe `world::testutil::run_until`.
pub fn run_until(w: &mut World, max_seconds: f64, done: &dyn Fn(&World) -> bool) -> f64 {
    crate::world::testutil::run_until(w, max_seconds, done)
}

/// Go: `addPeasant` (unit_professions_test.go): legt einen Bauern mit Beruf `profession` bei `x` an.
pub fn add_peasant(w: &mut World, x: f64, profession: &str) -> usize {
    let hp = w
        .vals
        .as_ref()
        .and_then(|v| v.troops.get("peasant"))
        .map_or(0.0, |t| t.hp);
    let id = w.new_id();
    let anchor_x = w.hub_x;
    w.troops.push(crate::state::Troop {
        id,
        kind: "peasant".into(),
        x,
        hp,
        max_hp: hp,
        anchor_x,
        profession: profession.into(),
        ..Default::default()
    });
    w.troops.len() - 1
}

/// Go: `loseLayer` (unit_warrior.go) mit dem Wertesatz der Welt.
pub fn lose_layer(w: &mut World, ti: usize) {
    if let Some(v) = w.vals.clone() {
        crate::unit::warrior::lose_layer(w, &v, ti);
    }
}

/// Go: `payAt` (rules_test.go), siehe `world::testutil::pay_at`.
pub fn pay_at(w: &mut World, players: &[usize], x: f64, seconds: f64, done: &dyn Fn(&World) -> bool) -> bool {
    crate::world::testutil::pay_at(w, players, x, seconds, done)
}

/// Go: `firstNode` (mine_test.go): der erste Knoten `kind` (Index in `w.nodes`).
pub fn first_node(w: &World, kind: &str) -> usize {
    match w.nodes.iter().position(|n| n.kind == kind) {
        Some(ni) => ni,
        None => panic!("kein {kind} in der Mine"),
    }
}

pub fn end_island(_players: i64) -> (Island, f64) {
    panic!("RU15: end_island")
}
