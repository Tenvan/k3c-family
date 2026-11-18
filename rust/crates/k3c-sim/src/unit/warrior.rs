//! Krieger, Schwert und Verlust (Port von `engine/sim/unit_warrior.go`, docs/rules/buerger.md § 1 bis 3, B-014, B-122;
//! Q34, Q38, Q46, Q53, Q54, Q67 bis Q69):
//!   - Schwert: eigenes Zahlziel an der Werkstatt (Angebot sword mit dx aus data/buildings.json), Regal swordRack,
//!     Herstellungszeit craftSeconds.sword; ein Bauer holt es (fetchSword) und wird Krieger.
//!   - Krieger stehen innen vor der äußersten gebauten Sperre ihrer Seite: mit gebautem Tor der äußersten Linie zwischen
//!     Mauer und Tor, sonst hinter der Mauer; Nahkampf mit den Werten aus troops.json, Seitenverteilung wie Schützen.
//!   - Verlust-Kaskade statt Tod: Ausrüstung fällt zu Boden (Drop, Ereignis disarmed), der Bürger wird Bauer; ein
//!     Bauer lässt eine Münze fallen und wird Landstreicher. Ausrüstung am Boden holen Bauern wie eine Waffe (pickup).

use std::collections::BTreeMap;

use serde_json::json;

use super::escort::escort_post;
use super::flee::enemy_within;
use super::hammer::{HAMMER_KIND, drop_hammer};
use super::jobs::{make_fighter, release_job};
use super::professions::{craft_seconds, near_offer};
use super::walk_to;
use crate::combat::{combat_spot_near, leave_squad, squad_post};
use crate::economy::{can_afford, clear_pending, nearest, spend};
use crate::enemies::BODY;
use crate::state::common::{apply_damage, cause_or_other, event, is_dangerous, outer_wall};
use crate::state::island_ref::{IslandRef, cheap};
use crate::state::pending::{emit, fighters, outpost, troop_limit, unit_x, weapons_in_flight};
use crate::state::{ItemDrop, Job, World, WorldValues};
use k3c_data::troops::TroopData;

/// Go: `isFighter`: Ist `kind` eine Kämpfer-Figur aus `troops` (nicht Landstreicher, Bauer oder Beruf)?
pub fn is_fighter(troops: &BTreeMap<String, TroopData>, kind: &str) -> bool {
    kind != "vagrant" && kind != "peasant" && troops.contains_key(kind)
}

/// Go: `swordSpot` ist der Ort des Schwert-Zahlziels am Platz `si`; `None`, wenn sein Gebäude keins hat.
pub fn sword_spot(w: &World, v: &WorldValues, si: usize) -> Option<f64> {
    let s = &w.sites[si];
    let b = v.buildings.get(&s.kind)?;
    b.offers.iter().find(|o| o.kind == "sword").map(|o| s.x + o.dx)
}

/// Go: `swordAt` ist die gebaute Werkstatt (Index), deren Schwert-Zahlziel in Zahl-Reichweite von `x` liegt; mit
/// `payable` nur, wenn sie gerade Münzen nimmt.
pub fn sword_at(w: &World, v: &WorldValues, isl: Option<&IslandRef>, x: f64, payable: bool) -> Option<usize> {
    (0..w.sites.len()).find(|&si| {
        sword_spot(w, v, si).is_some_and(|spot| {
            w.sites[si].state == "built"
                && (spot - x).abs() <= v.economy.pay_range_units
                && (!payable || sword_payable(w, v, isl, si))
        })
    })
}

/// Go: `swordPayable`: Platz im Regal und das Schwert noch nicht voll bezahlt.
pub fn sword_payable(w: &World, v: &WorldValues, isl: Option<&IslandRef>, si: usize) -> bool {
    let s = &w.sites[si];
    let in_work = i64::from(s.sword_done > 0.0);
    let rack = v.buildings.get(&s.kind).map_or(0, |b| b.sword_rack);
    let gold = v.troops.get("warrior").map_or(0, |t| cheap(isl, &t.cost).gold);
    s.swords + in_work < rack && s.sword_paid_gold < gold
}

/// Go: `nearFullTarget`: Liegt `x` an einem Bauplatz, Angebot oder Schwert-Zahlziel, das gerade nichts nimmt
/// (`payOneCoin`)?
pub fn near_full_target(w: &World, v: &WorldValues, x: f64) -> bool {
    near_offer(w, v, x)
        || sword_at(w, v, None, x, false).is_some()
        || combat_spot_near(w, v, x)
        || nearest(&w.sites, |s| s.x, x, v.economy.pay_range_units, |_| true).is_some()
}

