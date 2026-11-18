//! Endboss (Port von `engine/sim/boss_endboss.go`, docs/rules/bosse.md § 1 und § 1.1, K2.1c): Er liegt in seinem Bau
//! (Entität `lair` im Level, nur in Biomen mit `lair`) und wartet ohne Zeitdruck; solange er wartet, gibt es ihn nicht
//! als Gegner, er ist also unverwundbar und tut nichts. Sobald ein lebender Spieler näher als `lair.triggerUnits` am Bau
//! ist, erscheint er dort (`bossSpawned`, HP mit dem Spielerfaktor der Insel in diesem Moment) und bleibt aktiv bis zu
//! seinem Tod, auch über einen Burgfall hinweg. Er läuft nicht (speed 0), sondern kämpft am Bau; Beschwörungen laufen wie
//! jeder Gegner zum Hub. Seine Phasen (`phases`) wechseln an den HP-Schwellen, jeder Wechsel wird einmal gemeldet
//! (`bossPhase`). Der Sieg setzt `IslandRef::endboss_defeated`; danach bleibt der Bau leer. Kein rng außer den Münzen der
//! Belohnung.

use serde_json::json;

use k3c_data::bosses::{BossAbility, BossData};

use super::abilities::arm_boss;
use super::data::boss_by_id;
use super::spawn_boss;
use crate::state::common::event;
use crate::state::island_ref::IslandRef;
use crate::state::links::{crowned, emit};
use crate::state::{Enemy, Player, World, WorldValues};

/// Go: `endbossFight`, der laufende Kampf gegen den Endboss einer Insel. `boss_id` ist die ID des Gegners (Go: Zeiger
/// auf den `Enemy`), `phase` der Index in `BossData::phases`, `home` der Index der Stufe des Kampfs (Go: `*World`;
/// sonst der Bau, mit der Dev-Aktion `boss` die Stufe des Monarchen).
///
/// `parked` ersetzt Gos Zeiger über einen Burgfall hinweg: `castleFallen` räumt die Gegner der Stufe ab, der Endboss
/// bleibt. In Rust legt der Burgfall (RU14) den Gegner mit `park_endboss` hierher, `step_endboss` holt ihn zurück.
#[derive(Debug, Clone, PartialEq, Default)]
pub struct EndbossFight {
    pub boss_id: i64,
    pub phase: usize,
    pub home: usize,
    /// Begleitschutz läuft (unit_escort.go): rastet ein, sobald ein Monarch nahe kam, bis der Boss stirbt.
    pub escort: bool,
    /// Der aus der Stufe abgeräumte Endboss (nach einem Burgfall), bis `step_endboss` ihn zurückstellt.
    pub parked: Option<Enemy>,
}

/// Burgfall (Go: `castleFallen` räumt die Gegner ab, der Endboss bleibt): nimmt den Endboss der Stufe aus `w.enemies` und
/// parkt ihn im Kampf; `step_endboss` stellt ihn wieder auf. Ohne Kampf in dieser Stufe passiert nichts.
pub fn park_endboss(w: &mut World, isl: Option<&mut IslandRef>) {
    let Some(isl) = isl else {
        return;
    };
    let stage = isl.stage;
    let Some(f) = isl.endboss.as_mut().filter(|f| f.home == stage) else {
        return;
    };
    if let Some(i) = w.enemies.iter().position(|e| e.id == f.boss_id) {
        f.parked = Some(w.enemies.remove(i));
    }
}

/// Go: `stepEndboss` (vor `stepEnemies`): Auslösung am Bau, Rückkehr nach einem Burgfall, Phasenwechsel und Tempo.
pub fn step_endboss(w: &mut World, v: &WorldValues, isl: Option<&mut IslandRef>, dt: f64) {
    let Some(isl) = isl else {
        return;
    };
    let stage = isl.stage;
    if let Some(f) = isl.endboss.as_mut() {
        if f.home != stage {
            return;
        }
        if !w.enemies.iter().any(|e| e.id == f.boss_id)
            && let Some(e) = f.parked.take()
        {
            w.enemies.push(e); // castleFallen räumt die Gegner ab, der Endboss bleibt
        }
        endboss_phase(w, v, f, dt);
        escort_check(w, f);
        return;
    }
    let Some(x) = lair_x(w) else {
        return;
    };
    let Some(b) = w.lair_boss.as_ref() else {
        return;
    };
    if isl.endboss_defeated || isl.defeated_bosses.contains(&b.id) || !player_near(w, x, b.lair.trigger_units) {
        return;
    }
    let b = b.clone(); // einmal beim Auslösen
    let ei = spawn_boss(w, v, &b, x, Some(isl));
    if let Some(a) = b.phases.first().and_then(|p| p.ability.as_ref()) {
        arm_boss(&mut w.enemies[ei], a);
    }
    isl.endboss = Some(EndbossFight {
        boss_id: w.enemies[ei].id,
        home: stage,
        ..EndbossFight::default()
    });
}

