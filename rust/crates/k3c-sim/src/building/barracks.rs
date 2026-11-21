//! Kaserne und Kämpfer-Limit (Port von `engine/sim/building_barracks.go`, B-116, Q29): Es zählen nur Kämpfer, Basis je Hub
//! plus Zuschlag je gebauter Kaserne (data/buildings.json › barracks.troopLimit). Geprüft wird nur beim Waffe-Holen
//! (`weapon_to_fetch`, `unit/warrior.rs`).

use crate::state::World;
use crate::unit::warrior::{fetches_fighter_weapon, is_fighter};
use crate::world::values_room::values;

/// Go: `fighters` zählt die Kämpfer der Welt: Bogenschützen, Krieger und Elite (Q40); Bauern, Berufe, Landstreicher nicht.
pub fn fighters(w: &World) -> i64 {
    let v = values(w);
    w.troops.iter().filter(|t| is_fighter(&v.troops, &t.kind)).count() as i64
}

/// Go: `weaponsInFlight` zählt die Bauern außer `w.troops[ti]`, die gerade eine Kämpfer-Waffe holen (Regal oder Boden);
/// sie werden gleich Kämpfer.
pub fn weapons_in_flight(w: &World, ti: usize) -> i64 {
    let v = values(w);
    w.troops
        .iter()
        .enumerate()
        .filter(|(oi, o)| *oi != ti && o.job.as_ref().is_some_and(|j| fetches_fighter_weapon(w, &v, j)))
        .count() as i64
}

/// Go: `troopLimit` ist das Kämpfer-Limit der Welt: Basis plus Zuschlag je gebauter Kaserne; eine zerstörte zählt nicht.
pub fn troop_limit(w: &World) -> i64 {
    let base = values(w).buildings.get("barracks").map_or(0, |b| b.troop_limit.base);
    base + barracks_bonus(w)
}

/// Limit-Zuschlag der gebauten Kasernen der Stufe (ohne Basis).
pub fn barracks_bonus(w: &World) -> i64 {
    let v = values(w);
    let per_building = v.buildings.get("barracks").map_or(0, |b| b.troop_limit.per_building);
    let built = w
        .sites
        .iter()
        .filter(|s| s.kind == "barracks" && s.state == "built")
        .count() as i64;
    per_building * built
}

/// EQ1.4: Kämpfer der ganzen Insel (diese Stufe plus alle anderen); ohne Insel oder mit Go-Regeln nur die Stufe.
pub fn island_fighters(w: &World) -> i64 {
    fighters(w) + w.fighters_elsewhere
}

/// EQ1.4: Truppen-Limit der ganzen Insel (Basis einmal, Zuschlag aller Kasernen); ohne Insel nur die Stufe.
pub fn island_limit(w: &World) -> i64 {
    troop_limit(w) + w.limit_elsewhere
}