/// Go: `paySword`: Spieler `pi` zahlt eine Münze auf das Schwert am Platz `si`.
pub fn pay_sword(w: &mut World, v: &WorldValues, isl: Option<&IslandRef>, pi: usize, si: usize) {
    w.sites[si].sword_paid_gold += 1;
    if !sword_payable(w, v, isl, si) {
        clear_pending(&mut w.players[pi]);
    }
}

/// Go: `refundSword` zieht bis zu `amount` Münzen vom Schwert des Platzes `id` zurück und liefert die Anzahl.
pub fn refund_sword(w: &mut World, id: i64, amount: i64) -> i64 {
    let Some(s) = w.site_mut(id) else {
        return 0;
    };
    let back = amount.min(s.sword_paid_gold);
    s.sword_paid_gold -= back;
    back
}

/// Go: `stepSwordCraft` wie `step_bow_craft`: Ein voll bezahltes Schwert geht in Arbeit, sobald das Material da ist
/// und gerade keins entsteht, und liegt nach craftSeconds.sword (je Handwerker schneller) im Regal.
pub fn step_sword_craft(w: &mut World, v: &WorldValues, isl: Option<&IslandRef>, si: usize) {
    if w.sites[si].sword_done > 0.0 && w.time >= w.sites[si].sword_done {
        w.sites[si].swords += 1;
        w.sites[si].sword_done = 0.0;
    }
    let cost = v.troops.get("warrior").map(|t| cheap(isl, &t.cost)).unwrap_or_default();
    if w.sites[si].sword_done == 0.0 && w.sites[si].sword_paid_gold >= cost.gold && can_afford(&w.stock, &cost) {
        spend(&mut w.stock, &cost);
        w.sites[si].sword_paid_gold = 0;
        w.sites[si].sword_done = w.time + craft_seconds(w, v, si, "sword");
    }
}

/// Der beste Kandidat von `weapon_to_fetch`: Auftragsart, Platz, Drop und Abstand (der `Job` entsteht erst am Ende, damit
/// die Suche nichts anlegt).
struct Pick {
    typ: &'static str,
    site_id: i64,
    drop_id: i64,
    dist: f64,
}

/// Go: `weaponToFetch`: die nächste Ausrüstung am Boden oder Bogen bzw. Schwert im Regal, die noch kein anderer Bauer
/// holt; `None`, wenn es nichts gibt oder ein Gegner näher als `fetchEnemyDistance` ist (am Bauern oder am Ziel,
/// kampf.md § 4; ersetzt hier die globale Gefahr, nur Vorposten bleiben bei Gefahr tabu). Bei gleicher Distanz zählt der
/// Boden vor dem Regal. Kämpfer-Waffen nur unter dem Limit (Q29, einzige Prüfstelle): sonst bleiben sie liegen.
pub fn weapon_to_fetch(w: &World, v: &WorldValues, ti: usize) -> Option<Job> {
    let t = &w.troops[ti];
    let near = v.troops.get("peasant").map_or(0.0, |p| p.fetch_enemy_distance);
    if t.hammer || enemy_within(w, t.x, near) {
        return None; // ein Handwerker bleibt Handwerker: Werkzeug und Waffe schließen sich aus (B-950)
    }
    let full = fighters(w) + weapons_in_flight(w, ti) >= troop_limit(w);
    let mut best: Option<Pick> = None;
    // strikt kleiner: bei Gleichstand bleibt der frühere (Boden) Kandidat
    let mut consider = |typ: &'static str, site_id: i64, drop_id: i64, x: f64| {
        let dist = (x - t.x).abs();
        if best.as_ref().is_none_or(|b| dist < b.dist) && !enemy_within(w, x, near) {
            best = Some(Pick {
                typ,
                site_id,
                drop_id,
                dist,
            });
        }
    };
    for d in &w.drops {
        if (!full || !is_fighter(&v.troops, &d.kind)) && fetching(w, ti, "pickup", d.id) == 0 {
            consider("pickup", 0, d.id, d.x);
        }
    }
    if shelf_stocked(w) {
        shelf_jobs(w, ti, &mut consider, full);
    }
    best.map(|b| Job {
        r#type: b.typ.into(),
        site_id: b.site_id,
        drop_id: b.drop_id,
        ..Job::default()
    })
}

