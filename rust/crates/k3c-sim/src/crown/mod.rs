//! Krone (Port von `engine/sim/crown.go`, docs/rules/kampf.md §§ 5 und 6, B-930, B-931): Der Monarch hat keine HP. Jeder
//! Treffer kostet Münzen, ohne Gold fällt die Krone in hohem Bogen nach hinten, weg vom nächsten Gegner (deterministisch,
//! ohne Würfel). Ohne Krone kann der Monarch nur laufen und Kronen aufheben. Gegner heben die Krone auf und tragen sie ins
//! Portal (Kronenraub = Game Over); ohne Portale in der Stufe lassen sie sie liegen. Ein Mitspieler trägt eine fremde
//! Krone zu ihrem Besitzer zurück. Eine Krone je Monarch.
//!
//! Die gefallene Krone liegt in `Player::crown`. Trägt ein Gegner sie, hält `Enemy::crown` eine Kopie (Go teilt den
//! Zeiger): `follow_enemy` und `drop_crown_of` halten beide gleich. Go-Zeiger auf Spieler sind Indizes (`pi`); `owner`
//! der Krone ist `Player::index`. Funktionen, die Go über `w.values()` den Wertesatz holen lassen, gibt es mit und ohne
//! `_with` (wie in `boss/mod.rs`).

use serde_json::json;

use k3c_data::monarch::CrownData;

use crate::island::defeat::game_over;
use crate::state::common::{event, sign};
use crate::state::island_ref::{IslandRef, protected_night};
use crate::state::pending::{emit, unit_x, values};
use crate::state::shared::Crown;
use crate::state::{Coin, Enemy, World, WorldValues};

/// Go: `CrownFlying`: fliegt im Bogen von `from_x` nach `to_x`, nicht aufhebbar.
pub const CROWN_FLYING: &str = "flying";
/// Go: `CrownGround`: liegt am Boden und ist aufhebbar.
pub const CROWN_GROUND: &str = "ground";
/// Go: `CrownEnemy`: ein Gegner trägt sie zum Portal (`carrier` = Gegner-ID).
pub const CROWN_ENEMY: &str = "enemy";
/// Go: `CrownPlayer`: ein Mitspieler trägt sie zum Besitzer (`carrier` = Spieler-ID).
pub const CROWN_PLAYER: &str = "player";

/// Go: `crowned`: Der Monarch trägt seine Krone und darf handeln (Schlag, Skills, Zahlen, Bauen); ohne Krone nur laufen
/// und aufheben. Der Name sagt „hat die Krone auf“: Go prüft `p.Crown == nil` (die gefallene Krone ist `Some`).
pub fn crowned(p: &crate::state::Player) -> bool {
    p.crown.is_none()
}

/// Go: `crownDirection`: Richtung der Flugbahn, vom nächsten lebenden Gegner weg; ohne Gegner entgegen der Blickrichtung.
pub fn crown_direction(w: &World, pi: usize) -> f64 {
    let p = &w.players[pi];
    let mut near: Option<&Enemy> = None;
    for e in &w.enemies {
        if e.hp > 0.0 && near.is_none_or(|n| (e.x - p.x).abs() < (n.x - p.x).abs()) {
            near = Some(e);
        }
    }
    if let Some(n) = near
        && n.x != p.x
    {
        return sign(p.x - n.x);
    }
    if p.facing != 0 {
        return -(p.facing as f64);
    }
    -1.0
}

/// Go: `dropCrown`: lässt die Krone von `pi` in hohem Bogen nach hinten fallen (Ereignis `crownFallen`; `cause`: Gegnerart
/// des Treffers, B-182).
pub fn drop_crown(w: &mut World, pi: usize, cause: &str) {
    let v = values(w);
    drop_crown_with(w, &v, pi, cause);
}

/// `drop_crown` mit bereits geholtem Wertesatz.
pub fn drop_crown_with(w: &mut World, v: &WorldValues, pi: usize, cause: &str) {
    let c = &v.monarch.crown;
    let dir = crown_direction(w, pi);
    let p = &mut w.players[pi];
    let to = w.width_units.min((p.x + dir * c.distance_units).max(0.0));
    p.crown = Some(Crown {
        owner: p.index,
        state: CROWN_FLYING.into(),
        x: p.x,
        from_x: p.x,
        to_x: to,
        ..Crown::default()
    });
    p.vx = 0.0;
    let (index, x) = (p.index, p.x);
    emit(
        w,
        "crownFallen",
        event(json!({"player": index, "cause": cause, "x": unit_x(x), "toX": unit_x(to)})),
    );
}

/// Zustand einer gefallenen Krone als Kopie ohne Text (Go: `switch c.State`).
#[derive(Clone, Copy, PartialEq)]
enum Phase {
    Flying,
    Ground,
    Enemy,
    Player,
    Other,
}

fn phase_of(c: &Crown) -> Phase {
    match c.state.as_str() {
        CROWN_FLYING => Phase::Flying,
        CROWN_GROUND => Phase::Ground,
        CROWN_ENEMY => Phase::Enemy,
        CROWN_PLAYER => Phase::Player,
        _ => Phase::Other,
    }
}

