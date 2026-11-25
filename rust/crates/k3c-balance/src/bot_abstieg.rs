//! Abstieg (Port von `engine/balance/bot_abstieg.go`, BS11.2, B-642, Regel nach B-643): Im Insel-Lauf steigen die Bots über
//! den Tiefen-Eingang am Levelende ab, sobald ihre Stufe sicher steht: die Mauern der Linie 1 stehen auf beiden Seiten, es
//! ist heller Tag und mindestens Tag `MIN_DESCENT_DAY`. Der Hub-Ausbau gehört nicht zur Regel: Den Stein für Hub-Stufe 2
//! liefert die Höhle (Adern, Regel materialien-gebaeude.md § 1), der Weg nach unten kommt also vor dem Ausbau. Jeder Spieler
//! wechselt allein (Einzelwechsel, stufen.md § 1). Mit mehreren Monarchen steigen zuerst nur die mit ungeradem Index ab
//! (Rollen wie bei coop2: Monarch 2 holt das Material, Monarch 1 hält den Hub), sonst fiele der verlassene Hub schon in
//! Nacht 3 (BS3.4, run-3); der Hub-Halter folgt erst, wenn sein Heer steht (`may_descend`, B-651). Auf der neuen Stufe
//! spielt der Bot sein Profil weiter: Mauern und Bauten gelten je Stufe.

use std::collections::BTreeMap;
use std::sync::OnceLock;

use k3c_data::Source;
use k3c_sim::island::Island;
use k3c_sim::state::site_view::site_zone;
use k3c_sim::state::{Player, PlayerCommand, ResourceNode, World};

use crate::bot_material::node_for;
use crate::bot_wirtschaft::FIGHTER_WANT;
use crate::bots::{Bot, economy_view, go_to};
use k3c_data::buildings::Cost;

/// Der früheste Tag des Abstiegs (Tag 1 gehört dem Aufbau der ersten Mauern).
const MIN_DESCENT_DAY: i64 = 2;

/// Das Holz, das im Insel-Vorrat liegen soll, bevor der Bot absteigt: Unter Tage wächst kein Holz, Mauern, Türme und
/// Werkstatt der tiefen Stufe brauchen aber welches; ohne Reserve warten sie ewig auf Material (B-652, Höhle).
const DESCENT_WOOD: i64 = 400;

/// Go: `withDescent`: der Abstieg-Dekorator um `inner`.
pub fn descent_decide(inner: &Bot, deepest: i64, isl: &Island, w: &World, p: &Player) -> PlayerCommand {
    if let Some(x) = descent_x(w, deepest)
        && may_descend(isl, w, p)
        && stage_safe(w)
    {
        if mini_defeated(isl, w.biome.depth) && (isl.stock.wood >= DESCENT_WOOD || w.biome.depth > 0) {
            return go_to(p, x, false);
        }
        // Unter Tage kommt der Miniboss mit der Welle, und die füllt der Aggressionspool. Erst die Armee: Ein Miniboss mit
        // Tausenden HP trifft sonst auf einen Hub ohne Kämpfer (BS3.4, Eisenstollen: Lava-Golem gegen Monarch und einen Bauern).
        if let Some(n) = pump_node(isl, w, p)
            && economy_view(w).fighters >= FIGHTER_WANT
        {
            return go_to(p, n.x, true);
        }
    }
    inner.decide(isl, w, p)
}

/// Ein unmarkierter Knoten (Baum, Fels, Ader), den der Bot markiert, damit die Bauern sammeln: Unter Tage füllt jede
/// gesammelte Ressource den Aggressionspool (stufen.md § 2), bei vollem Pool kommt die Welle mit dem Miniboss. Nur dort
/// (Stufen mit Aggressionspool); im Wald kommt die Welle mit der Nacht von selbst.
pub fn pump_node<'a>(isl: &Island, w: &'a World, p: &Player) -> Option<&'a ResourceNode> {
    if w.aggression.is_none() || w.cycle.phase != "day" {
        return None;
    }
    let big = 1 << 20;
    let need = Cost {
        wood: big,
        stone: big,
        copper: big,
        iron: big,
        crystal: big,
        ..Cost::default()
    };
    node_for(isl, w, p, &need)
}

