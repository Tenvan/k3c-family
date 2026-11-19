//! Die Doppel-Truhe im Aufheb-Schritt (RU62.3): wartet auf den zweiten Monarchen oder öffnet mit Gold x `goldFactor`.
use serde_json::json;

use super::is_double_chest;
use crate::economy::fountain_coins;
use crate::monarch::chest_skill_point;
use crate::state::common::event;
use crate::state::coop_view::CoopChest;
use crate::state::island_ref::IslandRef;
use crate::state::{Pickup, World, WorldValues};

/// Beginn des Aufheb-Schritts: der Zustand für die Anzeige wird je Takt neu gefüllt (die Kapazität bleibt).
pub fn begin(w: &mut World) {
    w.coop.chests.clear();
}

/// Wirkt das Kennzeichen jetzt (Welt mit Koop-Regeln, mindestens zwei gesteuerte Monarchen)?
pub fn is_on(w: &World) -> bool {
    w.coop_active && w.coop_cfg.is_some()
}

/// Ist die Truhe `pk` eine Doppel-Truhe? Das Ergebnis liegt je Truhen-ID im Zwischenspeicher (kein Rechnen und keine
/// Zuteilung je Takt).
pub fn is_marked(w: &mut World, pk: &Pickup) -> bool {
    if let Some((_, m)) = w.coop.marks.iter().find(|(id, _)| *id == pk.id) {
        return *m;
    }
    let every = w.coop_cfg.as_ref().map_or(0, |c| c.double_chest.every_nth);
    let m = every > 0 && is_double_chest(&w.seed, w.biome.depth, pk.x, every);
    w.coop.marks.push((pk.id, m));
    m
}

/// Behandelt die Doppel-Truhe `pk` in diesem Takt; `true` = sie bleibt liegen. Zwei gesteuerte Monarchen der Stufe im
/// Aufheb-Radius `reach` (mit oder ohne Krone) öffnen sie: Gold = normales Truhen-Gold x `goldFactor` als Fontäne,
/// `chest_skill_point` für beide. Steht nur einer davor, bleibt sie zu und der Zustand nennt ihn als wartend.
pub fn try_open(w: &mut World, v: &WorldValues, mut isl: Option<&mut IslandRef>, pk: &Pickup, reach: f64) -> bool {
    let mut near = [0usize; 2];
    let mut n = 0;
    for (i, p) in w.players.iter().enumerate() {
        if p.free || (p.x - pk.x).abs() > reach {
            continue;
        }
        if n < 2 {
            near[n] = i;
        }
        n += 1;
    }
    let factor = w.coop_cfg.as_ref().map_or(1.0, |c| c.double_chest.gold_factor);
    if n < 2 {
        let waiting = (n == 1).then(|| w.players[near[0]].index);
        w.coop.chests.push(CoopChest {
            id: pk.id,
            x: pk.x,
            double: true,
            waiting,
        });
        return true;
    }
    let base = w.rng.int(v.economy.chest_gold[0], v.economy.chest_gold[1]);
    let gold = (base as f64 * factor).round() as i64;
    fountain_coins(w, v, pk.x, gold);
    let (a, b) = (w.players[near[0]].index, w.players[near[1]].index);
    w.events.push(event(
        json!({"type": "chest", "player": a, "partner": b, "gold": gold, "double": true}),
    ));
    chest_skill_point(w, v, isl.as_deref_mut(), near[0]);
    chest_skill_point(w, v, isl, near[1]);
    false
}