/// Go: `stepCrowns` (je Tick nach `stepPlayers`): Flug, Mitnehmen durch Träger, Aufheben durch Monarchen.
pub fn step_crowns(w: &mut World, v: &WorldValues, dt: f64) {
    for oi in 0..w.players.len() {
        let Some(phase) = w.players[oi].crown.as_ref().map(phase_of) else {
            continue;
        };
        if w.players[oi].free && phase != Phase::Enemy {
            // abwesender Spieler: die Krone sitzt wieder auf, kein Raub (B-931)
            restore_crown(w, oi, oi);
            continue;
        }
        match phase {
            Phase::Flying => fly_crown(w, &v.monarch.crown, oi, dt),
            Phase::Ground => pick_up_crown(w, &v.monarch.crown, oi),
            Phase::Enemy => follow_enemy(w, oi),
            Phase::Player => follow_bearer(w, &v.monarch.crown, oi),
            Phase::Other => {}
        }
    }
}

/// Go: `flyCrown`: Parabel von `from_x` nach `to_x` in `flightSeconds`, am Ende liegt die Krone am Boden.
fn fly_crown(w: &mut World, d: &CrownData, oi: usize, dt: f64) {
    let Some(c) = w.players[oi].crown.as_mut() else {
        return;
    };
    c.t += dt;
    let f = if d.flight_seconds > 0.0 {
        (c.t / d.flight_seconds).min(1.0)
    } else {
        1.0
    };
    c.x = c.from_x + (c.to_x - c.from_x) * f;
    c.z = 4.0 * d.height_units * f * (1.0 - f);
    if f >= 1.0 {
        c.state = CROWN_GROUND.into();
        c.z = 0.0;
        c.t = 0.0;
        let (owner, x) = (c.owner, c.x);
        emit(w, "crownLanded", event(json!({"player": owner, "x": unit_x(x)})));
    }
}

/// Go: `bearing`: Trägt der Monarch `qi` bereits eine fremde Krone?
fn bearing(w: &World, qi: usize) -> bool {
    let id = w.players[qi].id;
    w.players.iter().any(|o| {
        o.crown
            .as_ref()
            .is_some_and(|c| c.state == CROWN_PLAYER && c.carrier == id)
    })
}

/// Go: `pickUpCrown`: Der erste Monarch (nach Index) in Reichweite hebt die Krone auf. Der Besitzer setzt sie sich auf;
/// ein Mitspieler trägt sie zum Besitzer (Annahme: kein Zähler, Ankunft in Aufhebe-Reichweite genügt, `follow_bearer`).
fn pick_up_crown(w: &mut World, d: &CrownData, oi: usize) {
    let Some(cx) = w.players[oi].crown.as_ref().map(|c| c.x) else {
        return;
    };
    for qi in 0..w.players.len() {
        let q = &w.players[qi];
        if q.free || (q.x - cx).abs() > d.pickup_units || (qi != oi && bearing(w, qi)) {
            continue;
        }
        if qi == oi {
            restore_crown(w, oi, qi);
        } else {
            let (qid, qindex, owner_index) = (q.id, q.index, w.players[oi].index);
            if let Some(c) = w.players[oi].crown.as_mut() {
                c.state = CROWN_PLAYER.into();
                c.carrier = qid;
            }
            emit(
                w,
                "crownPickedUp",
                event(json!({"player": owner_index, "by": qindex, "x": unit_x(cx)})),
            );
        }
        return;
    }
}

/// Go: `restoreCrown`: setzt dem Besitzer `oi` seine Krone wieder auf; `bi` ist, wer sie zurückgebracht hat.
pub fn restore_crown(w: &mut World, oi: usize, bi: usize) {
    w.players[oi].crown = None;
    let (owner, by, x) = (w.players[oi].index, w.players[bi].index, w.players[oi].x);
    emit(
        w,
        "crownRestored",
        event(json!({"player": owner, "by": by, "x": unit_x(x)})),
    );
}

/// Go: `followBearer`: Die Krone folgt dem Mitspieler; erreicht er den Besitzer, bekommt der sie zurück.
fn follow_bearer(w: &mut World, d: &CrownData, oi: usize) {
    let Some(carrier) = w.players[oi].crown.as_ref().map(|c| c.carrier) else {
        return;
    };
    let Some(qi) = w.players.iter().position(|q| q.id == carrier) else {
        if let Some(c) = w.players[oi].crown.as_mut() {
            c.state = CROWN_GROUND.into();
            c.carrier = 0;
        }
        return;
    };
    let qx = w.players[qi].x;
    if let Some(c) = w.players[oi].crown.as_mut() {
        c.x = qx;
    }
    if qi != oi && (qx - w.players[oi].x).abs() <= d.pickup_units {
        restore_crown(w, oi, qi);
    }
}

