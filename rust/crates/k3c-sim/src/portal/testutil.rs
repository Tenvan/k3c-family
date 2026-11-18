//! Hilfen der Tests dieser Gruppe: `portalIsland`, `attacker` und `siege` aus `portal_test.go` ohne `CreateIsland` und
//! `Step` (RU10, RU14): eine Stufe mit zwei Portalen und einem Wertesatz samt Bossen.

use std::sync::Arc;

use crate::boss::testutil::boss_values;
use crate::enemies::testutil::bare_world;
use crate::state::{Troop, World, WorldValues};

/// Die Portal-HP der Biom-Daten (data/biomes/*.json › portals.hp), die `portalIsland` in Go prüft.
pub const PORTAL_HP: f64 = 500.0;

/// Eine Stufe mit zwei Portalen (Hub ± 40), Portal-HP aus den Daten und Verteidiger-Takt 15 s.
pub fn portal_stage() -> (World, Arc<WorldValues>) {
    let mut w = bare_world();
    let v = boss_values();
    w.vals = Some(v.clone());
    w.width_units = 400.0;
    w.portals = vec![w.hub_x - 40.0, w.hub_x + 40.0];
    w.biome.portals.hp = PORTAL_HP;
    w.biome.portals.defense.interval_seconds = 15.0;
    w.biome.portals.defense.size = 3;
    (w, v)
}

/// Go: `attacker`: ein Krieger im Angriff vor dem Portal `i`; liefert seinen Index.
pub fn attacker(w: &mut World, v: &WorldValues, i: usize) -> usize {
    let hp = v.troops.get("warrior").map_or(1.0, |d| d.hp);
    let id = w.new_id();
    w.troops.push(Troop {
        id,
        kind: "warrior".into(),
        x: w.portals[i],
        hp,
        max_hp: hp,
        anchor_x: w.hub_x,
        attacking: true,
        ..Troop::default()
    });
    w.troops.len() - 1
}
