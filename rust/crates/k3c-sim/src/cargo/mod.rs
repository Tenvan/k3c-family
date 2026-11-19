//! Fracht auf dem Schiff (RU62.1, B-967, docs/rules/stufen.md § 1, wirtschaft.md § 8; Entscheidung 013: neue Mechanik nur
//! in Rust, ohne Go-Orakel): Das Schiff hat einen Laderaum mit Plätzen. Am Steg des Hafens stehen drei Ladeplätze (Münzen,
//! Truppen, Material); die Fracht fährt mit und kommt auf der neuen Insel an. Daten: data/cargo.json (`k3c-data/cargo.rs`).
//!
//! Die Mechanik gehört der Insel (`Island::cargo`), nicht der Stufe: Der Vorrat gehört der Insel, und die Fracht überlebt
//! den Inselwechsel. Aktiv ist sie nur in Welten aus `cargo.json › worlds` (nicht `standard`), nutzbar erst mit fertigem
//! Segel und nicht auf der letzten Insel der Welt. Ohne `cfg` rechnet nichts und nichts wird gespeichert.
//!
//! - `step.rs`: der Takt (Aufträge, Einwürfe, Überlauf, Zustand für die Anzeige).
//! - `load.rs`: Prüfen und Laden je Ladeplatz.
//! - `orders.rs`: Bauern tragen Kisten zum Steg.
//! - `arrive.rs`: Abfahrt (Aufträge zurück) und Ankunft auf der neuen Insel.
//! - `save.rs`: Spielstand-Teil (`ISLAND_SAVE_VERSION` 10).
//! - `view.rs`: der Zustand `World::cargo` für die Anzeige.

use std::sync::Arc;

use k3c_data::Data;
use k3c_data::buildings::LevelData;
use k3c_data::cargo::CargoFile;

use crate::island::Island;
use crate::state::WorldValues;
use crate::world::ship::SHIP_KIND;

pub mod arrive;
pub mod load;
pub mod orders;
pub mod save;
pub mod step;
pub mod view;

#[cfg(test)]
mod arrive_tests;
#[cfg(test)]
mod load_tests;
#[cfg(test)]
mod save_tests;
#[cfg(test)]
mod step_tests;
#[cfg(test)]
pub(crate) mod testutil;

/// Art einer Ladung: Beutel Münzen.
pub const KIND_COINS: &str = "coins";
/// Art einer Ladung: Truppe.
pub const KIND_TROOP: &str = "troop";
/// Art einer Ladung: Kiste Material.
pub const KIND_CRATE: &str = "crate";

/// Einer der drei Ladeplätze am Steg (feste Reihenfolge: Münzen, Truppen, Material).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Bay {
    Coins,
    Troops,
    Crates,
}

impl Bay {
    /// Alle Ladeplätze in fester Reihenfolge.
    pub const ALL: [Bay; 3] = [Bay::Coins, Bay::Troops, Bay::Crates];

    /// Name im Zustand und in Ereignissen.
    pub fn name(self) -> &'static str {
        match self {
            Bay::Coins => "coins",
            Bay::Troops => "troops",
            Bay::Crates => "crates",
        }
    }

    /// Versatz zum Schiff in Units.
    pub fn offset(self, cfg: &CargoFile) -> f64 {
        match self {
            Bay::Coins => cfg.bays.coins,
            Bay::Troops => cfg.bays.troops,
            Bay::Crates => cfg.bays.crates,
        }
    }

    /// Münzen für einen Einwurf.
    pub fn price(self, cfg: &CargoFile) -> i64 {
        match self {
            Bay::Coins => cfg.purse_coins,
            Bay::Troops => cfg.troop_coins,
            Bay::Crates => cfg.crate_coins,
        }
    }
}

/// Eine Ladung an Bord.
#[derive(Debug, Clone, PartialEq, Default)]
pub struct CargoItem {
    /// `KIND_COINS`, `KIND_TROOP` oder `KIND_CRATE`.
    pub kind: String,
    /// Truppenart oder Material; beim Beutel leer.
    pub id: String,
    /// Münzen, Einheiten; bei der Truppe 1.
    pub amount: i64,
    /// Lebenspunkte der Truppe beim Verladen (sonst 0).
    pub hp: f64,
}

