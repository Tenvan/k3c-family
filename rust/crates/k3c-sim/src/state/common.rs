//! Gemeinsame Helfer (Port von `engine/sim/common.go`).
//!
//! Go-Zeiger auf Spieler und Truppen sind Indizes in `World::players` und `World::troops` (`pi`, `ti`); Plätze und
//! Knoten werden über ihre ID gesucht (Zustandsmodell, Vorentscheidung 2).

use k3c_det::fmax;
use serde_json::{Value, json};

use super::island_ref::{IslandRef, protected_night};
use super::links::{
    MERCHANT_ID, crowned, damage_merchant, drop_crown, each_passive, hit_coins, hit_event, invulnerable, is_ruin,
    is_wave_enemy, troop_damage, values,
};
use super::{Event, Player, ResourceNode, Site, Troop, World};

/// Go: `causeOther`, die Todesursache in crownFallen, wenn kein Gegner den Schaden verursacht hat.
pub const CAUSE_OTHER: &str = "other";

impl World {
    /// Troop mit dieser ID (lineare Suche wie in Go).
    pub fn troop(&self, id: i64) -> Option<&Troop> {
        self.troops.iter().find(|t| t.id == id)
    }

    pub fn troop_mut(&mut self, id: i64) -> Option<&mut Troop> {
        self.troops.iter_mut().find(|t| t.id == id)
    }

    pub fn node(&self, id: i64) -> Option<&ResourceNode> {
        self.nodes.iter().find(|n| n.id == id)
    }

    pub fn node_mut(&mut self, id: i64) -> Option<&mut ResourceNode> {
        self.nodes.iter_mut().find(|n| n.id == id)
    }

    /// Platz aus `sites`, danach der Zahlplatz für den Hub-Ausbau (`hub_site`).
    pub fn site(&self, id: i64) -> Option<&Site> {
        self.sites
            .iter()
            .find(|s| s.id == id)
            .or(self.hub_site.as_ref().filter(|s| s.id == id))
    }

    pub fn site_mut(&mut self, id: i64) -> Option<&mut Site> {
        match self.sites.iter().position(|s| s.id == id) {
            Some(i) => self.sites.get_mut(i),
            None => self.hub_site.as_mut().filter(|s| s.id == id),
        }
    }
}

/// Go: `approach`, bewegt x mit speed auf target zu, ohne darüber hinauszuschießen.
pub fn approach(x: f64, target: f64, speed: f64, dt: f64) -> f64 {
    let step = speed * dt;
    if (target - x).abs() <= step {
        return target;
    }
    x + sign(target - x) * step
}

/// Go: `sign`, wie Math.sign für endliche Zahlen.
pub fn sign(v: f64) -> f64 {
    if v > 0.0 {
        1.0
    } else if v < 0.0 {
        -1.0
    } else {
        0.0
    }
}

/// Go: `intPtr`.
pub fn int_ptr(v: i64) -> Option<i64> {
    Some(v)
}

/// Go: `shielded`: Der Monarch ist kein Ziel und nimmt keinen Schaden, wenn ein pausiertes Gerät ihn schützt, niemand ihn
/// steuert oder er unsichtbar ist.
pub fn shielded(p: &Player) -> bool {
    p.protected || p.free || p.stealth_for > 0.0
}

/// Go: `isDangerous`: nachts oder solange Gegner einer Welle in der Welt sind, bleiben Bauern im Hub.
pub fn is_dangerous(w: &World) -> bool {
    w.cycle.phase == "night" || w.enemies.iter().any(is_wave_enemy)
}

/// B-1253: Handwerker (mit Hammer) arbeiten ab Sonnenaufgang weiter, auch wenn noch Wellen-Gegner leben; sie suchen
/// nur nachts oder bei Feindkontakt (Gegner in Hörweite) Schutz. Alle anderen wie `is_dangerous`.
pub fn danger_for(w: &World, ti: usize) -> bool {
    let t = &w.troops[ti];
    if !t.hammer {
        return is_dangerous(w);
    }
    let near = crate::state::links::values(w)
        .troops
        .get("peasant")
        .map_or(0.0, |d| d.fetch_enemy_distance);
    w.cycle.phase == "night" || w.enemies.iter().any(|e| e.hp > 0.0 && (e.x - t.x).abs() <= near)
}

