//! Status: je Raum Plätze, Takt und Tick-Dauer, dazu abgestürzte Räume (früher `/api/status`, B-027; heute für das native
//! Spiel und Tests), und der verdichtete Zustand `Summary`. Port von `status.go`; die JSON-Namen sind die der Go-Tags.
use std::collections::BTreeMap;

use serde::Serialize;

use super::env::Store;
use super::info::Info;
use super::log::Log;
use super::manager::Manager;
use super::run::Failure;
use super::types::{MonarchState, Room};

/// Tick-Dauer in Millisekunden.
#[derive(Debug, Clone, Copy, PartialEq, Serialize)]
pub struct TickMs {
    pub last: f64,
    pub p99: f64,
}

/// Eine Stufe der Insel: Tiefe, Phase, Tag und die Indizes ihrer Spieler (= Monarchen).
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct StageStatus {
    pub depth: i64,
    pub phase: String,
    pub day: i64,
    pub players: Vec<i64>,
}

/// Ein Raum in `Manager::status`.
#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct Status {
    #[serde(flatten)]
    pub info: Info,
    /// Verbunden.
    pub devices: usize,
    /// `taken`, `waiting`, `free` je Index.
    pub monarchs: Vec<MonarchState>,
    pub tick: i64,
    #[serde(rename = "tickMs")]
    pub tick_ms: TickMs,
    pub stages: Vec<StageStatus>,
}

/// Ein Gerät im Raum. `id` ist nur eine Kennung (Anfang der Geräte-ID); die volle ID dient dem Wiederverbinden und
/// verlässt den Server nie.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct DeviceInfo {
    pub id: String,
    pub connected: bool,
    pub slots: Vec<i64>,
}

/// Der verdichtete Zustand eines Raums (Beobachtung in Tests).
#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct Summary {
    pub code: String,
    pub depth: i64,
    pub tick: i64,
    pub phase: String,
    pub day: i64,
    /// Je Monarch.
    pub gold: Vec<i64>,
    pub troops: BTreeMap<String, i64>,
    pub enemies: usize,
    #[serde(rename = "castleHp")]
    pub castle: f64,
    pub wave: i64,
    pub devices: Vec<DeviceInfo>,
    pub stages: Vec<StageStatus>,
    /// Zeitraffer, mindestens 1 (B-232).
    #[serde(rename = "timescale")]
    pub scale: i64,
    /// Dev-Pause (B-232).
    pub paused: bool,
    /// Dev-Schalter: Baukosten 1.
    #[serde(rename = "devCheapBuild")]
    pub dev_cheap: bool,
    /// Zusätzliches Lager-Maximum.
    #[serde(rename = "devStockBonus")]
    pub dev_stock_plus: i64,
    /// Gold-Maximum je Spieler zusätzlich (gleicher Bonus wie `devStockBonus`).
    #[serde(rename = "devPurseBonus")]
    pub dev_purse_bonus: i64,
    /// Zeitraffer endet mit der Tageszeit.
    #[serde(rename = "devTimescaleAuto")]
    pub dev_auto_reset: bool,
}

impl Room {
    /// Beschreibt alle Stufen.
    pub fn stages(&self) -> Vec<StageStatus> {
        self.isl
            .stages
            .iter()
            .map(|w| StageStatus {
                depth: w.biome.depth,
                phase: w.cycle.phase.clone(),
                day: w.cycle.day,
                players: w.players.iter().map(|p| p.index).collect(),
            })
            .collect()
    }

    /// Der Raum als Eintrag von `Manager::status`.
    pub fn status(&self) -> Status {
        let (last, p99) = self.tick_ms();
        Status {
            info: self.info(),
            devices: self.connected(),
            monarchs: self.states(),
            tick: self.tick,
            tick_ms: TickMs { last, p99 },
            stages: self.stages(),
        }
    }

    /// Verdichtet den aktuellen Zustand. Gegner, Truppen, Burg und Welle: die Startstufe; alle Stufen stehen in `stages`.
    pub fn summary(&self) -> Summary {
        let w = &self.isl.stages[self.start];
        let mut troops = BTreeMap::new();
        for t in &w.troops {
            *troops.entry(t.kind.clone()).or_insert(0) += 1;
        }
        Summary {
            code: self.code.clone(),
            depth: w.biome.depth,
            tick: self.tick,
            phase: w.cycle.phase.clone(),
            day: w.cycle.day,
            gold: self.isl.players().iter().map(|p| p.gold).collect(),
            troops,
            enemies: w.enemies.len(),
            castle: w.castle.hp,
            wave: w.wave,
            devices: self.device_infos(),
            stages: self.stages(),
            scale: self.scale(),
            paused: self.paused,
            dev_cheap: self.isl.dev.cheap_build,
            dev_stock_plus: self.isl.dev.stock_bonus,
            dev_purse_bonus: self.isl.dev.stock_bonus,
            dev_auto_reset: self.auto_reset,
        }
    }

    /// Beschreibt die Geräte, nach Kennung sortiert. Die Kennung ist der kürzeste gemeinsame Anfang (mindestens 6
    /// Zeichen) der Geräte-IDs, der alle Geräte des Raums unterscheidet.
    pub fn device_infos(&self) -> Vec<DeviceInfo> {
        let ids: Vec<&String> = self.devices.keys().collect();
        let mut n = 6;
        while n < 64 && !unique_prefixes(&ids, n) {
            n += 1;
        }
        let mut out: Vec<DeviceInfo> = self
            .devices
            .iter()
            .map(|(id, d)| DeviceInfo {
                id: prefix(id, n),
                connected: d.connected,
                slots: d.slots.keys().copied().collect(),
            })
            .collect();
        out.sort_by(|a, b| a.id.cmp(&b.id));
        out
    }
}

/// Die ersten `n` Bytes der ID (Go `id[:min(n, len(id))]`); an einer Zeichengrenze, sonst entfällt das Zeichen.
fn prefix(id: &str, n: usize) -> String {
    let mut end = n.min(id.len());
    while !id.is_char_boundary(end) {
        end -= 1;
    }
    id[..end].to_string()
}

/// Unterscheiden die ersten `n` Zeichen der IDs alle?
pub fn unique_prefixes(ids: &[&String], n: usize) -> bool {
    let mut seen = std::collections::BTreeSet::new();
    ids.iter().all(|id| seen.insert(prefix(id, n)))
}

impl<S: Store, L: Log> Manager<S, L> {
    /// Liefert alle Räume (nach Code) und die letzten Abstürze.
    pub fn status(&self) -> (Vec<Status>, Vec<Failure>) {
        (self.rooms.values().map(Room::status).collect(), self.failures.clone())
    }
}