/// Go: `shelfJobs` meldet die Regal-Aufträge (Bogen, Schwert) gebauter Werkstätten an `consider`; Vorposten nur ohne
/// Gefahr. Die Closure liest nur die Welt, darum genügt ein Durchlauf ohne Zwischenliste.
fn shelf_jobs(w: &World, ti: usize, consider: &mut dyn FnMut(&'static str, i64, i64, f64), full: bool) {
    let danger = is_dangerous(w);
    for s in &w.sites {
        if (s.kind != "workshop" && s.kind != "bowyer") || s.state != "built" || (danger && outpost(w, s.x)) {
            continue;
        }
        if !full && fetching(w, ti, "fetchBow", s.id) < s.bows {
            consider("fetchBow", s.id, 0, s.x); // Waffen nur unter dem Limit, Hämmer immer
        }
        if !full && fetching(w, ti, "fetchSword", s.id) < s.swords {
            consider("fetchSword", s.id, 0, s.x);
        }
        if fetching(w, ti, "fetchHammer", s.id) < s.hammers {
            consider("fetchHammer", s.id, 0, s.x);
        }
    }
}

/// Go: `shelfStocked`: Liegt in irgendeiner gebauten Werkstatt ein Bogen oder Schwert im Regal? Sonst holt niemand
/// etwas.
pub fn shelf_stocked(w: &World) -> bool {
    w.sites.iter().any(|s| {
        (s.kind == "workshop" || s.kind == "bowyer")
            && s.state == "built"
            && (s.bows > 0 || s.swords > 0 || s.hammers > 0)
    })
}

/// Go: `fetching` zählt die Bauern außer `ti` mit Auftrag `typ` an `id` (Platz bzw. Drop).
pub fn fetching(w: &World, ti: usize, typ: &str, id: i64) -> i64 {
    w.troops
        .iter()
        .enumerate()
        .filter(|&(oi, o)| {
            oi != ti
                && o.job
                    .as_ref()
                    .is_some_and(|j| j.r#type == typ && (j.site_id == id || j.drop_id == id))
        })
        .count() as i64
}

/// Go: `fetchesFighterWeapon`: Holt der Auftrag eine Kämpfer-Waffe (Regal oder Kämpfer-Ausrüstung am Boden)?
pub fn fetches_fighter_weapon(w: &World, v: &WorldValues, j: &Job) -> bool {
    if j.r#type == "pickup" {
        return drop_by_id(w, j.drop_id).is_some_and(|di| is_fighter(&v.troops, &w.drops[di].kind));
    }
    j.r#type == "fetchBow" || j.r#type == "fetchSword"
}

/// Go: `dropByID`: Index des Drops mit der ID.
pub fn drop_by_id(w: &World, id: i64) -> Option<usize> {
    w.drops.iter().position(|d| d.id == id)
}

/// Go: `fetchWeapon`: zum Regal bzw. zur Ausrüstung laufen, nehmen und damit Kämpfer werden (oder den Beruf
/// zurückbekommen).
pub fn fetch_weapon(w: &mut World, v: &WorldValues, ti: usize, dt: f64) {
    let near = v.troops.get("peasant").map_or(0.0, |p| p.fetch_enemy_distance);
    if enemy_within(w, w.troops[ti].x, near) {
        w.troops[ti].job = None; // ein Gegner kommt näher: nicht mitten im Kampf holen (kampf.md § 4)
        return;
    }
    let Some(j) = w.troops[ti].job.as_ref() else {
        return;
    };
    let (pickup, site_id, drop_id) = (j.r#type == "pickup", j.site_id, j.drop_id);
    let kind = Fetch::of(&j.r#type);
    let site = w.sites.iter().position(|s| s.id == site_id);
    let drop = drop_by_id(w, drop_id);
    let x = match (pickup, drop, site) {
        (true, Some(di), _) => w.drops[di].x,
        (false, _, Some(si)) if w.sites[si].state == "built" => w.sites[si].x,
        _ => {
            w.troops[ti].job = None;
            return;
        }
    };
    if !walk_to(w, v, ti, x, dt) {
        return;
    }
    w.troops[ti].job = None;
    if let Some(di) = drop {
        let d = w.drops.remove(di);
        equip(w, v, ti, &d.kind, d.work_site);
        return;
    }
    let Some(si) = site else {
        return;
    };
    match kind {
        Fetch::Bow if w.sites[si].bows > 0 => {
            w.sites[si].bows -= 1;
            equip(w, v, ti, "archer", 0);
        }
        Fetch::Hammer if w.sites[si].hammers > 0 => {
            w.sites[si].hammers -= 1;
            equip(w, v, ti, HAMMER_KIND, 0);
        }
        Fetch::Sword if w.sites[si].swords > 0 => {
            w.sites[si].swords -= 1;
            equip(w, v, ti, "warrior", 0);
        }
        _ => {}
    }
}

/// Die Regal-Art eines Holauftrags.
#[derive(Clone, Copy)]
enum Fetch {
    Bow,
    Hammer,
    Sword,
    Other,
}

impl Fetch {
    fn of(typ: &str) -> Self {
        match typ {
            "fetchBow" => Self::Bow,
            "fetchHammer" => Self::Hammer,
            "fetchSword" => Self::Sword,
            _ => Self::Other,
        }
    }
}

/// Go: `equip` rüstet `w.troops[ti]` mit `kind` aus: Hammer, Beruf oder Kämpfer-Figur.
pub fn equip(w: &mut World, v: &WorldValues, ti: usize, kind: &str, work_site: i64) {
    if kind == HAMMER_KIND {
        w.troops[ti].hammer = true;
        return;
    }
    if !is_fighter(&v.troops, kind) {
        let t = &mut w.troops[ti];
        t.profession = kind.to_string();
        t.work_site = work_site;
        return;
    }
    make_fighter(w, ti, kind);
    w.events.push(event(json!({"type": "armed"})));
}

/// Go: `warriorPost`: innen vor der äußersten gebauten Mauer der Seite (das Tor, B-652); ohne Mauer wie ein
/// Bogenschütze ohne Mauer. `front` ist der Ort, von dem aus der Krieger am Posten zuschlägt: die Sperre selbst, damit
/// er Gegner trifft, die an ihr stehen (ohne Mauer der Posten).
pub fn warrior_post(w: &World, v: &WorldValues, side: f64) -> (f64, f64) {
    match outer_wall(w, side) {
        None => {
            let post = w.hub_x + side * 10.0;
            (post, post)
        }
        Some(wall) => {
            let units = v.troops.get("warrior").map_or(0.0, |t| t.post_units);
            (wall.x - side * units, wall.x)
        }
    }
}

/// Go: `stepWarrior`: zum Posten, dann den nächsten Gegner in Reichweite schlagen. Gemessen wird wie bei Gegnern an
/// einer Mauer (Reichweite + Körper + 0.1, `buildingTargets`), am Posten von der Sperre aus, sonst von der eigenen
/// Position.
pub fn step_warrior(w: &mut World, v: &WorldValues, isl: Option<&IslandRef>, ti: usize, dt: f64) {
    let side = if w.troops[ti].anchor_x < w.hub_x { -1.0 } else { 1.0 };
    let (mut post, mut front) = warrior_post(w, v, side);
    if let Some(ex) = escort_post(w, v, isl, &w.troops[ti]) {
        (post, front) = (ex, ex); // Begleitschutz: der Krieger geht zum Endboss (unit_escort.go)
    } else if let Some(sx) = squad_post(w, v, &w.troops[ti]) {
        (post, front) = (sx, sx); // Angriff der Kampfvariante (combat.go)
    }
    let mut from = w.troops[ti].x;
    if walk_to(w, v, ti, post, dt) {
        from = front;
    }
    let t = &mut w.troops[ti];
    t.cooldown = (t.cooldown - dt).max(0.0);
    if t.cooldown > 0.0 {
        return;
    }
    let fallback = TroopData::default();
    let d = v.troops.get(&t.kind).unwrap_or(&fallback);
    let (range, damage, rate) = (d.range, d.damage, d.attacks_per_second);
    let Some(ei) = nearest(&w.enemies, |e| e.x, from, range + BODY + 0.1, |_| true) else {
        return;
    };
    let id = w.enemies[ei].id;
    apply_damage(w, id, damage);
    w.troops[ti].cooldown = 1.0 / rate;
}

/// Go: `loseLayer`: Verlust für eine Truppe mit HP ≤ 0 (Q67, kampf.md § 4): Mit Ausrüstung (Kämpfer, Beruf) fällt sie
/// zu Boden und der Bürger wird Bauer. Eine Kaskade bis zum Landstreicher gibt es nicht mehr: ein Bauer ohne Ausrüstung
/// behält Status und Münze und flieht zur Burg (unit_flee.go). Volle HP danach.
pub fn lose_layer(w: &mut World, v: &WorldValues, ti: usize) {
    release_job(w, ti);
    w.troops[ti].tower_id = None;
    w.troops[ti].cooldown = 0.0;
    drop_hammer(w, ti);
    // Ausrüstung: Figur eines Kämpfers, sonst Beruf ("" = keine)
    let kind = if is_fighter(&v.troops, &w.troops[ti].kind) {
        w.troops[ti].kind.clone()
    } else {
        w.troops[ti].profession.clone()
    };
    if !kind.is_empty() {
        let (id, x, work_site) = (w.new_id(), w.troops[ti].x, w.troops[ti].work_site);
        let cause = cause_or_other(&w.troops[ti].hit_by).to_string();
        w.drops.push(ItemDrop {
            id,
            kind: kind.clone(),
            x,
            work_site,
        });
        emit(
            w,
            "disarmed",
            event(json!({"kind": kind, "x": unit_x(x), "cause": cause})),
        );
        let hub = w.hub_x;
        let t = &mut w.troops[ti];
        t.kind = "peasant".into();
        t.profession.clear();
        t.work_site = 0;
        t.anchor_x = hub;
    }
    leave_squad(w, ti);
    let t = &mut w.troops[ti];
    t.fleeing = t.kind == "peasant";
    t.hp = v.troops.get(&t.kind).map_or(0.0, |d| d.hp);
    t.max_hp = t.hp;
}