/// Ein Bauer trägt eine Kiste zum Steg; das Material ist ab Auftrag aus dem Vorrat genommen (reserviert).
#[derive(Debug, Clone, PartialEq, Default)]
pub struct CrateOrder {
    pub resource: String,
    pub units: i64,
    /// ID der Truppe des Trägers; 0 = noch keiner (oder nach dem Laden eines Spielstands).
    pub carrier: i64,
    /// Sekunden bis zur Ankunft am Steg.
    pub eta: f64,
}

/// Ein laufender Einwurf eines Spielers an einem Ladeplatz (nur im Speicher: beim Speichern gilt er als abgebrochen).
#[derive(Debug, Clone, PartialEq)]
pub struct PayState {
    pub bay: Bay,
    pub paid: i64,
    /// Sekunden bis zur nächsten Münze (≤ 0: sofort).
    pub acc: f64,
}

/// Zustand der Fracht einer Insel.
#[derive(Debug, Clone, PartialEq, Default)]
pub struct Cargo {
    /// Die Regeln aus cargo.json; `None` = Insel ohne Fracht (nichts rechnet, nichts wird gespeichert).
    pub cfg: Option<Arc<CargoFile>>,
    /// Fracht an Bord in Lade-Reihenfolge.
    pub load: Vec<CargoItem>,
    /// Offene Träger-Aufträge (belegen je einen Platz).
    pub orders: Vec<CrateOrder>,
    /// Laufende Einwürfe je Spielerindex; nur über den Schlüssel zugreifen.
    pub pay: std::collections::BTreeMap<i64, PayState>,
    /// Spieler, deren Einwurf abgelehnt wurde und die die Taste noch halten (damit das Ereignis nur einmal kommt).
    pub rejected: std::collections::BTreeSet<i64>,
    /// Gerade angekommene Fracht, das Ereignis `cargoArrived` kommt im ersten Takt.
    pub arrived: Vec<CargoItem>,
}

impl Cargo {
    /// Belegte und reservierte Plätze.
    pub fn taken(&self) -> usize {
        self.load.len() + self.orders.len()
    }
}

/// Trägt die Welt `world_id` Fracht (`cargo.json › worlds`)?
pub fn world_has_cargo(data: &Data, world_id: &str) -> bool {
    data.cargo.worlds.iter().any(|w| w == world_id)
}

/// Der Wertesatz mit dem Laderaum als 4. Stufe des Schiffs (RU62.1): `data/ship.json` bleibt unberührt, die Stufe aus
/// cargo.json kommt dazu. Schon ergänzte Wertesätze bleiben unverändert (der Inselwechsel reicht den Satz der Welt weiter).
pub fn with_hold(v: &Arc<WorldValues>, data: &Data) -> Arc<WorldValues> {
    let base = data.ship.stages.len();
    let Some(ship) = v.buildings.get(SHIP_KIND) else {
        return Arc::clone(v);
    };
    if ship.levels.len() > base {
        return Arc::clone(v);
    }
    let mut out = (**v).clone();
    if let Some(ship) = out.buildings.get_mut(SHIP_KIND) {
        ship.levels.push(LevelData {
            cost: data.cargo.hold.cost.clone(),
            hp: data.cargo.hold.hp,
            build_seconds: data.cargo.hold.build_seconds,
            ..LevelData::default()
        });
    }
    Arc::new(out)
}

impl Island {
    /// Schaltet die Fracht der Insel ein (nur Welten aus cargo.json; Aufruf beim Bauen der Insel): Regeln übernehmen und den
    /// Laderaum als 4. Schiffsstufe in den Wertesatz aller Stufen eintragen.
    pub fn enable_cargo(&mut self, data: &Data) {
        self.cargo.cfg = Some(Arc::new(data.cargo.clone()));
        for w in &mut self.stages {
            let v = crate::world::vals(w);
            w.vals = Some(with_hold(&v, data));
            w.cargo_hold = true;
        }
    }
}
