//! Typen aus anderen Go-Dateien, die der Zustand (`World`, `Player`, `Site` …) braucht. Hier stehen nur die Daten;
//! die Gruppe, die die Go-Datei portiert (Quelle je Typ in der Doku), ergänzt die Funktionen.

use std::collections::BTreeMap;

use k3c_data::bosses::BossesFile;
use k3c_data::buildings::BuildingData;
use k3c_data::economy::EconomyData;
use k3c_data::enemies::EnemyData;
use k3c_data::events::NightEvent;
use k3c_data::goals::Goals;
use k3c_data::hub::HubData;
use k3c_data::monarch::{MonarchData, SkillData};
use k3c_data::troops::TroopData;
use k3c_data::waves::WavesData;
use serde::{Deserialize, Serialize};

use super::omit::{is_zero_f64, is_zero_i64};

/// Zustände einer gefallenen Krone (Go: `crown.go`).
pub const CROWN_FLYING: &str = "flying";
pub const CROWN_GROUND: &str = "ground";
pub const CROWN_ENEMY: &str = "enemy";
pub const CROWN_PLAYER: &str = "player";

/// Go: `sim.Drop` (unit_warrior.go), Ausrüstung am Boden; heißt in Rust `ItemDrop`, weil `Drop` den Prelude-Trait schattiert.
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
#[serde(default)]
pub struct ItemDrop {
    #[serde(rename = "id")]
    pub id: i64,
    #[serde(rename = "kind")]
    pub kind: String,
    #[serde(rename = "x")]
    pub x: f64,
    #[serde(rename = "workSite", skip_serializing_if = "is_zero_i64")]
    pub work_site: i64,
}

/// Go: `sim.Crown` (crown.go), die gefallene Krone eines Monarchen.
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
#[serde(default)]
pub struct Crown {
    /// `Player.Index` des Besitzers.
    #[serde(rename = "owner")]
    pub owner: i64,
    #[serde(rename = "state")]
    pub state: String,
    #[serde(rename = "x")]
    pub x: f64,
    #[serde(rename = "z")]
    pub z: f64,
    #[serde(rename = "fromX", skip_serializing_if = "is_zero_f64")]
    pub from_x: f64,
    #[serde(rename = "toX", skip_serializing_if = "is_zero_f64")]
    pub to_x: f64,
    /// Flugzeit in Sekunden.
    #[serde(rename = "t", skip_serializing_if = "is_zero_f64")]
    pub t: f64,
    /// ID des Trägers (Gegner oder Monarch), sonst 0.
    #[serde(rename = "carrier", skip_serializing_if = "is_zero_i64")]
    pub carrier: i64,
}

/// Go: `sim.Merchant` (merchant.go), der anwesende Händler.
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
#[serde(default)]
pub struct Merchant {
    #[serde(rename = "resource")]
    pub resource: String,
    #[serde(rename = "leaves")]
    pub leaves: i64,
    #[serde(rename = "buyPaid", skip_serializing_if = "is_zero_i64")]
    pub buy_paid: i64,
    #[serde(rename = "raid", skip_serializing_if = "super::omit::is_false")]
    pub raid: bool,
    #[serde(rename = "hp")]
    pub hp: f64,
    #[serde(rename = "maxHp")]
    pub max_hp: f64,
}

/// Go: `sim.WarnZone` (world_state_mirror.go), nächster Flächenschlag eines Bosses.
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
#[serde(default)]
pub struct WarnZone {
    #[serde(rename = "x")]
    pub x: f64,
    #[serde(rename = "r")]
    pub r: f64,
    #[serde(rename = "in")]
    pub r#in: f64,
}

/// Go: `sim.ActiveEvent` (world_state_mirror.go), ein laufendes Nacht-Event.
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
#[serde(default)]
pub struct ActiveEvent {
    #[serde(rename = "id")]
    pub id: String,
    #[serde(rename = "secondsLeft")]
    pub seconds_left: f64,
}

