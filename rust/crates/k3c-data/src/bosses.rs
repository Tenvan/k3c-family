//! data/bosses.json: Go `sim.BossData` (engine/sim/boss_data.go); Minibosse und Endbosse als Listen.

use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};

use crate::enemies::EnemyData;
use crate::{Fehler, decode};

/// Go: `struct { Bosses []sim.BossData; EndBosses []sim.BossData "json:\"endBosses\"" }`.
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
#[serde(default)]
pub struct BossesFile {
    #[serde(rename = "Bosses", alias = "bosses")]
    pub bosses: Vec<BossData>,
    #[serde(rename = "endBosses")]
    pub end_bosses: Vec<BossData>,
}

/// Go: `sim.BossData`.
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
#[serde(default)]
pub struct BossData {
    #[serde(rename = "ID", alias = "id")]
    pub id: String,
    #[serde(rename = "Name", alias = "name")]
    pub name: String,
    #[serde(rename = "Kind", alias = "kind")]
    pub kind: String,
    #[serde(rename = "Depth", alias = "depth")]
    pub depth: i64,
    #[serde(rename = "Wave", alias = "wave")]
    pub wave: i64,
    #[serde(rename = "Base", alias = "base")]
    pub base: String,
    #[serde(rename = "HPFactor", alias = "hpFactor")]
    pub hp_factor: f64,
    #[serde(rename = "DamageFactor", alias = "damageFactor")]
    pub damage_factor: f64,
    #[serde(rename = "Speed", alias = "speed")]
    pub speed: f64,
    #[serde(rename = "Range", alias = "range")]
    pub range: f64,
    #[serde(rename = "Ability", alias = "ability")]
    pub ability: BossAbility,
    #[serde(rename = "Reward", alias = "reward")]
    pub reward: BossDataReward,
    #[serde(rename = "Lair", alias = "lair")]
    pub lair: BossDataLair,
    #[serde(rename = "Phases", alias = "phases")]
    pub phases: Vec<BossPhase>,
}

/// Go: `sim.BossAbility`.
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
#[serde(default)]
pub struct BossAbility {
    #[serde(rename = "Type", alias = "type")]
    pub r#type: String,
    #[serde(rename = "Enemy", alias = "enemy")]
    pub enemy: String,
    #[serde(rename = "Count", alias = "count")]
    pub count: i64,
    #[serde(rename = "Radius", alias = "radius")]
    pub radius: f64,
    #[serde(rename = "IntervalSeconds", alias = "intervalSeconds")]
    pub interval_seconds: f64,
    #[serde(rename = "DamagePerSecond", alias = "damagePerSecond")]
    pub damage_per_second: f64,
    #[serde(rename = "DurationSeconds", alias = "durationSeconds")]
    pub duration_seconds: f64,
}

/// Go: `struct { Gold int; Material int }`.
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
#[serde(default)]
pub struct BossDataReward {
    #[serde(rename = "Gold", alias = "gold")]
    pub gold: i64,
    #[serde(rename = "Material", alias = "material")]
    pub material: i64,
}

/// Go: `struct { TriggerUnits float64; NoticeUnits float64; EscortUnits float64 }`.
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
#[serde(default)]
pub struct BossDataLair {
    #[serde(rename = "TriggerUnits", alias = "triggerUnits")]
    pub trigger_units: f64,
    #[serde(rename = "NoticeUnits", alias = "noticeUnits")]
    pub notice_units: f64,
    #[serde(rename = "EscortUnits", alias = "escortUnits")]
    pub escort_units: f64,
}

/// Go: `sim.BossPhase`.
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
#[serde(default)]
pub struct BossPhase {
    #[serde(rename = "BelowHPPercent", alias = "belowHpPercent")]
    pub below_hp_percent: f64,
    #[serde(rename = "Ability", alias = "ability")]
    pub ability: Option<BossAbility>,
    #[serde(rename = "Tempo", alias = "tempo")]
    pub tempo: f64,
}

/// `FILE` ist der Name unter data/.
pub const FILE: &str = "bosses.json";

/// `load` liest bosses.json und prüft beide Listen wie `loadBosses` und `loadEndBosses`. Wie in Go braucht die
/// Prüfung die Gegner (enemies.json) und die Tiefen der Biome (`hasDepth`).
pub fn load(raw: &str, enemies: &BTreeMap<String, EnemyData>, depths: &[i64]) -> Result<BossesFile, Fehler> {
    let v: BossesFile = decode("data/bosses.json", raw)?;
    let refs = Refs { enemies, depths };
    check_boss_list(&v.bosses, "mini", &refs)?;
    check_boss_list(&v.end_bosses, "end", &refs)?;
    Ok(v)
}

