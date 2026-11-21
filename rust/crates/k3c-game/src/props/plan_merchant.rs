//! Händler und seine zwei Stände (HB1.2, B-1187): Der Händler steht nur, solange die Sim ihn führt (`w.merchant`),
//! mit Kauf-Stand (Gold gegen Material) und Verkauf-Stand (Material gegen Gold) an den Orten der Sim (`merchant_x`).
//! Über jedem Stand steht das Icon seines Materials, über dem Händler sein Lebensbalken, wenn er verletzt ist.

use k3c_sim::merchant::merchant_x;
use k3c_sim::state::World;

use super::buildings::Look;
use super::plan::{DARK, GOLD, HP_GREEN, Item, Planner, Shape, px, visible};
use super::plan_cost::resource_icon;
use crate::view::cell::Camera;

/// STALL ist Breite und Höhe der Tischplatte eines Stands (px) und `ROOF` die Höhe des Dachs.
const STALL: (f64, f64) = (90.0, 42.0);
const ROOF: f64 = 10.0;
const WOOD: [u8; 4] = [139, 94, 60, 255];
const CLOTH: [u8; 4] = [199, 91, 78, 255];
const CLOTH_SELL: [u8; 4] = [76, 130, 175, 255];

impl Planner {
    /// `merchant`: beide Stände und der Händler dazwischen, wenn er anwesend ist.
    pub fn merchant(&mut self, w: &World, k: &Camera) {
        let Some(m) = &w.merchant else {
            return;
        };
        for (side, cloth) in [("buy", CLOTH), ("sell", CLOTH_SELL)] {
            let x = merchant_x(w, side);
            if !visible(k, x, 3.0) {
                continue;
            }
            self.stall(x, cloth);
            let icon = Look::new(resource_icon(&m.resource), "idle");
            if self.size(icon).is_some() {
                self.top.push(Item {
                    shape: Shape::Sprite,
                    look: icon,
                    x,
                    y: px(STALL.1 + ROOF + 36.0),
                    w: 0.7,
                    ..Default::default()
                });
            }
            if side == "buy" && m.hp < m.max_hp && m.max_hp > 0.0 {
                self.bar(x, px(STALL.1 + ROOF + 70.0), px(60.0), m.hp / m.max_hp, HP_GREEN);
            }
        }
    }

    /// `stall`: Tisch mit Dach in der Farbe der Seite.
    fn stall(&mut self, x: f64, cloth: [u8; 4]) {
        let (w, h) = (px(STALL.0), px(STALL.1));
        Planner::rect(&mut self.body, x, 0.0, w, h, WOOD);
        Planner::rect(&mut self.body, x, h + px(26.0), w + px(10.0), px(ROOF), cloth);
        for dx in [-0.5, 0.5] {
            Planner::rect(&mut self.body, x + dx * (w - px(6.0)), 0.0, px(5.0), h + px(26.0), DARK);
        }
        self.body.push(Item {
            shape: Shape::Circle,
            x,
            y: h,
            w: px(6.0),
            clr: GOLD,
            ..Default::default()
        });
    }
}
