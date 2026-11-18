//! Ziele der Gegner (Port von `engine/sim/enemies.go`, ab `type target`): Mauer suchen, Ziele in Reichweite sammeln,
//! das passende wählen, angreifen.
//!
//! Gos `candidates` baut je Aufruf eine Liste; hier durchläuft `for_each_candidate` dieselben Ziele in derselben
//! Reihenfolge ohne Liste, `choose_target` merkt sich je Rang nur das beste (kein Allokieren im Takt).

use serde_json::json;

use super::traits::attack_rate;
use super::{BODY, is_barrier, is_building};
use crate::state::common::{apply_damage_by, event, shielded};
use crate::state::island_ref::{IslandRef, protected_night};
use crate::state::pending::{
    MERCHANT_ID, arrow_event, crowned, drop_loot, emit, frost_armor_hit, merchant_x, moon_damage, unit_x,
};
use crate::state::{Enemy, Projectile, World, WorldValues};
use crate::unit::archer::is_on_tower;

/// Art eines Ziels (Go: Texte `player`, `troop`, `merchant`, `wall`, `castle`, `site`, `building`).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TargetKind {
    Player,
    Troop,
    Merchant,
    /// Mauer oder Tor.
    Wall,
    Castle,
    /// Turm.
    Site,
    /// Übrige Gebäude.
    Building,
}

impl TargetKind {
    /// Der Go-Text der Art.
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Player => "player",
            Self::Troop => "troop",
            Self::Merchant => "merchant",
            Self::Wall => "wall",
            Self::Castle => "castle",
            Self::Site => "site",
            Self::Building => "building",
        }
    }
}

/// Go: `target`. `player` ist der Index in `World::players`.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Target {
    pub id: i64,
    pub x: f64,
    pub kind: TargetKind,
    pub player: Option<usize>,
}

/// Die Mauer (oder das Tor), die einen Gegner aufhält: nur ID und X werden gelesen.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Wall {
    pub id: i64,
    pub x: f64,
}

/// Go: `blockingWall`: die nächste Mauer vor dem Gegner, zwischen ihm und der Burg.
pub fn blocking_wall(w: &World, e: &Enemy, dir: f64) -> Option<Wall> {
    let mut best: Option<Wall> = None;
    for s in &w.sites {
        if !is_barrier(s) {
            continue;
        }
        let ahead = if dir > 0.0 {
            s.x >= e.x - BODY && s.x < w.hub_x
        } else {
            s.x <= e.x + BODY && s.x > w.hub_x
        };
        if ahead && best.is_none_or(|b| (s.x - e.x).abs() < (b.x - e.x).abs()) {
            best = Some(Wall { id: s.id, x: s.x });
        }
    }
    best
}

/// Go: `reach` sagt, ob ein Gegner x angreifen kann. Nahkämpfer erreichen nichts hinter der Mauer.
struct Reach {
    ex: f64,
    range: f64,
    ranged: bool,
    dir: f64,
    wall: Option<Wall>,
}

impl Reach {
    fn reachable(&self, x: f64) -> bool {
        self.ranged
            || self
                .wall
                .is_none_or(|wl| (self.dir > 0.0 && x <= wl.x) || (self.dir <= 0.0 && x >= wl.x))
    }

    fn in_range(&self, x: f64) -> bool {
        (x - self.ex).abs() <= self.range + BODY && self.reachable(x)
    }
}

/// Go: `candidates`: alle Ziele in Reichweite in der Reihenfolge von `chooseTarget` in TS: Spieler, Truppen, Händler,
/// Mauer oder Tor, Türme (nur Fernkämpfer), übrige Gebäude, Burg.
fn for_each_candidate(w: &World, r: &Reach, castle_radius: f64, mut f: impl FnMut(Target)) {
    for (pi, p) in w.players.iter().enumerate() {
        if crowned(p) && !shielded(p) && r.in_range(p.x) {
            f(Target {
                id: p.id,
                x: p.x,
                kind: TargetKind::Player,
                player: Some(pi),
            });
        }
    }
    for t in &w.troops {
        if t.kind != "vagrant" && (r.ranged || !is_on_tower(w, t)) && r.in_range(t.x) {
            f(Target {
                id: t.id,
                x: t.x,
                kind: TargetKind::Troop,
                player: None,
            });
        }
    }
    if w.merchant.is_some() {
        let x = merchant_x(w, "buy");
        if r.in_range(x) {
            f(Target {
                id: MERCHANT_ID,
                x,
                kind: TargetKind::Merchant,
                player: None,
            });
        }
    }
    building_targets(w, r, castle_radius, &mut f);
}

fn building_targets(w: &World, r: &Reach, castle_radius: f64, f: &mut impl FnMut(Target)) {
    if let Some(wl) = r.wall
        && (wl.x - r.ex).abs() <= r.range + BODY + 0.1
    {
        f(Target {
            id: wl.id,
            x: wl.x,
            kind: TargetKind::Wall,
            player: None,
        });
    }
    for s in &w.sites {
        if r.ranged && s.kind == "tower" && s.state == "built" && r.in_range(s.x) {
            f(Target {
                id: s.id,
                x: s.x,
                kind: TargetKind::Site,
                player: None,
            });
        } else if is_building(s) && r.in_range(s.x) {
            f(Target {
                id: s.id,
                x: s.x,
                kind: TargetKind::Building,
                player: None,
            });
        }
    }
    if (w.castle.x - r.ex).abs() <= r.range + castle_radius + 0.1 && r.reachable(w.castle.x) {
        f(Target {
            id: w.castle.id,
            x: w.castle.x,
            kind: TargetKind::Castle,
            player: None,
        });
    }
}