struct Refs<'a> {
    enemies: &'a BTreeMap<String, EnemyData>,
    depths: &'a [i64],
}

fn check_boss_list(list: &[BossData], kind: &str, refs: &Refs) -> Result<(), Fehler> {
    let fail = |s: String| Err(Fehler::Pruefung(format!("data/bosses.json: {s}")));
    for (i, b) in list.iter().enumerate() {
        if b.kind != kind {
            return fail(format!("{}: Art {:?} in der Liste für {kind:?}", b.id, b.kind));
        }
        if let Err(e) = check_boss(b, refs) {
            return fail(format!("{}: {e}", b.id));
        }
        if list[..i].iter().any(|o| o.id == b.id) {
            return fail(format!("{} doppelt", b.id));
        }
    }
    Ok(())
}

fn check_boss(b: &BossData, refs: &Refs) -> Result<(), String> {
    if b.id.is_empty() {
        return Err("ohne id".into());
    }
    if b.kind != "mini" && b.kind != "end" {
        return Err(format!("unbekannte Art {:?}", b.kind));
    }
    if !refs.depths.contains(&b.depth) {
        return Err(format!("unbekannte Stufe {}", b.depth));
    }
    if !refs.enemies.contains_key(&b.base) {
        return Err(format!("unbekannter Bezug {:?}", b.base));
    }
    if b.kind == "mini" && b.wave <= 0 {
        return Err("ohne Welle (Miniboss)".into());
    }
    if b.hp_factor <= 0.0 || b.damage_factor < 0.0 || b.speed < 0.0 || b.range <= 0.0 {
        return Err("ungültige Werte".into());
    }
    if b.reward.gold < 0 || b.reward.material < 0 {
        return Err("negative Belohnung".into());
    }
    if b.kind == "end" {
        return check_phases(b, refs);
    }
    check_ability(&b.ability, refs)
}

/// `check_phases`: Ein Endboss braucht einen Bau mit Auslöse-Abstand und Phasen, deren erste eine Fähigkeit hat und
/// deren HP-Schwellen streng fallen (zwischen 0 und 100 %).
fn check_phases(b: &BossData, refs: &Refs) -> Result<(), String> {
    if b.lair.trigger_units <= 0.0 || b.phases.is_empty() || b.phases[0].ability.is_none() {
        return Err("endboss braucht lair.triggerUnits > 0 und eine erste Phase mit Fähigkeit".into());
    }
    if b.lair.notice_units < b.lair.trigger_units {
        return Err("lair.noticeUnits darf nicht unter triggerUnits liegen".into());
    }
    let mut below = 100.0;
    for (i, p) in b.phases.iter().enumerate() {
        if i > 0 && (p.below_hp_percent <= 0.0 || p.below_hp_percent >= below) {
            return Err(format!("phase {}: belowHpPercent muss fallen und über 0 liegen", i + 1));
        }
        if i > 0 {
            below = p.below_hp_percent;
        }
        if p.tempo < 0.0 {
            return Err(format!("phase {}: negatives Tempo", i + 1));
        }
        if let Some(a) = &p.ability {
            check_ability(a, refs).map_err(|e| format!("phase {}: {e}", i + 1))?;
        }
    }
    Ok(())
}

fn check_ability(a: &BossAbility, refs: &Refs) -> Result<(), String> {
    let known = refs.enemies.contains_key(&a.enemy);
    let t = a.r#type.as_str();
    if a.interval_seconds <= 0.0 {
        return Err("ohne Intervall der Fähigkeit".into());
    }
    if t == "summon" && (!known || a.count <= 0) {
        return Err("summon braucht bekannten Gegner und Anzahl > 0".into());
    }
    if (t == "aoe" || t == "shardThrow") && a.radius <= 0.0 {
        return Err(format!("{t} ohne Radius"));
    }
    if t == "flameTrail" && (a.radius <= 0.0 || a.damage_per_second <= 0.0 || a.duration_seconds <= 0.0) {
        return Err("flameTrail braucht Radius, Schaden und Dauer > 0".into());
    }
    if !["summon", "aoe", "flameTrail", "shardThrow"].contains(&t) {
        return Err(format!("unbekannte Fähigkeit {:?}", a.r#type));
    }
    Ok(())
}