/// Go: `outerWall`, die intakte Mauer auf der Seite `side` (-1 links, 1 rechts), die am weitesten außen steht.
pub fn outer_wall(w: &World, side: f64) -> Option<&Site> {
    let mut best: Option<&Site> = None;
    for s in &w.sites {
        if s.kind != "wall" || s.state != "built" || sign(s.x - w.hub_x) != side {
            continue;
        }
        if best.is_none_or(|b| (s.x - w.hub_x).abs() > (b.x - w.hub_x).abs()) {
            best = Some(s);
        }
    }
    best
}

/// Go: `siteByID`.
pub fn site_by_id(w: &World, id: i64) -> Option<&Site> {
    w.site(id)
}

/// Go: `troopByID`.
pub fn troop_by_id(w: &World, id: i64) -> Option<&Troop> {
    w.troop(id)
}

/// Go: `nodeByID`.
pub fn node_by_id(w: &World, id: i64) -> Option<&ResourceNode> {
    w.node(id)
}

/// Go: `isWorker`: Lebt die Truppe mit dieser ID noch?
pub fn is_worker(w: &World, id: Option<i64>) -> bool {
    id.is_some_and(|id| w.troops.iter().any(|o| o.id == id))
}

/// Go: `applyDamage`, Schaden ohne Gegner als Quelle auf ein Ziel per ID. Die Aufrufer (Zauberturm, Monarch, Zauber)
/// treffen Gegner und Truppen, nie Plätze; darum braucht der Schaden den Bezug zur Insel (geschützte Nacht) nicht.
pub fn apply_damage(w: &mut World, target_id: i64, damage: f64) {
    apply_damage_by(w, target_id, damage, CAUSE_OTHER, None);
}

/// Go: `hitTroop`: Eine getroffene Truppe verliert HP; ein getroffener Bürger flieht zur Burg. `ti` ist der Index.
pub fn hit_troop(w: &mut World, ti: usize, damage: f64, cause: &str) {
    let damage = troop_damage(w, ti, damage);
    let t = &mut w.troops[ti];
    t.hp -= damage;
    t.hit_by = cause.to_string();
    if t.serving() {
        t.fleeing = true;
    }
    let (id, x) = (t.id, t.x);
    hit_event(w, "troop", id, x, damage);
}

/// Go: `applyDamageBy`: Schaden auf ein Ziel per ID; `cause` ist die Gegnerart für crownFallen, "" = `CAUSE_OTHER`.
/// Tod und Zerstörung behandelt der Tick.
pub fn apply_damage_by(w: &mut World, target_id: i64, damage: f64, cause: &str, isl: Option<&IslandRef>) {
    if let Some(pi) = w.players.iter().position(|p| p.id == target_id) {
        damage_player(w, pi, damage, cause);
        return;
    }
    if let Some(ti) = w.troops.iter().position(|t| t.id == target_id) {
        hit_troop(w, ti, damage, cause);
        return;
    }
    if let Some(ei) = w.enemies.iter().position(|e| e.id == target_id) {
        if !invulnerable(w, &w.enemies[ei]) {
            // Phase (`phases`): kein Schaden, kein hit
            let e = &mut w.enemies[ei];
            e.hp -= damage;
            let (id, x) = (e.id, e.x);
            hit_event(w, "enemy", id, x, damage);
        }
        return;
    }
    if target_id == MERCHANT_ID && w.merchant.is_some() {
        damage_merchant(w, damage);
        return;
    }
    if w.castle.id == target_id {
        w.castle.hp -= damage;
        let (id, x) = (w.castle.id, w.castle.x);
        hit_event(w, "castle", id, x, damage);
        return;
    }
    hit_site(w, target_id, damage, isl);
}

/// Der Platz-Zweig von `applyDamageBy`: nur gebaute Plätze nehmen Schaden; ein zerstörter wird wieder leer.
fn hit_site(w: &mut World, id: i64, damage: f64, isl: Option<&IslandRef>) {
    let Some(s) = w.site_mut(id).filter(|s| s.state == "built") else {
        return;
    };
    s.hp -= damage;
    let (hp, sid, x) = (s.hp, s.id, s.x);
    hit_event(w, "site", sid, x, damage);
    if hp <= 0.0 && protected_night(w, isl) {
        if let Some(s) = w.site_mut(id) {
            s.hp = 1.0;
        }
    } else if hp <= 0.0 {
        destroy_site(w, id);
    }
}

