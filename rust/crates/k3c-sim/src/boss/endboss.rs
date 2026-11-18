//! Endboss (Port von `engine/sim/boss_endboss.go`, docs/rules/bosse.md § 1 und § 1.1, K2.1c). RU15.1 legt den
//! Kampf-Zustand und die Abfrage der aktuellen Fähigkeit an, die `boss/mod.rs` braucht; RU15.2 ergänzt den Rest.

use k3c_data::bosses::{BossAbility, BossData};

use crate::state::island_ref::IslandRef;

/// Go: `endbossFight`, der laufende Kampf gegen den Endboss einer Insel. `boss_id` ist die ID des Gegners (Go: Zeiger
/// auf den `Enemy`), `phase` der Index in `BossData::phases`, `home` der Index der Stufe des Kampfs (Go: `*World`;
/// sonst der Bau, mit der Dev-Aktion `boss` die Stufe des Monarchen).
#[derive(Debug, Clone, PartialEq, Default)]
pub struct EndbossFight {
    pub boss_id: i64,
    pub phase: usize,
    pub home: usize,
    /// Begleitschutz läuft (unit_escort.go): rastet ein, sobald ein Monarch nahe kam, bis der Boss stirbt.
    pub escort: bool,
}

/// Go: `endbossDefeated`: Merker für Siegvariante und Inselwechsel; der Kampf ist vorbei.
pub fn endboss_defeated(b: &BossData, isl: Option<&mut IslandRef>) {
    if let Some(i) = isl
        && b.kind == "end"
    {
        i.endboss_defeated = true;
        i.endboss = None;
    }
}

/// Go: `bossAbilityOf`: die Fähigkeit, die der Boss gerade nutzt: beim Endboss die der aktuellen Phase (ohne eigene die
/// der Phase davor), sonst seine einzige.
pub fn boss_ability_of<'a>(b: &'a BossData, isl: Option<&IslandRef>) -> &'a BossAbility {
    let Some(fight) = isl.and_then(|i| i.endboss.as_ref()).filter(|_| b.kind == "end") else {
        return &b.ability;
    };
    let last = fight.phase.min(b.phases.len().saturating_sub(1));
    b.phases
        .get(..=last)
        .and_then(|ps| ps.iter().rev().find_map(|p| p.ability.as_ref()))
        .unwrap_or(&b.ability)
}