/// Go: `escortCheck`: rastet den Begleitschutz ein, sobald ein lebender Monarch näher als `lair.escortUnits` am Boss ist.
pub fn escort_check(w: &World, f: &mut EndbossFight) {
    let Some(b) = w.lair_boss.as_ref() else {
        return;
    };
    let Some(boss) = w.enemies.iter().find(|e| e.id == f.boss_id) else {
        return;
    };
    if !f.escort && b.lair.escort_units > 0.0 && player_near(w, boss.x, b.lair.escort_units) {
        f.escort = true;
    }
}

/// Go: `lairX`: Ort des Baus aus dem Level der Stufe (`None`: die Stufe hat keinen).
pub fn lair_x(w: &World) -> Option<f64> {
    w.level.entities.iter().find(|e| e.kind == "lair").map(|e| e.x)
}

/// Go: `AtLair`: Der Endboss der Stufe wartet noch in seinem Bau (nicht ausgelöst, nicht besiegt) und der lebende
/// Spieler ist näher als `lair.noticeUnits` daran; die Aktionsliste nennt dann den Bau als Auslöser (B-563, `lair`).
pub fn at_lair(w: &World, isl: Option<&IslandRef>, p: &Player) -> bool {
    let (Some(isl), Some(b), Some(x)) = (isl, w.lair_boss.as_ref(), lair_x(w)) else {
        return false;
    };
    if isl.endboss.is_some() || isl.endboss_defeated || isl.defeated_bosses.contains(&b.id) {
        return false;
    }
    !p.free && crowned(p) && (p.x - x).abs() < b.lair.notice_units
}

/// Go: `playerNear`: Ein lebender, gesteuerter Spieler ist näher als `dist` an `x`.
pub fn player_near(w: &World, x: f64, dist: f64) -> bool {
    w.players
        .iter()
        .any(|p| !p.free && crowned(p) && (p.x - x).abs() < dist)
}

/// Go: `endbossPhase`: wechselt in jede Phase, deren HP-Schwelle unterschritten ist (je einmal, mit Meldung), stellt ihre
/// Fähigkeit ein und lässt bei Tempo > 1 Abklingzeit und Fähigkeit entsprechend schneller ablaufen (`stepEnemies` zieht
/// `dt` ab, hier kommt der Rest dazu).
pub(crate) fn endboss_phase(w: &mut World, v: &WorldValues, f: &mut EndbossFight, dt: f64) {
    let Some(ei) = w.enemies.iter().position(|e| e.id == f.boss_id) else {
        return;
    };
    let Some(b) = boss_by_id(&v.bosses, &w.enemies[ei].kind) else {
        return;
    };
    while f.phase + 1 < b.phases.len() && {
        let e = &w.enemies[ei];
        100.0 * e.hp < b.phases[f.phase + 1].below_hp_percent * e.max_hp
    } {
        f.phase += 1;
        let p = &b.phases[f.phase];
        if let Some(a) = &p.ability {
            arm_boss(&mut w.enemies[ei], a);
        }
        if p.tempo > 0.0 {
            w.enemies[ei].speed *= p.tempo;
        }
        emit(w, "bossPhase", event(json!({"boss": b.id, "phase": f.phase + 1})));
    }
    let t = phase_tempo(b, f.phase);
    if t > 1.0 {
        let extra = (t - 1.0) * dt;
        let e = &mut w.enemies[ei];
        e.cooldown = (e.cooldown - extra).max(0.0);
        e.aoe_in = (e.aoe_in - extra).max(0.0);
    }
}

/// Go: `phaseTempo`: Tempo der Phase, ohne Angabe das der Phase davor (1: normal).
pub fn phase_tempo(b: &BossData, phase: usize) -> f64 {
    let last = phase.min(b.phases.len().saturating_sub(1));
    b.phases
        .get(..=last)
        .and_then(|ps| ps.iter().rev().find(|p| p.tempo > 0.0))
        .map_or(1.0, |p| p.tempo)
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
