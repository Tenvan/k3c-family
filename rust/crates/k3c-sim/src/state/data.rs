//! Balancing-Daten aus `data/` (Port von `engine/sim/data.go`).
//!
//! Die Lese-Typen von Go (`Cost`, `BuildingData`, `TroopData`, `HubData` …) und das Laden samt Prüfungen (`load`,
//! `loadFrom`, `loadBiomes`, `checkPool`, `checkWaves`) liegen seit RU3 in `k3c-data`; sie werden hier wiedergegeben, nicht
//! doppelt geführt. Gos Paket-Variablen (`buildings`, `troops`, `economy` …) und `UseData` gibt es in Rust nicht: Es gibt
//! keinen veränderlichen globalen Zustand, ein Wertesatz ist `Arc<WorldValues>` (Zustandsmodell, Vorentscheidung 6) und
//! kommt aus `k3c_data::load_all` (`Data`).

use k3c_data::biome::{Biome, Cycle};

pub use k3c_data::buildings::{
    ArmorStage, BuildingData, Cost, Heal, LevelData, Offer, Plantation, Spell, TroopLimit, Vagrants,
};
pub use k3c_data::economy::{EconomyData, Gatherable, VeinData};
pub use k3c_data::enemies::EnemyData;
pub use k3c_data::goals::Goals;
pub use k3c_data::hub::{HubData, HubSite, SitePlacementData};
pub use k3c_data::troops::TroopData;
pub use k3c_data::waves::WavesData;

/// Go: `checkPool`: Jeder Gegner in `enemies.portal` und `enemies.night` steht in enemies.json, der Portal-Pool hat
/// mindestens einen Standard- und einen Elite-Gegner (B-129).
pub use k3c_data::biome::check_pool;

/// Go: `firstDayNight`, der globale Zyklus: der des ersten Tag/Nacht-Bioms (Oberwelt), wie `globalDayNight()` in TS.
/// `biomes` ist die nach Tiefe sortierte Liste (`Data::biomes`).
#[allow(clippy::panic)] // STAB(B-1076): Daten ohne Tag/Nacht-Biom, Fehlerweg folgt
pub fn first_day_night(biomes: &[Biome]) -> Cycle {
    for b in biomes {
        if b.cycle.r#type == "dayNight" {
            return b.cycle.clone();
        }
    }
    panic!("Kein Biom mit Tag/Nacht-Zyklus");
}

/// Go: `hasDepth`: Gibt es ein Biom dieser Tiefe? (Port von `hasDepth` aus travel.ts.)
pub fn has_depth(biomes: &[Biome], depth: i64) -> bool {
    biomes.iter().any(|b| b.depth == depth)
}

/// Go: `checkWaves`: `bossHpPerExtraPlayer` ist Pflicht, beide Zuschläge sind nicht negativ (B-650).
pub fn check_waves(w: &WavesData) -> Result<(), String> {
    match w.boss_hp_per_extra_player {
        None => Err("data/waves.json: bossHpPerExtraPlayer fehlt".to_string()),
        Some(b) if b < 0.0 || w.per_extra_player < 0.0 => {
            Err("data/waves.json: perExtraPlayer und bossHpPerExtraPlayer dürfen nicht negativ sein".to_string())
        }
        Some(_) => Ok(()),
    }
}