/// Der Miniboss der Stufe ist besiegt (oder die Stufe hat keinen). Wer vorher geht, lässt die Stufe mit ihrer Welle allein,
/// der Boss kommt und der Hub fällt (BS3.4, Test `TestFullGame3AlleMinibosse`).
pub fn mini_defeated(isl: &Island, depth: i64) -> bool {
    mini_boss_id(depth).is_none_or(|id| isl.defeated_bosses.iter().any(|d| d == id))
}

/// Die ID des Minibosses der Tiefe aus data/bosses.json (`None`: keiner). Die Datei ist fest (eingebettet, kein
/// Datenordner) und wird einmal je Prozess gelesen (B-654).
pub fn mini_boss_id(depth: i64) -> Option<&'static str> {
    mini_boss_ids().get(&depth).map(String::as_str)
}

fn mini_boss_ids() -> &'static BTreeMap<i64, String> {
    static IDS: OnceLock<BTreeMap<i64, String>> = OnceLock::new();
    IDS.get_or_init(|| {
        let mut ids = BTreeMap::new();
        let Ok(raw) = Source::Embedded.read("bosses.json") else {
            return ids;
        };
        let Ok(file) = serde_json::from_str::<serde_json::Value>(&raw) else {
            return ids;
        };
        let list = file
            .get("bosses")
            .and_then(|b| b.as_array())
            .map_or(&[][..], Vec::as_slice);
        for b in list {
            let (id, kind) = (
                b.get("id").and_then(|v| v.as_str()),
                b.get("kind").and_then(|v| v.as_str()),
            );
            let depth = b.get("depth").and_then(|v| v.as_i64()).unwrap_or(0);
            if let (Some(id), Some("mini")) = (id, kind) {
                ids.entry(depth).or_insert_with(|| id.to_string());
            }
        }
        ids
    })
}

/// Ein einzelner Monarch geht immer, bei mehreren nur die mit ungeradem Index.
pub fn may_descend(isl: &Island, w: &World, p: &Player) -> bool {
    if isl.player_count() == 1 || p.index % 2 == 1 {
        return true;
    }
    // Der Hub-Halter folgt, sobald sein Heer den Hub allein hält (B-651): Zwei Monarchen unten verdoppeln das Morgengold der
    // tiefen Stufe, sonst reicht es dort nie für Bögen gegen die Welle.
    let e = economy_view(w);
    e.fighters >= FIGHTER_WANT && e.hub_level >= 2
}

/// Der Tiefen-Eingang der Stufe, wenn es eine tiefere Stufe der Insel gibt.
pub fn descent_x(w: &World, deepest: i64) -> Option<f64> {
    if w.biome.depth >= deepest {
        return None;
    }
    w.level.entities.iter().find(|e| e.kind == "exit").map(|e| e.x)
}

/// Mauern der innersten Linie auf beiden Seiten gebaut, heller Tag, ab Tag `MIN_DESCENT_DAY`.
pub fn stage_safe(w: &World) -> bool {
    w.cycle.phase == "day" && w.cycle.day >= MIN_DESCENT_DAY && inner_walls_built(w)
}

/// Je Seite mit Mauer-Plätzen steht mindestens eine gebaute Mauer der Zone 1 (B-652).
pub fn inner_walls_built(w: &World) -> bool {
    // Index 0 = rechts, 1 = links: gibt es Mauer-Plätze der Zone 1 auf der Seite, und steht eine davon gebaut?
    let (mut sides, mut built) = ([false; 2], [false; 2]);
    for s in &w.sites {
        if site_zone(w, s) == Some(1) && s.kind == "wall" {
            let side = usize::from(s.x < w.hub_x);
            sides[side] = true;
            built[side] = built[side] || s.state == "built";
        }
    }
    let any = sides.iter().any(|&b| b);
    any && (0..2).all(|i| !sides[i] || built[i])
}

/// Die tiefste Tiefe der Stufen der Insel.
pub fn deepest_of(isl: &Island) -> i64 {
    isl.stages.iter().map(|w| w.biome.depth).fold(0, i64::max)
}
