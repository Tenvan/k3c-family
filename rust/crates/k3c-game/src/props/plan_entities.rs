//! Port von `native/k3c-game/props/plan.go` (Teil 3: feste Objekte und Natur): `entities`, `exit_sign`,
//! `entity_shape`, `nodes`, `node_shape`, `plantation_tree`.

use k3c_sim::state::{ResourceNode, World};

use super::data::values;
use super::nature::{anim_phase, deco_roots, entity_look, mine_sign_look, node_look, object, on_plot, variant_look};
use super::plan::{GOLD, Item, Planner, Shape, WHITE, px, visible};
use super::plan_combat::portal_down;
use super::plan_objects::fell_shrink;
use super::sites::{TextKey, TextWhat};
use crate::view::cell::Camera;

/// `PENNANT_MAX`: höchste Lage von Balken über Knoten (Baumhöhe, pennant.ts › PENNANT_MAX_HEIGHT).
pub fn pennant_max() -> f64 {
    px(360.0)
}

fn circle(x: f64, y: f64, w: f64, clr: [u8; 4]) -> Item {
    Item {
        shape: Shape::Circle,
        x,
        y,
        w,
        clr,
        ..Default::default()
    }
}

fn tri(x: f64, y: f64, w: f64, h: f64, clr: [u8; 4]) -> Item {
    Item {
        shape: Shape::Tri,
        x,
        y,
        w,
        h,
        clr,
        ..Default::default()
    }
}

impl Planner {
    /// `entities`: feste Objekte des Levels (Portal, Ausgang mit Schild, Busch, Hafen-Steg, Rekrutierungslager).
    pub fn entities(&mut self, w: &World, k: &Camera, biome: &str) {
        for e in &w.level.entities {
            let Some((l, dx)) = entity_look(&e.kind, biome) else {
                continue;
            };
            let x = e.x + dx;
            if !visible(k, x, 4.0) || (e.kind == "portal" && portal_down(w, e.x)) {
                continue;
            }
            if !self.sprite(l, x, 0.0, anim_phase(x, 8)) {
                self.entity_shape(&e.kind, x);
            }
            if e.kind == "exit" {
                self.exit_sign(w, l, x);
            }
        }
    }

    /// `exit_sign`: „Tiefe n / alle hierher“ über dem Ausgang, ohne tiefere Stufe „verschüttet“.
    pub fn exit_sign(&mut self, w: &World, l: super::buildings::Look, x: f64) {
        let h = self.size(l).map_or_else(|| px(180.0), |(_, h)| h);
        let mut n = w.biome.depth + 1;
        if !values().depths.contains(&n) {
            n = -1;
        }
        let y = h + px(24.0);
        if n < 0 {
            self.label(x, y, "verschüttet", WHITE);
            return;
        }
        let mut key = TextKey::new(TextWhat::Exit);
        key.n = n;
        let mut texts = std::mem::take(&mut self.texts);
        let line = texts.get(&key, |k| format!("Tiefe {}", k.n));
        self.label(x, y + 1.1, line, WHITE);
        self.label(x, y, "alle hierher", WHITE);
        self.texts = texts;
    }

    /// `entity_shape`: Platzhalter-Formen ohne Grafik (worldRenderer.ts › drawStatic).
    pub fn entity_shape(&mut self, kind: &str, x: f64) {
        match kind {
            "portal" => Planner::rect(&mut self.body, x, 0.0, px(110.0), px(220.0), [123, 44, 191, 255]),
            "exit" => Planner::rect(&mut self.body, x, 0.0, px(160.0), px(180.0), [17, 17, 17, 255]),
            "harbor" => Planner::rect(&mut self.body, x, 0.0, 6.0, px(24.0), [139, 90, 43, 255]),
            "recruitCamp" => self.body.push(tri(x, 0.0, px(120.0), px(90.0), [233, 196, 106, 255])),
            _ => self.body.push(circle(x, px(14.0), px(18.0), [64, 145, 108, 255])),
        }
    }

    /// `nodes`: Bäume, Felsen, Erz, Büsche und Adern mit Deko-Wurzeln, Abbau-Schild und Fortschritt; Plantage-Bäume später.
    pub fn nodes(&mut self, w: &World, k: &Camera, biome: &str) {
        for n in &w.nodes {
            if !visible(k, n.x, 3.0) || self.plantation_tree(w, n) {
                continue;
            }
            if n.kind == "tree" {
                self.roots.clear();
                let per = values().roots_by_biom.get(biome).copied().unwrap_or(0);
                deco_roots(&mut self.roots, n.x, biome, per);
                for i in 0..self.roots.len() {
                    self.sprite(object("wurzel"), n.x + self.roots[i], 0.0, 0);
                }
            }
            let l = variant_look(&n.kind, &n.variant, biome, n.x).unwrap_or_else(|| node_look(&n.kind, n.x, biome));
            let h = if let Some((_, h)) = self.size(l) {
                self.body.push(Item {
                    shape: Shape::Sprite,
                    look: l,
                    x: n.x,
                    phase: anim_phase(n.x, 8),
                    sy: fell_shrink(n.progress),
                    ..Default::default()
                });
                h
            } else {
                self.node_shape(&n.kind, n.x);
                px(158.0)
            };
            if n.marked && !self.sprite(mine_sign_look(&n.kind), n.x - px(24.0), 0.0, anim_phase(n.x, 4)) {
                Planner::rect(
                    &mut self.body,
                    n.x - px(24.0),
                    px(14.0),
                    px(40.0),
                    px(40.0),
                    [176, 137, 104, 255],
                );
            }
            if n.progress > 0.0 {
                self.bar(n.x, h.min(pennant_max()) + px(28.0), px(50.0), n.progress, GOLD);
            }
        }
    }

    /// `node_shape`: Platzhalter ohne Grafik (Baum aus Stamm und Krone, Fels, Erz, Busch).
    pub fn node_shape(&mut self, kind: &str, x: f64) {
        match kind {
            "tree" => {
                Planner::rect(&mut self.body, x, 0.0, px(14.0), px(80.0), [107, 66, 38, 255]);
                self.body.push(tri(x, px(65.0), px(80.0), px(90.0), [45, 106, 79, 255]));
            }
            "rock" => self.body.push(circle(x, px(16.0), px(22.0), [141, 153, 174, 255])),
            "copperOre" => self.body.push(circle(x, px(16.0), px(20.0), [184, 115, 51, 255])),
            _ => self.body.push(circle(x, px(14.0), px(18.0), [64, 145, 108, 255])),
        }
    }

    /// `plantation_tree` meldet, ob der Knoten ein Baum auf einem Platz einer gebauten Baumschule ist (plantation.ts).
    pub fn plantation_tree(&self, w: &World, n: &ResourceNode) -> bool {
        if n.kind != "tree" {
            return false;
        }
        let (slots, strip) = values()
            .buildings
            .get("nursery")
            .map_or((0, 0.0), |b| (b.plantation.slots, b.plantation.strip_units));
        w.sites.iter().any(|s| {
            s.kind == "nursery"
                && s.state == "built"
                && (n.x - s.x).abs() <= strip + 1.0
                && on_plot(n.x, s.x, slots, strip)
        })
    }
}
