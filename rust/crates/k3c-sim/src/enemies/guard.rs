//! Gegnerlager (Port von `engine/sim/enemies_guard.go`, docs/rules/geplante-chunks.md § 9, B-517): Jede Wache der
//! Planung (`Layout::guards`) ist ein Gegner mit Kennung `guard`. Sie steht an ihrem Heimatpunkt (`home_x`), greift
//! Spieler und Truppen im Aggro-Radius um den Heimatpunkt an, kehrt danach zurück und ist besiegbar; sie flieht nie, klaut
//! kein Gold, gehört zu keiner Welle (kein Beitrag zur Gefahr der Bauern, zum Aggressionspool und zum Tagesanbruch) und
//! kommt nach dem Tod nie wieder (`guards_cleared`, Spielstand).

use std::sync::OnceLock;

use k3c_data::{Source, guards};

use super::target::{Target, TargetKind};
use super::traits::strike;
use super::{BODY, spawn_with};
use crate::state::common::{shielded, sign};
use crate::state::island_ref::IslandRef;
use crate::state::links::crowned;
use crate::state::save::{HubSave, key};
use crate::state::{Enemy, World, WorldValues};
use crate::unit::archer::is_on_tower;

/// Go: `level.DefaultGuardRules().AggroRadiusUnits`: der Radius aus den eingebetteten Daten, einmal gelesen. Die
/// eingebetteten Daten prüft `k3c-data`; ein Lesefehler ist dort nicht möglich, der Rückfall 0 hielte die Wachen ruhig.
fn aggro_radius() -> f64 {
    static RADIUS: OnceLock<f64> = OnceLock::new();
    *RADIUS.get_or_init(|| {
        Source::Embedded
            .read(guards::FILE)
            .ok()
            .and_then(|raw| guards::load(&raw).ok())
            .map_or(0.0, |g| g.aggro_radius_units)
    })
}

/// Go: `guardKey`: Kennung einer Wache für Spielstand und Räumung, Art und Heimatpunkt (`kind@x` wie bei Plätzen).
pub fn guard_key(e: &Enemy) -> String {
    key(&e.kind, e.home_x)
}

/// Go: `spawnGuards`: stellt die Wachen der Stufe beim ersten Tick auf, ohne die geräumten. Erst dann stehen Insel und
/// Skalierung fest (Laden setzt sie nach dem Bau der Stufe); ohne Planung gibt es keine Wachen.
pub fn spawn_guards(w: &mut World, v: &WorldValues, isl: Option<&IslandRef>) {
    if w.guards_spawned {
        return;
    }
    w.guards_spawned = true;
    for gi in 0..w.level.guards.len() {
        let (kind, x) = (w.level.guards[gi].kind.clone(), w.level.guards[gi].x); // einmal je Wache beim Aufbau
        if w.guards_cleared.get(&key(&kind, x)).copied().unwrap_or(false) {
            continue;
        }
        let ei = spawn_with(w, v, &kind, x, 1.0, 1.0, isl);
        w.enemies[ei].guard = true;
    }
}

/// Go: `stepGuard`: Die Wache kämpft gegen das nächste Ziel im Aggro-Radius oder kehrt zum Heimatpunkt zurück. Sie
/// nimmt nicht am Marsch der Wellen teil.
pub fn step_guard(w: &mut World, v: &WorldValues, ei: usize, isl: Option<&IslandRef>, dt: f64) {
    w.enemies[ei].fleeing = false;
    let foe = guard_foe(w, &w.enemies[ei]);
    let (weather_speed, weather_range) = (w.weather_speed, w.weather_range); // RU62.5: Taifun (sonst 1.0)
    let e = &mut w.enemies[ei];
    let Some(foe) = foe else {
        let d = e.home_x - e.x;
        e.x += sign(d) * d.abs().min(e.speed() * weather_speed * dt);
        return;
    };
    let dist = foe.x - e.x;
    let range = if e.has("ranged") {
        e.range * weather_range
    } else {
        e.range
    };
    if dist.abs() <= range + BODY {
        if e.cooldown <= 0.0 {
            strike(w, v, ei, &foe, isl);
        }
        return;
    }
    e.x += sign(dist) * dist.abs().min(e.speed() * weather_speed * dt);
}

/// Go: `guardFoe`: das nächste Ziel (Spieler, Truppen ohne Landstreicher) im Aggro-Radius um den Heimatpunkt; `None`
/// ohne Ziel.
fn guard_foe(w: &World, e: &Enemy) -> Option<Target> {
    let radius = aggro_radius();
    let mut best: Option<Target> = None;
    let mut consider = |t: Target| {
        if (t.x - e.home_x).abs() <= radius && best.is_none_or(|b| (t.x - e.x).abs() < (b.x - e.x).abs()) {
            best = Some(t);
        }
    };
    for (pi, p) in w.players.iter().enumerate() {
        if crowned(p) && !shielded(p) {
            consider(Target {
                id: p.id,
                x: p.x,
                kind: TargetKind::Player,
                player: Some(pi),
            });
        }
    }
    let ranged = e.has("ranged");
    for t in &w.troops {
        if t.kind != "vagrant" && (ranged || !is_on_tower(w, t)) {
            consider(Target {
                id: t.id,
                x: t.x,
                kind: TargetKind::Troop,
                player: None,
            });
        }
    }
    best
}

/// Go: `isWaveEnemy`: Gegner, die zu einer Welle gehören (alle außer den Wachen eines Lagers).
pub fn is_wave_enemy(e: &Enemy) -> bool {
    !e.guard
}

/// Go: `keepGuards`: nach dem Burgfall verschwinden die Gegner der Welle, die Wachen bleiben.
pub fn keep_guards(enemies: &mut Vec<Enemy>) {
    enemies.retain(|e| e.guard);
}

/// Go: `markCleared`: trägt eine besiegte Wache als geräumt ein (kein Respawn, auch nicht nach Laden und Burgfall).
pub fn mark_cleared(w: &mut World, e: &Enemy) {
    w.guards_cleared.insert(guard_key(e), true);
}

/// Go: `guardsClearedKeys`: die geräumten Lager für den Spielstand, sortiert (stabile Datei).
pub fn guards_cleared_keys(w: &World) -> Vec<String> {
    w.guards_cleared.keys().cloned().collect()
}

/// Go: `applyGuards`: übernimmt die geräumten Lager aus dem Spielstand und entfernt bereits aufgestellte Wachen dieser
/// Lager.
pub fn apply_guards(w: &mut World, h: &HubSave) {
    w.guards_cleared.clear();
    for k in &h.guards_cleared {
        w.guards_cleared.insert(k.clone(), true);
    }
    let cleared = &w.guards_cleared;
    w.enemies.retain(|e| !(e.guard && cleared.contains_key(&guard_key(e))));
}
