//! Zustand der Simulation, Teil 3: Gegner, Geschosse, Flächen, Camps, Reise (Port von `engine/sim/types.go`).

use serde::{Deserialize, Serialize};

use super::omit::{is_false, is_zero_f64, is_zero_i64};
use super::shared::{Crown, Drop, WarnZone};

/// Ein Ereignis des letzten Ticks, z. B. `{"type": "night", "day": 1}`. Eine Map, weil die Felder je Typ wechseln und
/// Werte wie `player: 0` erhalten bleiben müssen (Go: `map[string]any`; die Schlüssel sind sortiert wie bei Go).
pub type Event = serde_json::Map<String, serde_json::Value>;

/// Ein Gegner.
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
#[serde(default)]
pub struct Enemy {
    #[serde(rename = "id")]
    pub id: i64,
    #[serde(rename = "kind")]
    pub kind: String,
    #[serde(rename = "x")]
    pub x: f64,
    #[serde(rename = "hp")]
    pub hp: f64,
    #[serde(rename = "maxHp")]
    pub max_hp: f64,
    #[serde(rename = "damage")]
    pub damage: f64,
    #[serde(rename = "speed")]
    pub speed: f64,
    #[serde(rename = "range")]
    pub range: f64,
    #[serde(rename = "traits")]
    pub traits: Vec<String>,
    #[serde(rename = "cooldown")]
    pub cooldown: f64,
    #[serde(rename = "fleeing")]
    pub fleeing: bool,
    #[serde(rename = "carriedGold")]
    pub carried_gold: i64,
    /// Weggetragene Ausrüstung.
    #[serde(rename = "carried", skip_serializing_if = "Option::is_none")]
    pub carried: Option<Drop>,
    /// Getragene Krone eines Monarchen: im Portal ist sie geraubt.
    #[serde(rename = "crown", skip_serializing_if = "Option::is_none")]
    pub crown: Option<Crown>,
    /// Portal, aus dem der Gegner kam.
    #[serde(rename = "homeX")]
    pub home_x: f64,
    /// Sekunden betäubt (kein Schritt, kein Angriff).
    #[serde(rename = "stun", skip_serializing_if = "is_zero_f64")]
    pub stun: f64,
    /// So lange zielt der Gegner auf den Spieler `taunt_id`.
    #[serde(rename = "tauntId", skip_serializing_if = "is_zero_i64")]
    pub taunt_id: i64,
    #[serde(rename = "tauntFor", skip_serializing_if = "is_zero_f64")]
    pub taunt_for: f64,
    /// Faktor der Geschwindigkeit für `slow_for` Sekunden.
    #[serde(rename = "slow", skip_serializing_if = "is_zero_f64")]
    pub slow: f64,
    #[serde(rename = "slowFor", skip_serializing_if = "is_zero_f64")]
    pub slow_for: f64,
    /// Sekunden bis zum nächsten Flächenschlag (`aoe`).
    #[serde(rename = "aoeIn", skip_serializing_if = "is_zero_f64")]
    pub aoe_in: f64,
    /// Spawn-Zeit, nur bei `phases`.
    #[serde(rename = "spawned", skip_serializing_if = "is_zero_f64")]
    pub spawned: f64,
    /// Phase (nur Endboss, ab 1).
    #[serde(rename = "phase", skip_serializing_if = "is_zero_i64")]
    pub phase: i64,
    /// Nächster Flächenschlag.
    #[serde(rename = "warn", skip_serializing_if = "Option::is_none")]
    pub warn: Option<WarnZone>,
    /// Miniboss oder Endboss; `kind` ist dann die Boss-ID.
    #[serde(rename = "boss", skip_serializing_if = "is_false")]
    pub boss: bool,
    /// Wache eines Gegnerlagers.
    #[serde(rename = "guard", skip_serializing_if = "is_false")]
    pub guard: bool,
}

/// Ein laufender Lightning Storm: `per_second` Schaden je Sekunde an Gegnern im Radius um `x`, noch `left` Sekunden.
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
#[serde(default)]
pub struct Storm {
    #[serde(rename = "x")]
    pub x: f64,
    #[serde(rename = "radius")]
    pub radius: f64,
    #[serde(rename = "perSecond")]
    pub per_second: f64,
    #[serde(rename = "left")]
    pub left: f64,
    /// ID des wirkenden Spielers.
    #[serde(rename = "owner")]
    pub owner: i64,
}

/// Ein Pfeil oder Geschoss.
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
#[serde(default)]
pub struct Projectile {
    #[serde(rename = "id")]
    pub id: i64,
    #[serde(rename = "x")]
    pub x: f64,
    #[serde(rename = "targetId")]
    pub target_id: i64,
    /// player, enemy.
    #[serde(rename = "team")]
    pub team: String,
    #[serde(rename = "damage")]
    pub damage: f64,
    #[serde(rename = "speed")]
    pub speed: f64,
    /// Gegnerart des Schützen für crownFallen; ohne Ausgabe.
    #[serde(skip)]
    pub cause: String,
    /// Radius in Units, in dem der Einschlag alle Ziele trifft; 0 = nur das Ziel.
    #[serde(rename = "splash", skip_serializing_if = "is_zero_f64")]
    pub splash: f64,
}

/// Eine Bodenfläche (Flammenspur): `damage_per_second` an Spielern, Truppen und Gebäuden im Radius um `x`, noch
/// `seconds_left` Sekunden.
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
#[serde(default)]
pub struct Hazard {
    #[serde(rename = "x")]
    pub x: f64,
    #[serde(rename = "radius")]
    pub radius: f64,
    #[serde(rename = "damagePerSecond")]
    pub damage_per_second: f64,
    #[serde(rename = "secondsLeft")]
    pub seconds_left: f64,
    /// Boss-ID für crownFallen; ohne Ausgabe.
    #[serde(skip)]
    pub cause: String,
    /// Sekunden bis zum nächsten Schaden; ohne JSON.
    #[serde(skip)]
    pub tick_in: f64,
}

/// Eine Truhe oder ein Skill-Punkt.
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
#[serde(default)]
pub struct Pickup {
    #[serde(rename = "id")]
    pub id: i64,
    #[serde(rename = "kind")]
    pub kind: String,
    #[serde(rename = "x")]
    pub x: f64,
}

/// Ein Rekrutierungs-Camp.
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
#[serde(default)]
pub struct Camp {
    #[serde(rename = "x")]
    pub x: f64,
    #[serde(rename = "respawnIn")]
    pub respawn_in: f64,
}

/// Ein geplanter Gegner einer Welle.
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
#[serde(default)]
pub struct QueuedSpawn {
    #[serde(rename = "kind")]
    pub kind: String,
    #[serde(rename = "x")]
    pub x: f64,
    #[serde(rename = "at")]
    pub at: f64,
    /// Grad-Faktoren der Insel zum Wellenstart; ein Gradwechsel ändert wartende Gegner nicht. Ohne JSON.
    #[serde(skip)]
    pub hp_factor: f64,
    #[serde(skip)]
    pub damage_factor: f64,
}

/// Alle Spieler stehen an einem Tiefen-Eingang oder einer Treppe, Fortschritt 0..1.
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
#[serde(default)]
pub struct Travel {
    #[serde(rename = "x")]
    pub x: f64,
    #[serde(rename = "toDepth")]
    pub to_depth: i64,
    /// exit, stairsUp, stairsDown.
    #[serde(rename = "via")]
    pub via: String,
    #[serde(rename = "progress")]
    pub progress: f64,
}
