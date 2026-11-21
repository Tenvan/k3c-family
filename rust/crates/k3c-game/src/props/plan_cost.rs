//! Preis-Anzeige der Bauplätze (HB1.1, B-1186): Münz-Plätze in Reihen, darüber die Kosten als Rohstoff-Icons mit Zahl
//! (statt „20 Holz“) und der kleine Name (bei der Burg „x/n“ des Ausbaus). Fehlt ein Icon-Sheet, steht der Kurztext.

use k3c_data::buildings::Cost;

use super::buildings::Look;
use super::data::{cost_of, materials, resource_name};
use super::plan::{GOLD, Item, POOR_RED, Planner, Shape, WHITE, px};

/// ICON_ANIM ist die Animation der Icons; `RESOURCE_ICONS` ordnet jedem Rohstoff sein Icon zu (wie `hud/icons.rs`;
/// `props` darf `hud` nicht importieren, die Icons meldet das HUD für jedes Level).
const ICON_ANIM: &str = "idle";
const RESOURCE_ICONS: [(&str, &str); 5] = [
    ("wood", "icons/icon-holz"),
    ("stone", "icons/icon-stein"),
    ("copper", "icons/icon-kupfer"),
    ("iron", "icons/icon-eisen"),
    ("crystal", "icons/icon-kristall"),
];

/// `item_icon` ist das Icon des Gegenstands, den der Hersteller `kind` ins Regal legt (EQ1.2; wie `ITEM_ICONS` im HUD,
/// `props` darf `hud` nicht importieren): Werkstatt Gürtel, Bogner Bogen, Waffenschmied Schwert, Rüstungsschmied
/// Rüstung; "" für jeden anderen Platz.
pub(super) fn item_icon(kind: &str) -> &'static str {
    match kind {
        "workshop" => "icons/icon-guertel",
        "bowyer" => "icons/icon-bogen",
        "smithy" => "icons/icon-schwert",
        "armory" => "icons/icon-ruestung",
        _ => "",
    }
}

pub(super) fn resource_icon(res: &str) -> &'static str {
    RESOURCE_ICONS.iter().find(|(r, _)| *r == res).map_or("", |(_, a)| a)
}

/// SMALL ist die Texthöhe der Baustellen in Units (vorher 1: 32 Spiel-Pixel).
pub const SMALL: f64 = 0.55;
/// ICON ist die Kantenlänge der Kosten-Icons in Units (das Icon-Bild ist 1 Unit groß).
const ICON: f64 = 0.55;
/// GAP ist der Abstand zwischen Icon und Zahl und zwischen zwei Kosten.
const GAP: f64 = 0.12;
/// DIGIT ist die Breite einer Ziffer in Units bei der Texthöhe `SMALL`.
const DIGIT: f64 = 0.3;

impl Planner {
    /// `coin_rows`: Münz-Plätze in Reihen zu zehn (gefüllt = bezahlt); liefert die Höhe der Zeile darüber.
    pub fn coin_rows(&mut self, x: f64, y: f64, total: i64, paid: i64) -> f64 {
        const PER_ROW: i64 = 10;
        let (step, r) = (px(22.0), px(8.0));
        for i in 0..total {
            let row = i / PER_ROW;
            let in_row = PER_ROW.min(total - row * PER_ROW);
            let cx = x + (i % PER_ROW) as f64 * step - (in_row - 1) as f64 * step / 2.0;
            let cy = y + row as f64 * step;
            if i < paid {
                self.top.push(Item {
                    shape: Shape::Circle,
                    x: cx,
                    y: cy,
                    w: r,
                    clr: GOLD,
                    ..Default::default()
                });
            }
            self.top.push(Item {
                shape: Shape::Ring,
                x: cx,
                y: cy,
                w: r,
                size: px(2.0),
                clr: GOLD,
                ..Default::default()
            });
        }
        y + ((total + PER_ROW - 1) / PER_ROW) as f64 * step
    }

    /// `price`: Münz-Plätze, darüber ein Text; rot, wenn Material fehlt.
    pub fn price(&mut self, x: f64, y: f64, total: i64, paid: i64, text: &str, enough: bool) {
        let ty = self.coin_rows(x, y, total, paid);
        let clr = if enough { WHITE } else { POOR_RED };
        self.label(x, ty, text, clr);
    }

    /// `price_cost`: Münz-Plätze (`coins` = Summe, bezahlt), darüber die Kosten als Icons mit Zahl und der kleine Name.
    pub fn price_cost(&mut self, x: f64, y: f64, coins: (i64, i64), name: &str, cost: &Cost, enough: bool) {
        let ty = self.coin_rows(x, y, coins.0, coins.1);
        let clr = if enough { WHITE } else { POOR_RED };
        let mats = materials(cost);
        let looks: Vec<Look> = mats.iter().map(|r| Look::new(resource_icon(r), ICON_ANIM)).collect();
        if looks.iter().all(|l| self.size(*l).is_some()) {
            self.cost_row(x, ty, cost, (&mats, &looks), clr);
        } else {
            let short: Vec<String> = mats
                .iter()
                .map(|r| format!("{} {}", cost_of(cost, r), resource_name(r).unwrap_or("")))
                .collect();
            self.label_sized(x, ty, &short.join(", "), clr, SMALL);
        }
        self.label_sized(x, ty + ICON + GAP, name, clr, SMALL);
    }

    /// `shelf_icons`: je Stück im Regal ein Icon des Gegenstands (EQ1.2), links beginnend bei `x - 40`, Abstand 30 Spiel-
    /// Pixel; `false`, wenn das Icon-Sheet fehlt (dann bleibt der Rückfall des Aufrufers).
    pub fn shelf_icons(&mut self, x: f64, kind: &str, count: i64) -> bool {
        let look = Look::new(item_icon(kind), ICON_ANIM);
        if self.size(look).is_none() {
            return false;
        }
        for i in 0..count {
            self.top.push(Item {
                shape: Shape::Sprite,
                look,
                x: x - px(40.0) + i as f64 * px(30.0),
                y: px(46.0),
                w: px(30.0),
                ..Default::default()
            });
        }
        true
    }

    /// `item_tag`: das Icon des Gegenstands links neben dem Preisschild am Regal (EQ1.2).
    pub fn item_tag(&mut self, x: f64, y: f64, kind: &str) {
        let look = Look::new(item_icon(kind), ICON_ANIM);
        if self.size(look).is_some() {
            self.top.push(Item {
                shape: Shape::Sprite,
                look,
                x: x - px(56.0),
                y,
                w: px(26.0),
                ..Default::default()
            });
        }
    }

    /// `cost_row`: Icon und Zahl je Rohstoff, die Reihe mittig um x, Unterkante y.
    fn cost_row(&mut self, x: f64, y: f64, cost: &Cost, (mats, looks): (&[&str], &[Look]), clr: [u8; 4]) {
        let nums: Vec<String> = mats.iter().map(|r| cost_of(cost, r).to_string()).collect();
        let width = |n: &str| ICON + GAP + DIGIT * n.len() as f64;
        let total: f64 = nums.iter().map(|n| width(n)).sum::<f64>() + GAP * nums.len().saturating_sub(1) as f64;
        let mut cx = x - total / 2.0;
        for (n, look) in nums.iter().zip(looks) {
            self.top.push(Item {
                shape: Shape::Sprite,
                look: *look,
                x: cx + ICON / 2.0,
                y,
                w: ICON,
                ..Default::default()
            });
            self.label_sized(cx + ICON + GAP + DIGIT * n.len() as f64 / 2.0, y, n, clr, SMALL);
            cx += width(n) + GAP;
        }
    }
}