/// Go: `sim.SwitchTarget` (island_targets.go), ein Ziel am Hafen.
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
#[serde(default)]
pub struct SwitchTarget {
    #[serde(rename = "id")]
    pub id: String,
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "etappe")]
    pub etappe: i64,
}

/// Go: `sim.SwitchState` (world_state_mirror.go), Wechselpunkt der Insel.
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
#[serde(default)]
pub struct SwitchState {
    #[serde(rename = "open")]
    pub open: bool,
    #[serde(rename = "progress")]
    pub progress: f64,
    #[serde(rename = "ready")]
    pub ready: bool,
    #[serde(rename = "targets", skip_serializing_if = "Vec::is_empty")]
    pub targets: Vec<SwitchTarget>,
    #[serde(rename = "choice", skip_serializing_if = "String::is_empty")]
    pub choice: String,
    #[serde(rename = "completed", skip_serializing_if = "Vec::is_empty")]
    pub completed: Vec<String>,
}

/// Go: `sim.CycleInfo` (world_cycle.go), Stand des Tag/Nacht-Zyklus.
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
#[serde(default)]
pub struct CycleInfo {
    /// day, dusk, night, dawn.
    #[serde(rename = "phase")]
    pub phase: String,
    #[serde(rename = "day")]
    pub day: i64,
    #[serde(rename = "progress")]
    pub progress: f64,
    #[serde(rename = "secondsLeft")]
    pub seconds_left: f64,
}

/// Go: `sim.combatRuntime` (combat.go): bezahlter Betrag je Zahlziel und bezahlte, noch nicht vergebene Schilde.
/// Nicht im Spielstand, nicht im JSON.
#[derive(Debug, Clone, PartialEq, Default)]
pub struct CombatRuntime {
    pub paid: BTreeMap<String, i64>,
    pub shields: i64,
}

/// Go: `sim.CombatSettings` (combat_data.go), `combat` einer Welt; leer = Standard.
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
#[serde(default)]
pub struct CombatSettings {
    #[serde(rename = "mode", skip_serializing_if = "String::is_empty")]
    pub mode: String,
    #[serde(rename = "groupSize", skip_serializing_if = "is_zero_i64")]
    pub group_size: i64,
}

/// Go: `sim.skillTable` (monarch_data.go): `monarch.json › skills` als nach ID sortierte Liste samt Stelle jeder ID.
#[derive(Debug, Clone, PartialEq, Default)]
pub struct SkillTable {
    pub list: Vec<SkillData>,
    pub index: BTreeMap<String, usize>,
}

/// Go: `sim.WorldValues` (world_values.go), der aufgelöste Wertesatz einer Welt. Wird als `Arc<WorldValues>` geteilt
/// und nie geändert (Vorentscheidung 6).
#[derive(Debug, Clone, PartialEq, Default)]
pub struct WorldValues {
    /// Skins der Welt: Basis-Asset → Ersatz-Asset (WE6.6, B-994); das Spiel zeichnet damit, die Sim rechnet nicht damit.
    pub skins: BTreeMap<String, String>,
    /// Mit dem Platz `ship` aus ship.json (`withShip`).
    pub buildings: BTreeMap<String, BuildingData>,
    pub troops: BTreeMap<String, TroopData>,
    pub economy: EconomyData,
    pub combat: CombatSettings,
    pub hub: HubData,
    pub monarch: MonarchData,
    pub enemies: BTreeMap<String, EnemyData>,
    pub waves: WavesData,
    pub goals: Goals,
    pub events: Vec<NightEvent>,
    pub skills: SkillTable,
    /// Go: die Paket-Variablen `bosses` und `endBosses` (boss_data.go); RU14 füllt sie beim Aufbau des Wertesatzes.
    pub bosses: BossesFile,
    /// HB1.4/HB1.5: native Regeln (Wächter-Bäume der Lager `unit/camp_guard.rs`, Hafen nah am Lager mit Start dort und Bäumen `island/harbor_start.rs`). Nur die native Regel,
    /// ohne Daten-Datei; die Go-Golden-Läufe schalten es ab, weil Go sie nicht kennt.
    pub native_rules: bool,
}