/// Go: `followEnemy`: Die Krone folgt dem Gegner; ist er weg (gefallen, abgereist), liegt sie an seiner letzten Stelle.
fn follow_enemy(w: &mut World, oi: usize) {
    let Some(carrier) = w.players[oi].crown.as_ref().map(|c| c.carrier) else {
        return;
    };
    // `e.Crown == c` (derselbe Zeiger): der Gegner trägt diese Krone, wenn er sie als Kopie hält
    let found = w.enemies.iter().position(|e| e.id == carrier && e.crown.is_some());
    if let Some(i) = found {
        let x = w.enemies[i].x;
        if let Some(c) = w.players[oi].crown.as_mut() {
            c.x = x;
        }
        if let Some(c) = w.enemies[i].crown.as_mut() {
            c.x = x; // Go: derselbe Zeiger
        }
        return;
    }
    if let Some(c) = w.players[oi].crown.as_mut() {
        c.state = CROWN_GROUND.into();
        c.carrier = 0;
    }
}

/// Go: `takeCrownOrLoot`: lässt den Gegner `ei` eine Krone am Boden aufheben (Kronenraub) oder, mit `stealsGold`, eine
/// Münze aus einem Treffer auf den Monarchen. Nicht fliehende Gegner ohne Last, keine Bosse und Wachen, nur solange die
/// Stufe Portale hat und nicht in einer geschützten Nacht; Kronen abwesender (Free) Monarchen sind nicht raubbar.
pub fn take_crown_or_loot(w: &mut World, v: &WorldValues, ei: usize, isl: Option<&IslandRef>) {
    let protected = protected_night(w, isl);
    let e = &w.enemies[ei];
    if e.boss || e.guard || e.fleeing || e.crown.is_some() || w.portals.is_empty() || protected {
        return;
    }
    let (ex, eid) = (e.x, e.id);
    let grab = v.monarch.crown.grab_units;
    for pi in 0..w.players.len() {
        let p = &mut w.players[pi];
        let free = p.free;
        let Some(c) = p.crown.as_mut() else {
            continue;
        };
        if free || c.state != CROWN_GROUND || (c.x - ex).abs() > grab {
            continue;
        }
        c.state = CROWN_ENEMY.into();
        c.carrier = eid;
        let copy = c.clone();
        let index = p.index;
        let e = &mut w.enemies[ei];
        e.crown = Some(copy);
        e.fleeing = true;
        emit(w, "crownTaken", event(json!({"player": index, "x": unit_x(ex)})));
        return;
    }
    if !w.enemies[ei].has("stealsGold") {
        return;
    }
    let Some(i) = w.coins.iter().position(|c| c.loot && (c.x - ex).abs() <= grab) else {
        return;
    };
    w.coins.remove(i);
    let e = &mut w.enemies[ei];
    e.carried_gold += 1;
    e.fleeing = true;
}

/// Go: `dropCrownOf`: lässt einen Gegner die getragene Krone an seiner Stelle fallen (gefallen, kein Portal mehr). Der
/// Gegner ist bei `drop_carried` schon aus `w.enemies` genommen; die Krone des Besitzers liegt in `Player::crown`.
pub fn drop_crown_of(w: &mut World, e: &mut Enemy) {
    let Some(copy) = e.crown.take() else {
        return;
    };
    if let Some(c) = w
        .players
        .iter_mut()
        .find(|p| p.index == copy.owner)
        .and_then(|p| p.crown.as_mut())
    {
        c.state = CROWN_GROUND.into();
        c.carrier = 0;
        c.x = e.x;
        c.z = 0.0;
    }
}

/// Go: `crownStolen`: Ein Gegner trägt eine Krone ins Portal: Kronenraub, das Spiel ist verloren (kampf.md § 6), in jedem
/// Grad. Mit Insel endet sie (`IslandRef::over`, `game_over`).
pub fn crown_stolen(w: &mut World, e: &Enemy, isl: Option<&mut IslandRef>) {
    let owner = e.crown.as_ref().map_or(0, |c| c.owner);
    emit(w, "crownStolen", event(json!({"player": owner, "x": unit_x(e.x)})));
    game_over(w, isl);
}

/// Go: `hitCoins`: Münzen eines Treffers fallen neben dem Monarchen `pi` auf den Boden, aufsammelbar für alle; der
/// Getroffene selbst sammelt sie erst nach einer Sperrzeit wieder ein. Deterministisch gestaffelt, ohne Würfel.
pub fn hit_coins(w: &mut World, pi: usize, n: i64) {
    let v = values(w);
    hit_coins_with(w, &v, pi, n);
}

/// `hit_coins` mit bereits geholtem Wertesatz.
pub fn hit_coins_with(w: &mut World, v: &WorldValues, pi: usize, n: i64) {
    let back = crown_direction(w, pi);
    let d = &v.monarch.crown;
    let (px, pid) = (w.players[pi].x, w.players[pi].id);
    for i in 0..n {
        let x = w
            .width_units
            .min((px + back * (d.coin_offset_units + d.coin_step_units * i as f64)).max(0.0));
        let id = w.new_id();
        w.coins.push(Coin {
            id,
            x,
            blocked_player_id: Some(pid),
            blocked_until: w.time + d.block_seconds,
            loot: true,
            ..Coin::default()
        });
    }
}

#[cfg(test)]
mod review_tests;
#[cfg(test)]
mod tests;
#[cfg(test)]
pub(crate) mod testutil;
