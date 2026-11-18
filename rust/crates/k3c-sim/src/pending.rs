//! Platzhalter der Gruppen `enemies` und `unit` (Port-Leitfaden § 7): Funktionen späterer Sprints (RU10, RU15, RU18, RU19),
//! die RU11 ruft. Der liefernde Sprint löscht den Platzhalter; RU22 prüft, dass keiner übrig ist. Die Platzhalter
//! aus RU8 stehen weiter in `state/pending.rs`. Go-Zeiger auf Gegner und Truppen sind hier Indizes (`ei`, `ti`).
#![allow(clippy::unimplemented)] // Platzhalter: wartet auf den genannten Sprint (RU10, RU15, RU18, RU19)

use crate::state::World;
use crate::state::island_ref::IslandRef;

// Von RU15 portiert (Bosse): Die Import-Stelle bleibt `pending`, damit die Gegner-Dateien unverändert bleiben.
pub use crate::boss::abilities::hit_splash;
pub use crate::boss::{boss_ability, boss_defeated, trigger_boss};

// Von RU14.2 portiert (world_cycle.go): die Import-Stelle bleibt `pending`.
pub use crate::world::cycle::add_pool;

// Von RU15 portiert (Krone, Portale): die Import-Stelle bleibt `pending`.
pub use crate::crown::{crown_stolen, drop_crown_of, take_crown_or_loot};
pub use crate::portal::drop_loot;

pub use crate::merchant::merchant_x; // Von RU18.4 portiert (merchant.go)

pub use crate::monarch::{damage_mult_of, frost_armor_hit}; // Von RU18.5 portiert (monarch_passives.go)

pub use crate::economy::{scaled_gold, scatter_coins}; // Von RU18.2 portiert (economy_fountain.go, economy.go)

// Von RU18.1 portiert (building_*.go): die Import-Stelle bleibt `pending`, damit die Gruppen `unit` und `enemies`
// unverändert bleiben.
pub use crate::building::barracks::{fighters, troop_limit, weapons_in_flight};
pub use crate::building::outpost::{outpost, outpost_job};
pub use crate::building::repair::{repair, repair_job};
pub use crate::building::spell_tower::is_spell_tower;
pub use crate::building::tavern::{vagrants_at, wander_radius};
pub use crate::building::upgrades::armored_hp;

// Von RU14.4 portiert (world_vein.go): die Import-Stelle bleibt `pending`.
pub use crate::world::vein::{gather_vein, is_vein, node_free};

/// Go: `addStockCapped` (island_storage.go): legt höchstens bis zur Kapazität der Insel ab und liefert die aufgenommene
/// Menge; `None`: kein Baumaterial (Go: `ok` false).
pub fn add_stock_capped(_w: &mut World, _isl: Option<&IslandRef>, _resource: &str, _amount: i64) -> Option<i64> {
    unimplemented!("RU10: add_stock_capped")
}

pub use crate::monarch::add_pool_points; // Von RU18.4 portiert (monarch.go)

pub use crate::hub::{each_site, finish_upgrade, next_level, upgrade_job}; // Von RU18.3 portiert (hub_level.go)

// Von RU19.1 und RU19.2 portiert (dev.go, events*.go): die Import-Stelle bleibt `pending`.
pub use crate::dev::pure;
pub use crate::events::merchant::{merchant_raid_end, merchant_raid_start};
pub use crate::events::moon::{full_moon_extra, moon_damage, moon_drop};
pub use crate::events::{arrow_event, build_progress_event, emit, unit_x};
