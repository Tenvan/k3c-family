//! Hilfen der Tests dieser Gruppe (Go: `quietWorld`, `dayWorld`, `payAt`, `runUntil`, `clearNodes` aus `rules_test.go`,
//! `hub_level_test.go`, `world_lines_test.go`). Tests, die `create_world` oder `step` brauchen, warten auf die Teilschritte
//! späterer Sprints (`pending.rs`) und tragen `ignore`; die Gerüste ohne `create_world` (`built_wall_world`) laufen schon.

pub use crate::enemies::testutil::{bare_world, built_site};

use crate::campaign::new_world as campaign_new_world;
use crate::state::{PlayerCommand, World};
use crate::unit::spawn_vagrant;

use super::{Options, step, vals};

/// Go: `dt`, ein Tick der Tests.
pub const DT: f64 = 1.0 / 30.0;

/// Go: `newWorld`: eine Stufe der Kampagne (neutrale Natur); bricht bei einem Fehler ab wie Gos `panic`.
pub fn new_world(depth: i64, seed: &str, opts: Options) -> World {
    campaign_new_world(depth, seed, opts).expect("Welt")
}

/// Go: `quietWorld`: eine Welt ohne Truppen, Camps und Truhen.
pub fn quiet_world() -> World {
    let mut w = new_world(0, "test", Options::default());
    w.troops.clear();
    w.camps.clear();
    w.pickups.clear();
    w
}

/// Go: `dayWorld`: eine ruhige Welt, in der der Tag nicht endet, mit einem Bauern an der Burg.
pub fn day_world() -> World {
    let mut w = quiet_world();
    w.cycle_speed = 1e-6;
    let hub = w.hub_x;
    let ti = spawn_vagrant(&mut w, hub, hub);
    let hp = vals(&w).troops.get("peasant").map_or(0.0, |t| t.hp);
    let t = &mut w.troops[ti];
    t.kind = "peasant".into();
    t.hp = hp;
    t.max_hp = hp;
    t.hammer = true;
    w
}

/// Go: `clearNodes`: keine Bäume und Felsen.
pub fn clear_nodes(w: &mut World) {
    w.nodes.clear();
}

/// Go: `payAt`: lässt die Spieler `players` (Plätze) an `x` zahlen, bis `done` gilt oder `seconds` um sind; `false` bei
/// Zeitablauf.
pub fn pay_at(w: &mut World, players: &[usize], x: f64, seconds: f64, done: impl Fn(&World) -> bool) -> bool {
    let mut cmds = vec![PlayerCommand::default(); w.players.len()];
    for &pi in players {
        cmds[w.players[pi].index as usize].pay = true;
    }
    let mut tm = 0.0;
    while tm < seconds {
        for &pi in players {
            w.players[pi].x = x;
        }
        step(w, &cmds, None, DT);
        if done(w) {
            return true;
        }
        tm += DT;
    }
    false
}

/// Go: `runUntil`: rechnet ohne Eingaben, bis `done` gilt; die Rückgabe sind die simulierten Sekunden, -1 bei Zeitablauf.
pub fn run_until(w: &mut World, seconds: f64, done: impl Fn(&World) -> bool) -> f64 {
    let mut tm = 0.0;
    while tm < seconds {
        step(w, &[], None, DT);
        if done(w) {
            return tm + DT;
        }
        tm += DT;
    }
    -1.0
}

/// Eine Stufe von Hand mit einer gebauten Mauer (kein `create_world`): Wertesatz, Hub und Burg wie `bare_world`. Liefert
/// Welt und Platz der Mauer.
pub fn built_wall_world() -> (World, usize) {
    let mut w = bare_world();
    let x = w.hub_x - 20.0;
    let si = built_site(&mut w, "wall", x);
    (w, si)
}
