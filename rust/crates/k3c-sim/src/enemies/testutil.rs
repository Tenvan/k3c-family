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

// Hilfen aus anderen Go-Dateien, die `CreateWorld`/`Step` (RU14) oder Inseln (RU10) brauchen.

pub fn quiet_world() -> World {
    panic!("RU14: quiet_world")
}

pub fn day_world() -> World {
    panic!("RU14: day_world")
}

pub fn add_player(_w: &mut World) -> usize {
    panic!("RU14: add_player")
}

pub fn step(_w: &mut World, _cmds: Option<&[PlayerCommand]>, _dt: f64) {
    panic!("RU14: step")
}

pub fn step_for(_w: &mut World, _seconds: f64) {
    panic!("RU14: step_for")
}

pub fn warrior_at(_w: &mut World, _x: f64) -> usize {
    panic!("RU14: warrior_at")
}

pub fn build_site(_w: &mut World, _kind: &str) -> usize {
    panic!("RU14: build_site")
}

pub fn line_site(_w: &mut World, _kind: &str, _side: i64, _n: i64) -> usize {
    panic!("RU14: line_site")
}

pub fn probe(_w: &mut World, _players: &[usize]) {
    panic!("RU14: probe")
}

pub fn was_hit(_w: &World, _pi: usize) -> bool {
    panic!("RU14: was_hit")
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
