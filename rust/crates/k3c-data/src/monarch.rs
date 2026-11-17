//! data/monarch.json: Go `sim.monarchData` (engine/sim/monarch_data.go, `loadMonarch`, `checkPresets`).

use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};

use crate::{Fehler, decode};

/// Go: `sim.monarchData`.
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
#[serde(default)]
pub struct MonarchData {
    #[serde(rename = "Base", alias = "base")]
    pub base: MonarchDataBase,
    #[serde(rename = "SprintMultiplier", alias = "sprintMultiplier")]
    pub sprint_multiplier: f64,
    #[serde(rename = "Acceleration", alias = "acceleration")]
    pub acceleration: f64,
    #[serde(rename = "HitCoins", alias = "hitCoins")]
    pub hit_coins: i64,
    #[serde(rename = "Crown", alias = "crown")]
    pub crown: CrownData,
    #[serde(rename = "Mount", alias = "mount")]
    pub mount: MountData,
    #[serde(rename = "Attack", alias = "attack")]
    pub attack: MonarchDataAttack,
    #[serde(rename = "TierPoints", alias = "tierPoints")]
    pub tier_points: Vec<i64>,
    #[serde(rename = "SkillPointSources", alias = "skillPointSources")]
    pub skill_point_sources: MonarchDataSkillPointSources,
    #[serde(rename = "Lines", alias = "lines")]
    pub lines: BTreeMap<String, MonarchDataLines>,
    #[serde(rename = "Skills", alias = "skills")]
    pub skills: BTreeMap<String, SkillData>,
    #[serde(rename = "Presets", alias = "presets")]
    pub presets: BTreeMap<String, PresetData>,
}

/// Go: `struct { Damage float64; Speed float64 }`.
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
#[serde(default)]
pub struct MonarchDataBase {
    #[serde(rename = "Damage", alias = "damage")]
    pub damage: f64,
    #[serde(rename = "Speed", alias = "speed")]
    pub speed: f64,
}

/// Go: `sim.CrownData`.
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
#[serde(default)]
pub struct CrownData {
    #[serde(rename = "DistanceUnits", alias = "distanceUnits")]
    pub distance_units: f64,
    #[serde(rename = "HeightUnits", alias = "heightUnits")]
    pub height_units: f64,
    #[serde(rename = "FlightSeconds", alias = "flightSeconds")]
    pub flight_seconds: f64,
    #[serde(rename = "PickupUnits", alias = "pickupUnits")]
    pub pickup_units: f64,
    #[serde(rename = "GrabUnits", alias = "grabUnits")]
    pub grab_units: f64,
    #[serde(rename = "BlockSeconds", alias = "blockSeconds")]
    pub block_seconds: f64,
    #[serde(rename = "CoinOffsetUnits", alias = "coinOffsetUnits")]
    pub coin_offset_units: f64,
    #[serde(rename = "CoinStepUnits", alias = "coinStepUnits")]
    pub coin_step_units: f64,
}

/// Go: `sim.MountData`.
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
#[serde(default)]
pub struct MountData {
    #[serde(rename = "ID", alias = "id")]
    pub id: String,
    #[serde(rename = "Sprite", alias = "sprite")]
    pub sprite: String,
    #[serde(rename = "SpeedFactor", alias = "speedFactor")]
    pub speed_factor: f64,
    #[serde(rename = "SprintFactor", alias = "sprintFactor")]
    pub sprint_factor: f64,
}

/// Go: `struct { Damage float64; Range float64; Cooldown float64 }`.
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
#[serde(default)]
pub struct MonarchDataAttack {
    #[serde(rename = "Damage", alias = "damage")]
    pub damage: f64,
    #[serde(rename = "Range", alias = "range")]
    pub range: f64,
    #[serde(rename = "Cooldown", alias = "cooldown")]
    pub cooldown: f64,
}

/// Go: `struct { Hidden int; Miniboss int; Endboss int; Milestone int; ChestEvery int }`.
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
#[serde(default)]
pub struct MonarchDataSkillPointSources {
    #[serde(rename = "Hidden", alias = "hidden")]
    pub hidden: i64,
    #[serde(rename = "Miniboss", alias = "miniboss")]
    pub miniboss: i64,
    #[serde(rename = "Endboss", alias = "endboss")]
    pub endboss: i64,
    #[serde(rename = "Milestone", alias = "milestone")]
    pub milestone: i64,
    #[serde(rename = "ChestEvery", alias = "chestEvery")]
    pub chest_every: i64,
}

