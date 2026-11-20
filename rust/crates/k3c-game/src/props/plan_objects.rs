//! Port von `native/k3c-game/props/plan_objects.go`. Zeichenliste der Plantage, Truhen, Sterne und Münzen
//! (worldRenderer.ts). Die Fackeln (Halter und Flamme) zeichnet world über der Lichtkarte (lightView.ts › drawTorches,
//! NG2.3).

use k3c_sim::state::{ResourceNode, World};

use super::buildings::Look;
use super::coin_fountain::coin_pose;
use super::data::values;
use super::listen::{StageInfo, open_alpha};
use super::nature::{
    COIN_LOOK, PLOT_TOLERANCE, anim_phase, chest_open_look, object, pickup_look, plant_tree_look, plot_x,
    seedling_look, young_tree_look,
};
use super::plan::{GOLD, Item, Planner, Shape, px, visible};
use super::plan_entities::pennant_max;
use crate::view::cell::Camera;

/// `fell_shrink` ist der vertikale Maßstab eines Baums beim Fällen: bis zu 30 % niedriger (worldRenderer.ts ›
/// updateNode: `setScale(1, 1 - progress * 0.3)`).
pub fn fell_shrink(progress: f64) -> f64 {
    1.0 - progress * 0.3
}

/// `young_stage`: Der Platz zeigt den Jungbaum, sobald die Hälfte der Wachszeit vorbei ist (B-1149); `grow_in` ist die
/// Restzeit, 0 heißt: wächst nicht.
pub fn young_stage(grow_in: f64, grow_seconds: f64) -> bool {
    grow_seconds > 0.0 && grow_in > 0.0 && grow_in <= grow_seconds * 0.5
}

/// `tree_at` liefert den Baum-Knoten, der auf dem Platz x steht (`None`: keiner).
pub fn tree_at(nodes: &[ResourceNode], x: f64) -> Option<&ResourceNode> {
    nodes
        .iter()
        .find(|n| n.kind == "tree" && (n.x - x).abs() < PLOT_TOLERANCE)
}

impl Planner {
    /// `plantation`: Setzling auf leeren Plätzen, niedrige Pappel auf gewachsenen (GEB5, vor der Baumschule).
    pub fn plantation(&mut self, w: &World, k: &Camera) {
        let (slots, strip, grow_seconds) = values().buildings.get("nursery").map_or((0, 0.0, 0.0), |b| {
            (b.plantation.slots, b.plantation.strip_units, b.plantation.grow_seconds)
        });
        for s in &w.sites {
            if s.kind != "nursery" || s.state != "built" || !visible(k, s.x, strip + 1.0) {
                continue;
            }
            for i in 0..slots {
                let x = plot_x(s.x, i, slots, strip);
                if let Some(n) = tree_at(&w.nodes, x) {
                    self.felled_tree(n, x);
                    continue;
                }
                let grow_in = s.plots.get(i as usize).map_or(0.0, |p| p.grow_in);
                let look = young_tree_look()
                    .filter(|_| young_stage(grow_in, grow_seconds))
                    .unwrap_or_else(seedling_look);
                self.sprite(look, x, 0.0, anim_phase(x, 8));
            }
        }
    }

    /// `felled_tree`: niedrige Pappel auf dem Platz; beim Fällen schrumpft sie und der Balken läuft (worldRenderer.ts ›
    /// updateNode, wie bei den Waldbäumen in `nodes`).
    pub fn felled_tree(&mut self, n: &ResourceNode, x: f64) {
        let Some((_, h)) = self.size(plant_tree_look()) else {
            return;
        };
        self.body.push(Item {
            shape: Shape::Sprite,
            look: plant_tree_look(),
            x,
            phase: anim_phase(x, 8),
            sy: fell_shrink(n.progress),
            ..Default::default()
        });
        if n.progress > 0.0 {
            self.bar(x, h.min(pennant_max()) + px(28.0), px(50.0), n.progress, GOLD);
        }
    }

    /// `pickups`: Truhen geschlossen, Sterne darüber; aufgehobene Truhen kurz offen. Eine Truhe mit wirkendem Kennzeichen
    /// (`World::coop`) ist die Doppel-Truhe mit zwei Schlössern: das des wartenden Monarchen leuchtet (RU62.4, B-998).
    pub fn pickups(&mut self, w: &World, info: Option<&StageInfo>, k: &Camera, clock: f64) {
        for pk in &w.pickups {
            let double = if pk.kind == "chest" {
                w.coop.chests.iter().find(|c| c.id == pk.id)
            } else {
                None
            };
            let (l, lift) = match double {
                Some(c) => (double_chest_look(c.waiting), 0.0),
                None => pickup_look(&pk.kind),
            };
            if !l.ok() || !visible(k, pk.x, 1.0) {
                continue;
            }
            if double.is_some() {
                self.remember_double(pk.x);
            }
            if !self.sprite(l, pk.x, lift, anim_phase(pk.x, 8)) {
                Planner::rect(&mut self.body, pk.x, lift, px(44.0), px(36.0), [156, 102, 68, 255]);
            }
        }
        let Some(info) = info else {
            return;
        };
        for o in &info.openings {
            let a = open_alpha(o, clock);
            let open = if self.double_seen.iter().any(|x| (x - o.x).abs() < 0.01) {
                double_chest_open_look()
            } else {
                chest_open_look()
            };
            if a > 0.0
                && self.sprite(open, o.x, 0.0, 0)
                && let Some(last) = self.body.last_mut()
            {
                last.alpha = a;
            }
        }
    }

    /// `remember_double`: merkt den Ort einer Doppel-Truhe (höchstens 16, die älteste fällt weg).
    fn remember_double(&mut self, x: f64) {
        if self.double_seen.iter().any(|d| (d - x).abs() < 0.01) {
            return;
        }
        if self.double_seen.len() >= 16 {
            self.double_seen.remove(0);
        }
        self.double_seen.push(x);
    }

    /// `coins`: Münzen am Boden (Papier-Münze, ohne Grafik ein goldener Kreis).
    pub fn coins(&mut self, w: &World, k: &Camera) {
        for c in &w.coins {
            let (x, lift, shown) = coin_pose(c, w.time); // Truhen-Fontäne im Bogen (B-953)
            if shown && visible(k, x, 0.5) && !self.sprite(COIN_LOOK, x, px(lift), anim_phase(c.x, 8)) {
                self.body.push(Item {
                    shape: Shape::Circle,
                    x,
                    y: px(8.0) + px(lift),
                    w: px(8.0),
                    clr: GOLD,
                    ..Default::default()
                });
            }
        }
    }
}

/// `double_chest_look`: Doppel-Truhe, Schloss des wartenden Monarchen leuchtet (0 links, 1 rechts), sonst beide dunkel.
pub fn double_chest_look(waiting: Option<i64>) -> Look {
    let anim = match waiting {
        Some(0) => "links",
        Some(1) => "rechts",
        _ => "zu",
    };
    Look::new(object("doppel-truhe").asset, anim)
}

/// `double_chest_open_look`: die aufgehobene Doppel-Truhe.
pub fn double_chest_open_look() -> Look {
    object("doppel-truhe-offen")
}
