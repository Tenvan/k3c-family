//! Hilfen der Tests dieser Gruppe: `combatIsland`, `spawnSoldiers`, `combatPay`, `firstLeader`, `spotFor`, `groupReady`,
//! `squadSize` und `takeBannerAt` aus den Go-Testdateien `combat_test.go`. Hilfen, die Inseln (RU10) oder die Gruppen
//! (RU15.3) brauchen, melden den liefernden Sprint; Tests, die sie rufen, tragen `ignore`.
//! `combat_stage` ersetzt `combatIsland` ohne `CreateIsland`: eine Welt mit Wertesatz, Toren und zwei Spielern.

use std::sync::Arc;

use super::spots::{CombatSpot, SpotKind, combat_spots};
use crate::enemies::testutil::{bare_world, test_values};
use crate::state::shared::CombatSettings;
use crate::state::{Player, PlayerCommand, Site, Troop, World, WorldValues};

/// Go: `combatIsland` ohne Insel: eine Welt mit der Variante `mode` (Gruppengröße 3), je einer gebauten Mauer als Tor
/// links und rechts (Hub ± 10), zwei Monarchen mit 40 Gold und zwei Portalen bei Hub ∓ 40. Liefert Welt, linkes und
/// rechtes Tor.
pub fn combat_stage(mode: &str) -> (World, f64, f64) {
    let mut v = test_values();
    v.combat = CombatSettings {
        mode: mode.to_string(),
        group_size: 3,
    };
    let mut w = bare_world();
    w.width_units = 400.0;
    w.hub_level = v.economy.combat.shield_min_hub_level;
    w.vals = Some(Arc::new(v));
    for side in [-1.0, 1.0] {
        let id = w.new_id();
        w.sites.push(Site {
            id,
            kind: "wall".into(),
            x: w.hub_x + side * 10.0,
            state: "built".into(),
            ..Site::default()
        });
    }
    for index in 0..2 {
        let id = w.new_id();
        w.players.push(Player {
            id,
            index,
            gold: 40,
            x: w.hub_x,
            ..Player::default()
        });
    }
    w.portals = vec![w.hub_x - 40.0, w.hub_x + 40.0];
    w.biome.portals.hp = 500.0; // Go: `CreateIsland` legt HP und Verteidiger-Takt der Portale an
    w.biome.portals.defense.interval_seconds = 15.0;
    w.biome.portals.defense.size = 3;
    crate::portal::sync_portals(&mut w);
    let (left, right) = (w.hub_x - 10.0, w.hub_x + 10.0);
    (w, left, right)
}

/// Der Wertesatz der Welt (kurz für Tests).
pub fn vals(w: &World) -> Arc<WorldValues> {
    w.vals.clone().expect("Wertesatz")
}

/// Go: `spotFor`: das Zahlziel der Art `kind` an der Seite `side` (leer: keines).
pub fn spot_for(w: &World, kind: SpotKind, side: i64) -> Option<CombatSpot> {
    combat_spots(w, &vals(w))
        .into_iter()
        .find(|s| s.kind == kind && s.side == side)
}

/// Eine Truppe der Art `kind` bei Hub + `dx` mit Anker bei Hub + `anchor` (ohne `spawnVagrant`/`makeFighter`).
pub fn troop(w: &mut World, kind: &str, dx: f64, anchor: f64) -> usize {
    let (x, anchor) = (w.hub_x + dx, w.hub_x + anchor);
    let id = w.new_id();
    w.troops.push(Troop {
        id,
        kind: kind.into(),
        x,
        anchor_x: anchor,
        hp: 10.0,
        max_hp: 10.0,
        ..Troop::default()
    });
    w.troops.len() - 1
}

/// Go: `spawnSoldiers`: `n` Krieger auf der rechten Seite am Tor (Anker Hub + 1).
pub fn spawn_soldiers(w: &mut World, n: usize) -> Vec<usize> {
    (0..n).map(|_| troop(w, "warrior", 0.0, 1.0)).collect()
}

/// Go: `combatPay`: Spieler `i` steht an `x` und zahlt `ticks` Takte lang.
pub fn combat_pay(w: &mut World, i: usize, x: f64, ticks: usize) {
    for _ in 0..ticks {
        w.players[i].x = x;
        w.players[i].vx = 0.0;
        let mut cmds = vec![PlayerCommand::default(); w.players.len()];
        cmds[i].pay = true;
        crate::world::step(w, &cmds, None, crate::world::testutil::DT);
    }
}

/// Go: `run` (rules_test.go): tickt die Stufe `seconds` lang.
pub fn run(w: &mut World, seconds: f64) {
    crate::enemies::testutil::run(w, seconds);
}

/// Go: `firstLeader`: der erste Gruppenführer (Index).
pub fn first_leader(w: &World) -> Option<usize> {
    w.troops.iter().position(|t| t.leader)
}

/// Go: `groupReady`: ein Führer mit voller Gruppe am rechten Tor.
pub fn group_ready() -> (World, usize) {
    let (mut w, _, _) = combat_stage("groups");
    spawn_soldiers(&mut w, 5);
    let x = spot_for(&w, SpotKind::Shield, 0).map_or(0.0, |s| s.x);
    combat_pay(&mut w, 0, x, 76);
    run(&mut w, 8.0);
    match first_leader(&w) {
        Some(l) if super::squad::group_count(&w, l) == 3 => (w, l),
        _ => panic!("Gruppe nicht bereit"),
    }
}

/// Go: `takeBannerAt`: Spieler `i` nimmt das Banner am rechten Tor.
pub fn take_banner_at(w: &mut World, i: usize) {
    let x = spot_for(w, SpotKind::Banner, 1).map_or(0.0, |s| s.x);
    combat_pay(w, i, x, 20);
}

/// Go: `squadSize`: Soldaten und Schützen im Trupp des Monarchen `pi`.
pub fn squad_size(w: &World, pi: usize) -> (usize, usize) {
    let id = w.players[pi].id;
    let mut out = (0, 0);
    for t in w.troops.iter().filter(|t| t.group == id) {
        if super::is_soldier(t) {
            out.0 += 1;
        } else {
            out.1 += 1;
        }
    }
    out
}