/// Go: `struct { Name string }`.
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
#[serde(default)]
pub struct MonarchDataLines {
    #[serde(rename = "Name", alias = "name")]
    pub name: String,
}

/// Go: `sim.SkillData`.
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
#[serde(default)]
pub struct SkillData {
    #[serde(rename = "ID", alias = "iD")]
    pub id: String,
    #[serde(rename = "Line", alias = "line")]
    pub line: String,
    #[serde(rename = "Kind", alias = "kind")]
    pub kind: String,
    #[serde(rename = "Requires", alias = "requires")]
    pub requires: String,
    #[serde(rename = "Tier", alias = "tier")]
    pub tier: i64,
    #[serde(rename = "Cooldown", alias = "cooldown")]
    pub cooldown: f64,
    #[serde(rename = "Effect", alias = "effect")]
    pub effect: SkillEffect,
    #[serde(rename = "Provisional", alias = "provisional")]
    pub provisional: bool,
}

/// Go: `sim.skillEffect`.
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
#[serde(default)]
pub struct SkillEffect {
    #[serde(rename = "Type", alias = "type")]
    pub r#type: String,
    #[serde(rename = "Radius", alias = "radius")]
    pub radius: f64,
    #[serde(rename = "Range", alias = "range")]
    pub range: f64,
    #[serde(rename = "Duration", alias = "duration")]
    pub duration: f64,
    #[serde(rename = "HP", alias = "hp")]
    pub hp: f64,
    #[serde(rename = "Damage", alias = "damage")]
    pub damage: f64,
    #[serde(rename = "Factor", alias = "factor")]
    pub factor: f64,
    #[serde(rename = "PerSecond", alias = "perSecond")]
    pub per_second: f64,
    #[serde(rename = "Amount", alias = "amount")]
    pub amount: f64,
    #[serde(rename = "BackDamage", alias = "backDamage")]
    pub back_damage: f64,
    #[serde(rename = "Defense", alias = "defense")]
    pub defense: f64,
    #[serde(rename = "Aura", alias = "aura")]
    pub aura: f64,
    #[serde(rename = "Reduction", alias = "reduction")]
    pub reduction: f64,
    #[serde(rename = "Chance", alias = "chance")]
    pub chance: f64,
    #[serde(rename = "SpeedBonus", alias = "speedBonus")]
    pub speed_bonus: f64,
    #[serde(rename = "DamageBonus", alias = "damageBonus")]
    pub damage_bonus: f64,
    #[serde(rename = "SpellBonus", alias = "spellBonus")]
    pub spell_bonus: f64,
    #[serde(rename = "CooldownBonus", alias = "cooldownBonus")]
    pub cooldown_bonus: f64,
    #[serde(rename = "Castle", alias = "castle")]
    pub castle: bool,
    #[serde(rename = "FreeHitEvery", alias = "freeHitEvery")]
    pub free_hit_every: i64,
    #[serde(rename = "Size", alias = "size")]
    pub size: i64,
    #[serde(rename = "MaxGold", alias = "maxGold")]
    pub max_gold: i64,
}

/// Go: `sim.presetData`.
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
#[serde(default)]
pub struct PresetData {
    #[serde(rename = "Name", alias = "name")]
    pub name: String,
    #[serde(rename = "ActiveSkills", alias = "activeSkills")]
    pub active_skills: Vec<String>,
}

/// `FILE` ist der Name unter data/.
pub const FILE: &str = "monarch.json";

/// `load` liest monarch.json und prüft die Presets wie `loadMonarch`: Jedes Preset nennt in activeSkills nur Skills
/// aus skills (B-504), Presets sortiert.
pub fn load(raw: &str) -> Result<MonarchData, Fehler> {
    let m: MonarchData = decode("data/monarch.json", raw)?;
    for (id, p) in &m.presets {
        for s in &p.active_skills {
            if !m.skills.contains_key(s) {
                return Err(Fehler::Pruefung(format!(
                    "data/monarch.json: preset {id} nennt unbekannten Skill {s}"
                )));
            }
        }
    }
    Ok(m)
}
