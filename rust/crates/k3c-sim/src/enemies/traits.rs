//! Traits der Gegner (Port von `engine/sim/enemies_traits.go`, gegner.md § 2): Flächenschlag (`aoe`), Phasen (`phases`),
//! Kiting (`kiting`) und die Angriffsrate je Gegner; der Schwarm (`swarm`) entsteht beim Planen der Welle (`waves.rs`).
//! Parameter stehen in data/enemies.json. Die Daten-Lader (`knownTraits`, `loadEnemyData`, `loadEnemies`) liegen seit
//! RU3 in `k3c_data::enemies`.

use serde_json::json;

use super::target::{Target, TargetKind, attack};
use super::{BODY, is_barrier};
use crate::state::common::{apply_damage_by, approach, event, shielded, sign};
use crate::state::island_ref::IslandRef;
use crate::state::links::{crowned, emit, frost_armor_hit, is_on_tower, unit_x, values};
use crate::state::{Enemy, World, WorldValues};

/// Go: `shipKind` (ship.go).
const SHIP_KIND: &str = "ship";

/// Go: `attackRate`: Angriffe je Sekunde laut Daten, ohne Eintrag 1.
pub fn attack_rate(v: &WorldValues, kind: &str) -> f64 {
    match v.enemies.get(kind) {
        Some(d) if d.attacks_per_second > 0.0 => d.attacks_per_second,
        _ => 1.0,
    }
}

/// Go: `strike`: Ist der Flächenschlag fällig (`aoe`), trifft er alle Ziele im Radius um das gewählte Ziel je einmal;
/// sonst der normale Angriff. Beide teilen die Abklingzeit der Angriffsrate.
pub fn strike(w: &mut World, v: &WorldValues, ei: usize, t: &Target, isl: Option<&IslandRef>) {
    if !w.enemies[ei].has("aoe") || w.enemies[ei].aoe_in > 0.0 {
        attack(w, v, ei, t, isl);
        return;
    }
    let (radius, interval) = v
        .enemies
        .get(w.enemies[ei].kind.as_str())
        .map_or((0.0, 0.0), |d| (d.aoe.radius, d.aoe.interval_seconds));
    let e = &mut w.enemies[ei];
    e.cooldown = 1.0 / attack_rate(v, &e.kind);
    e.aoe_in = interval;
    let (id, x, damage) = (e.id, e.x, e.damage);
    emit(w, "strike", event(json!({"from": id, "x": unit_x(x)})));
    let targets = aoe_targets(w, v, t.x, radius); // alle zehn Sekunden oder seltener, nie je Takt
    let mut kind = std::mem::take(&mut w.enemies[ei].kind); // ohne Kopie als Ursache; danach zurück
    for c in &targets {
        apply_damage_by(w, c.id, damage, &kind, isl);
        if let Some(pi) = c.player {
            std::mem::swap(&mut w.enemies[ei].kind, &mut kind); // der Gegner braucht seine Art kurz zurück
            frost_armor_hit(w, pi, ei);
            std::mem::swap(&mut w.enemies[ei].kind, &mut kind);
        }
    }
    w.enemies[ei].kind = kind;
}

/// Go: `aoeTargets`: Spieler, Truppen (ohne Landstreicher und Schützen auf Türmen), gebaute Plätze und die Burg im Radius
/// um `x`.
pub(crate) fn aoe_targets(w: &World, v: &WorldValues, x: f64, radius: f64) -> Vec<Target> {
    let within = |at: f64, extra: f64| (at - x).abs() <= radius + extra;
    let mut out = Vec::new();
    for (pi, p) in w.players.iter().enumerate() {
        if crowned(p) && !shielded(p) && within(p.x, 0.0) {
            out.push(Target {
                id: p.id,
                x: p.x,
                kind: TargetKind::Player,
                player: Some(pi),
            });
        }
    }
    for t in &w.troops {
        if t.kind != "vagrant" && !is_on_tower(w, t) && within(t.x, 0.0) {
            out.push(Target {
                id: t.id,
                x: t.x,
                kind: TargetKind::Troop,
                player: None,
            });
        }
    }
    for s in &w.sites {
        if s.state == "built" && s.kind != SHIP_KIND && within(s.x, 0.0) {
            out.push(Target {
                id: s.id,
                x: s.x,
                kind: TargetKind::Site,
                player: None,
            });
        }
    }
    if within(w.castle.x, v.hub.castle_radius_units) {
        out.push(Target {
            id: w.castle.id,
            x: w.castle.x,
            kind: TargetKind::Castle,
            player: None,
        });
    }
    out
}

/// Go: `invulnerable`: Ein Gegner mit `phases` ist ab Spawn alle `everySeconds` für `durationSeconds` unverwundbar.
pub fn invulnerable(w: &World, e: &Enemy) -> bool {
    invulnerable_in(&values(w), w.time, e)
}

/// `invulnerable` mit bereits geholtem Wertesatz und der Spielzeit.
pub fn invulnerable_in(v: &WorldValues, time: f64, e: &Enemy) -> bool {
    let Some(d) = v.enemies.get(e.kind.as_str()) else {
        return false;
    };
    let p = &d.phases;
    if !e.has("phases") || p.every_seconds <= 0.0 {
        return false;
    }
    let age = time - e.spawned;
    age >= p.every_seconds && age % p.every_seconds < p.duration_seconds
}

/// Go: `kite`: Ein Gegner mit `kiting` hält `kiteDistance` Abstand zu Spielern und Truppen. Ist einer näher, weicht er
/// Richtung Portal aus, nie über `home_x` hinaus und nie durch eine Mauer; an Mauer oder Portal bleibt er stehen.
/// `true`: Ein Ziel ist zu nah, der Gegner rückt nicht vor.
pub fn kite(w: &mut World, v: &WorldValues, ei: usize, dt: f64) -> bool {
    let e = &w.enemies[ei];
    if !e.has("kiting") {
        return false;
    }
    let distance = v.enemies.get(e.kind.as_str()).map_or(0.0, |d| d.kite_distance);
    let Some(threat) = nearest_threat(w, e) else {
        return false;
    };
    if (threat - e.x).abs() >= distance {
        return false;
    }
    let back = sign(e.home_x - e.x);
    if back == 0.0 || sign(e.x - threat) == -back {
        return true; // am Portal, oder der Weg zurück führt zum Ziel hin
    }
    let mut x = approach(e.x, e.home_x, e.speed() * w.weather_speed, dt);
    for s in &w.sites {
        let stop = s.x - back * BODY;
        if is_barrier(s) && (s.x - e.x) * back > 0.0 && (x - stop) * back > 0.0 {
            x = stop;
        }
    }
    if (x - e.x) * back > 0.0 {
        w.enemies[ei].x = x;
    }
    true
}

/// Go: `nearestThreat`: Ort des nächsten lebenden Spielers oder der nächsten Truppe (ohne Landstreicher).
fn nearest_threat(w: &World, e: &Enemy) -> Option<f64> {
    let mut best: Option<f64> = None;
    let mut closer = |x: f64| {
        if best.is_none_or(|b| (x - e.x).abs() < (b - e.x).abs()) {
            best = Some(x);
        }
    };
    for p in w.players.iter().filter(|p| crowned(p)) {
        closer(p.x);
    }
    for t in w.troops.iter().filter(|t| t.kind != "vagrant") {
        closer(t.x);
    }
    best
}
