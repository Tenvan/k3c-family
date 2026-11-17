//! data/waves.json: Go `sim.wavesData` (engine/sim/data.go, `loadWaves`, `checkWaves`).

use serde::{Deserialize, Serialize};

use crate::{Fehler, decode};

/// Go: `sim.wavesData`.
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
#[serde(default)]
pub struct WavesData {
    #[serde(rename = "Table", alias = "table")]
    pub table: Vec<WavesDataTable>,
    #[serde(rename = "SpawnSpreadSeconds", alias = "spawnSpreadSeconds")]
    pub spawn_spread_seconds: f64,
    #[serde(rename = "PerExtraPlayer", alias = "perExtraPlayer")]
    pub per_extra_player: f64,
    #[serde(rename = "BossHpPerExtraPlayer", alias = "bossHpPerExtraPlayer")]
    pub boss_hp_per_extra_player: Option<f64>,
    #[serde(rename = "DepthScaling", alias = "depthScaling")]
    pub depth_scaling: WavesDataDepthScaling,
    #[serde(rename = "StealGold", alias = "stealGold")]
    pub steal_gold: i64,
}

/// Go: `struct { FromWave int; Standard [2]int; Elite [2]int }`.
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
#[serde(default)]
pub struct WavesDataTable {
    #[serde(rename = "FromWave", alias = "fromWave")]
    pub from_wave: i64,
    #[serde(rename = "Standard", alias = "standard")]
    pub standard: [i64; 2],
    #[serde(rename = "Elite", alias = "elite")]
    pub elite: [i64; 2],
}

/// Go: `struct { HP float64; Damage float64; Speed float64 }`.
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
#[serde(default)]
pub struct WavesDataDepthScaling {
    #[serde(rename = "HP", alias = "hp")]
    pub hp: f64,
    #[serde(rename = "Damage", alias = "damage")]
    pub damage: f64,
    #[serde(rename = "Speed", alias = "speed")]
    pub speed: f64,
}

/// `FILE` ist der Name unter data/.
pub const FILE: &str = "waves.json";

/// `load` liest waves.json und prüft wie `checkWaves`: bossHpPerExtraPlayer ist Pflicht, beide Zuschläge sind nicht
/// negativ (B-650).
pub fn load(raw: &str) -> Result<WavesData, Fehler> {
    let w: WavesData = decode("data/waves.json", raw)?;
    let fail = |s: &str| Err(Fehler::Pruefung(format!("data/waves.json: {s}")));
    match w.boss_hp_per_extra_player {
        None => fail("bossHpPerExtraPlayer fehlt"),
        Some(b) if b < 0.0 || w.per_extra_player < 0.0 => {
            fail("perExtraPlayer und bossHpPerExtraPlayer dürfen nicht negativ sein")
        }
        Some(_) => Ok(w),
    }
}