/// Go: `damagePlayer` (kampf.md § 5): Der Monarch hat keine HP. Der Schild fängt den Treffer ab, jeder
/// `freeHitEvery`-te Treffer kostet nichts, Last Stand lässt die letzte Münze einen Treffer länger halten. Sonst kostet der
/// Treffer `hitCoins` Münzen, die zu Boden fallen; ohne Gold fällt die Krone. `pi` ist der Index des Spielers.
pub fn damage_player(w: &mut World, pi: usize, damage: f64, cause: &str) {
    if !crowned(&w.players[pi]) || shielded(&w.players[pi]) {
        return;
    }
    let mut lost = 0;
    if absorb_by_shield(&mut w.players[pi], damage) || free_hit(w, pi) {
        // abgefangen
    } else if last_stand_holds(w, pi) {
        w.players[pi].last_stand_for = 0.0;
    } else if w.players[pi].gold > 0 {
        lost = w.players[pi].gold.min(values(w).monarch.hit_coins);
        w.players[pi].gold -= lost;
        hit_coins(w, pi, lost);
    } else {
        drop_crown(w, pi, cause_or_other(cause));
        let (index, x) = (w.players[pi].index, w.players[pi].x);
        hit_event(w, "player", index, x, 0.0);
        return;
    }
    let (index, x) = (w.players[pi].index, w.players[pi].x);
    hit_event(w, "player", index, x, lost as f64);
}

/// Last Stand: Die letzte Münze hält einen Treffer länger.
fn last_stand_holds(w: &World, pi: usize) -> bool {
    let p = &w.players[pi];
    p.last_stand_for > 0.0 && p.gold > 0 && p.gold <= values(w).monarch.hit_coins
}

/// Go: `causeOrOther`.
pub fn cause_or_other(cause: &str) -> &str {
    if cause.is_empty() { CAUSE_OTHER } else { cause }
}

/// Go: `absorbByShield`: Der Schild nimmt den Treffer ganz (auch den Überschuss) und zieht den Schaden von seinen Punkten ab.
pub fn absorb_by_shield(p: &mut Player, damage: f64) -> bool {
    if p.shield <= 0.0 {
        return false;
    }
    p.shield = fmax(0.0, p.shield - fmax(1.0, damage));
    if p.shield == 0.0 {
        p.shield_for = 0.0;
    }
    true
}

/// Go: `freeHit`: Thick Skin (freeHitEvery n): jeder n-te Treffer kostet nichts; mehrere Passive: der kleinste Wert gilt.
pub fn free_hit(w: &mut World, pi: usize) -> bool {
    w.players[pi].hits += 1;
    let (pid, hits) = (w.players[pi].id, w.players[pi].hits);
    let mut every = 0;
    each_passive(w, &mut |o, e| {
        if o.id == pid && e.free_hit_every > 0 && (every == 0 || e.free_hit_every < every) {
            every = e.free_hit_every;
        }
    });
    every > 0 && hits % every == 0
}

/// Go: `destroySite`: Der Bauplatz ist wieder leer und muss neu bezahlt werden; eine Ruine bleibt Ruine. `id` ist die ID
/// des Platzes (in `sites` oder der Hub-Ausbau-Platz).
pub fn destroy_site(w: &mut World, id: i64) {
    let Some(s) = w.site(id) else {
        return;
    };
    let (kind, x, sid, was_built) = (s.kind.clone(), s.x, s.id, s.state == "built");
    let max_hp = values(w).buildings.get(&kind).map_or(0.0, |b| b.hp);
    let fresh = Site {
        id: sid,
        kind: kind.clone(),
        x,
        state: "unpaid".to_string(),
        max_hp,
        ruin: is_ruin(w, &kind, x),
        ..Default::default()
    };
    if let Some(s) = w.site_mut(id) {
        *s = fresh;
    }
    for t in w.troops.iter_mut() {
        if t.tower_id == Some(sid) {
            t.tower_id = None;
        }
        let build_job = t
            .job
            .as_ref()
            .is_some_and(|j| matches!(j.r#type.as_str(), "build" | "fetchBow" | "fetchHammer") && j.site_id == sid);
        if build_job {
            t.job = None;
        }
    }
    if was_built {
        w.events.push(event(json!({"type": "destroyed", "kind": kind})));
    }
}

/// Baut aus einem JSON-Objekt ein Ereignis (Go: `Event{...}`).
#[allow(clippy::panic)] // STAB(B-1076): Ereignis ist immer ein Objekt, Fehlerweg folgt
pub fn event(v: Value) -> Event {
    match v {
        Value::Object(m) => m,
        other => panic!("Ereignis ist kein Objekt: {other}"),
    }
}