/// Die bevorzugten Ziel-Arten eines Gegners (Go: `Enemy.prefers`), einmal je Wahl aus den Eigenschaften gelesen.
struct Prefers {
    buildings: bool,
    towers: bool,
    troops: bool,
    monarch: bool,
}

impl Prefers {
    fn of(e: &Enemy) -> Self {
        Self {
            buildings: e.has("prefersBuildings"),
            towers: e.has("prefersTowers"),
            troops: e.has("prefersTroops"),
            monarch: e.has("prefersMonarch") || e.has("stealsGold"),
        }
    }

    fn wants(&self, k: TargetKind) -> bool {
        use TargetKind::*;
        (self.buildings && matches!(k, Wall | Castle | Site | Building))
            || (self.towers && k == Site)
            || (self.troops && k == Troop)
            || (self.monarch && k == Player)
    }
}

/// Ersetzt `best`, wenn `c` strikt näher an `ex` liegt; bei Gleichstand bleibt das frühere (wie `reduce` in TS).
fn nearer(best: &mut Option<Target>, c: Target, ex: f64) {
    if best.is_none_or(|b| (c.x - ex).abs() < (b.x - ex).abs()) {
        *best = Some(c);
    }
}

/// Go: `chooseTarget`: ein verspottender Monarch in Reichweite zuerst (Taunt), dann der Händler beim Überfall
/// (events_merchant.go), sonst bevorzugte Ziele, davon das nächste; bei Gleichstand das frühere. Ohne bevorzugte Ziele:
/// übrige Gebäude nur, wenn nichts anderes in Reichweite ist (gegner.md § 4).
pub fn choose_target(w: &World, v: &WorldValues, ei: usize, dir: f64, wall: Option<Wall>) -> Option<Target> {
    let e = &w.enemies[ei];
    let r = Reach {
        ex: e.x,
        range: e.range,
        ranged: e.has("ranged"),
        dir,
        wall,
    };
    let prefers = Prefers::of(e);
    let raid = w.merchant.as_ref().is_some_and(|m| m.raid);
    let taunting = e.taunt_for > 0.0;
    let (mut taunt, mut raid_hit, mut pref, mut other, mut any) = (None, None, None, None, None);
    for_each_candidate(w, &r, v.hub.castle_radius_units, |c| {
        if taunt.is_none() && taunting && c.kind == TargetKind::Player && c.id == e.taunt_id {
            taunt = Some(c);
        }
        if raid_hit.is_none() && raid && c.kind == TargetKind::Merchant {
            raid_hit = Some(c);
        }
        if prefers.wants(c.kind) {
            nearer(&mut pref, c, e.x);
        }
        if c.kind != TargetKind::Building {
            nearer(&mut other, c, e.x);
        }
        nearer(&mut any, c, e.x);
    });
    taunt.or(raid_hit).or(pref).or(other).or(any)
}

/// Go: `attack`: Der Gegner greift das Ziel an (Gold stehlen, Pfeil schießen oder zuschlagen).
pub fn attack(w: &mut World, v: &WorldValues, ei: usize, t: &Target, isl: Option<&IslandRef>) {
    w.enemies[ei].cooldown = 1.0 / attack_rate(v, &w.enemies[ei].kind);
    if let Some(pi) = t.player
        && w.enemies[ei].has("stealsGold")
        && !w.enemies[ei].guard
        && w.players[pi].gold > 0
        && !protected_night(w, isl)
    {
        steal_gold(w, v, ei, pi);
        return;
    }
    if w.enemies[ei].has("ranged") {
        let id = w.new_id();
        let e = &w.enemies[ei];
        let p = Projectile {
            id,
            x: e.x,
            target_id: t.id,
            team: "enemy".into(),
            damage: moon_damage(w, e.damage),
            speed: 20.0,
            cause: e.kind.clone(), // je Pfeil eine Art; Pfeile entstehen höchstens einmal je Abklingzeit
            ..Default::default()
        };
        let from = e.id;
        arrow_event(w, &p, from);
        w.projectiles.push(p);
    } else {
        melee(w, ei, t);
    }
}

fn steal_gold(w: &mut World, v: &WorldValues, ei: usize, pi: usize) {
    let amount = w.players[pi].gold.min(v.waves.steal_gold);
    w.players[pi].gold -= amount;
    let e = &mut w.enemies[ei];
    e.carried_gold += amount;
    e.fleeing = true;
    if w.portals.is_empty() {
        drop_loot(w, ei); // ohne Portal gibt es keinen Raub: die Beute fällt als Münzen zu Boden
    }
    let index = w.players[pi].index;
    w.events
        .push(event(json!({"type": "goldStolen", "player": index, "amount": amount})));
}

fn melee(w: &mut World, ei: usize, t: &Target) {
    let (id, x, damage) = (w.enemies[ei].id, w.enemies[ei].x, w.enemies[ei].damage);
    emit(w, "strike", event(json!({"from": id, "x": unit_x(x)})));
    let kind = std::mem::take(&mut w.enemies[ei].kind); // ohne Kopie als Ursache; sofort zurück
    let dmg = moon_damage(w, damage); // Blutmond (events_moon.go)
    apply_damage_by(w, t.id, dmg, &kind);
    w.enemies[ei].kind = kind;
    if let Some(pi) = t.player {
        frost_armor_hit(w, pi, ei);
    }
}
