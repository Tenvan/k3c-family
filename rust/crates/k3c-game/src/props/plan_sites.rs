//! Port von `native/k3c-game/props/plan_sites.go` (Teil 1: Plätze, Gerüst, Bauschild, Preis): Zeichenliste der Bauplätze
//! und der Hintergrundmauer. Wartegrund, Risse und Schutt stehen in `plan_sites_marks.rs`.
use std::f64::consts::PI;

use k3c_sim::state::{Site, World};

use super::buildings::{
    DAMAGE_SHADE, Look, Marker, back_wall_look, back_walls, damage_stage, scaffold_look, show_rubble, sign_variant,
    site_look, site_marker, site_stage,
};
use super::data::{build_cost, cost_of, site_name, values};
use super::listen::{SiteFlags, StageInfo, info_flag, info_tagged};
use super::nature::ship_drop;
use super::plan::{DARK, GOLD, HP_GREEN, Item, Planner, Shape, px, visible};
use super::plan_cost::SMALL;
use super::sites::{TextKey, TextWhat, WaitReason, can_afford, shelf_of, shelf_rack, site_text, site_wait};
use crate::view::cell::Camera;

/// `SITE_SIZE`: Platzhalter-Größe je Platzart in Spiel-Pixeln (siteView.ts › SITE_SIZE).
pub const SITE_SIZE: [(&str, [f64; 2]); 14] = [
    ("wall", [36.0, 150.0]),
    ("tower", [70.0, 260.0]),
    ("workshop", [150.0, 120.0]),
    ("bowyer", [130.0, 110.0]),
    ("storage", [130.0, 100.0]),
    ("stairsUp", [110.0, 110.0]),
    ("stairsDown", [110.0, 110.0]),
    ("nursery", [140.0, 90.0]),
    ("barracks", [150.0, 130.0]),
    ("tavern", [130.0, 120.0]),
    ("healer", [100.0, 100.0]),
    ("smithy", [120.0, 110.0]),
    ("armory", [130.0, 120.0]),
    ("ship", [190.0, 160.0]),
];

/// `SITE_COLOR`: Farbe der Platzhalter-Form (siteView.ts › SITE_COLOR).
pub const SITE_COLOR: [(&str, [u8; 4]); 14] = [
    ("wall", [141, 153, 174, 255]),
    ("tower", [156, 102, 68, 255]),
    ("workshop", [188, 108, 37, 255]),
    ("bowyer", [214, 150, 70, 255]),
    ("storage", [127, 85, 57, 255]),
    ("stairsUp", [108, 117, 125, 255]),
    ("stairsDown", [52, 58, 64, 255]),
    ("nursery", [167, 201, 87, 255]),
    ("barracks", [108, 88, 76, 255]),
    ("tavern", [176, 137, 104, 255]),
    ("healer", [229, 229, 229, 255]),
    ("smithy", [73, 80, 87, 255]),
    ("armory", [92, 103, 125, 255]),
    ("ship", [74, 111, 165, 255]),
];

/// `SIGN_BOARD`: Farben des Bauschilds je Variante (Holz, Zierleiste).
pub const SIGN_BOARD: [[[u8; 4]; 2]; 3] = [
    [[176, 137, 104, 255], [244, 213, 141, 255]],
    [[156, 102, 68, 255], [193, 18, 31, 255]],
    [[141, 153, 174, 255], [52, 58, 64, 255]],
];

/// `site_size` liefert die Platzhalter-Größe (Spiel-Pixel) der Platzart (Go: `siteSize[kind]`).
/// `monarch_before`: steht ein Monarch vor dem Gebäude bei `x` mit Breite `full` (HB1.2: dann steht sein Name darüber).
pub fn monarch_before(info: Option<&StageInfo>, x: f64, full: f64) -> bool {
    info.is_some_and(|i| i.players.iter().any(|p| (p - x).abs() <= full / 2.0 + 0.5))
}

pub fn site_size(kind: &str) -> Option<[f64; 2]> {
    SITE_SIZE.iter().find(|(k, _)| *k == kind).map(|(_, s)| *s)
}

/// `site_color` liefert die Farbe der Platzhalter-Form (Go: `siteColor[kind]`, fehlt = leer).
pub fn site_color(kind: &str) -> [u8; 4] {
    SITE_COLOR.iter().find(|(k, _)| *k == kind).map_or([0; 4], |(_, c)| *c)
}

/// `placeholder` liefert Breite und Höhe der Platzhalter-Form (Units).
pub fn placeholder(kind: &str) -> (f64, f64) {
    let s = site_size(kind).unwrap_or([100.0, 100.0]);
    (px(s[0]), px(s[1]))
}

impl Planner {
    /// `with_text`: Text aus dem Puffer `texts`, während `f` den Planer verändert (der Puffer liegt dafür kurz außerhalb).
    pub fn with_text<R>(&mut self, key: TextKey, f: impl FnOnce(&mut Planner, &str) -> R) -> R {
        let mut texts = std::mem::take(&mut self.texts);
        let r = f(self, texts.get(&key, site_text));
        self.texts = texts;
        r
    }

    /// `back_walls`: gekachelte Hintergrundmauer hinter den Bauplätzen; ohne Grafik bleibt der Abschnitt leer.
    /// Die Mauern einer Seite liegen als Indizes in `w.sites` im Puffer `self.walls` (keine Allokation je Bild).
    pub fn back_walls(&mut self, w: &World, k: &Camera) {
        let mut backs = std::mem::take(&mut self.backs);
        backs.clear();
        backs = back_walls(backs, &mut self.walls, &w.sites, w.hub_x);
        for b in &backs {
            let l = back_wall_look(b.stage);
            if self.size(l).is_some() && visible(k, (b.from + b.to) / 2.0, (b.to - b.from) / 2.0 + 1.0) {
                self.body.push(Item {
                    shape: Shape::Tiles,
                    look: l,
                    x: b.from,
                    x2: b.to,
                    ..Default::default()
                });
            }
        }
        self.backs = backs;
    }

    /// `sites`: alle sichtbaren Bauplätze in der Reihenfolge der Welt, Mauern zuerst.
    pub fn sites(&mut self, w: &World, info: Option<&StageInfo>, k: &Camera) {
        self.sites_pass(w, info, k, true);
        self.sites_pass(w, info, k, false);
    }

    /// `sites_pass`: nur die Mauern (`walls`) oder nur die übrigen Bauplätze. `plan` zeichnet die Mauern vor der Burg,
    /// damit sie hinter der Burg und allen Gebäuden stehen (LA1.5, 🧑 2026-11-21).
    pub fn sites_pass(&mut self, w: &World, info: Option<&StageInfo>, k: &Camera, walls: bool) {
        for (i, s) in w.sites.iter().enumerate() {
            let fl = info_flag(info, i);
            if (s.kind == "wall") != walls || !fl.visible || !visible(k, s.x, 5.0) {
                continue;
            }
            if Planner::is_hall(w, s) {
                self.hall_site(w, s, info, fl);
            } else if s.state == "built" {
                self.built_site(w, s, info, fl);
            } else {
                self.open_site(w, s, info, fl);
            }
        }
    }

    /// `built_site`: Gebäude der Stufe mit Schaden, Ruine, Treppen-Schild, Lebensbalken, Bögen und Bogen-Preis,
    /// Wartegrund.
    pub fn built_site(&mut self, w: &World, s: &Site, info: Option<&StageInfo>, fl: SiteFlags) {
        self.built_site_look(w, s, info, fl, None);
    }

    /// `built_site_look`: wie `built_site`, mit eigener Grafik (Bauwerk des Einsiedlers, RU62.11).
    pub fn built_site_look(&mut self, w: &World, s: &Site, info: Option<&StageInfo>, fl: SiteFlags, own: Option<Look>) {
        let hub_level = w.hub_level.max(1);
        let mut l = own.unwrap_or_else(|| site_look(&s.kind, &s.state, site_stage(&s.kind, s.level, hub_level)));
        if self.size(l).is_none() {
            l = site_look(&s.kind, &s.state, 1); // Stufe nicht geladen: Grundform statt leerer Fläche
        }
        let stage = damage_stage(&s.state, s.hp, s.max_hp);
        let shade = DAMAGE_SHADE[stage as usize];
        let (full, h, crack) = if let Some((full, h)) = self.size(l) {
            self.body.push(Item {
                shape: Shape::Sprite,
                look: l,
                x: s.x,
                y: -ship_drop(&s.kind, s.level),
                shade,
                ..Default::default()
            });
            (full, h, full * 0.6) // Risse auf dem Bild nur in der Mitte, auf der Form über die ganze Breite
        } else {
            let (full, h) = placeholder(&s.kind);
            self.built_shape(&s.kind, s.x, full, h);
            if stage > 0 {
                Planner::rect(&mut self.body, s.x, 0.0, full, h, [0, 0, 0, (255.0_f32 * shade) as u8]);
            }
            (full, h, full)
        };
        self.cracks(s.x, crack, h, stage);
        if show_rubble(s.ruin, &s.state, s.hp, s.max_hp) {
            self.rubble(s.x, full);
        }
        let labeled = self.built_signs(s, info, fl, h);
        let (r, st) = site_wait(&s.state, &s.upgrade, s.build_progress, info.is_some_and(|i| i.danger));
        if r != WaitReason::None && !labeled {
            self.site_status(s, r, st, s.level, s.x, h + px(20.0));
        }
        if monarch_before(info, s.x, full) {
            self.label_sized(s.x, h + px(36.0), &site_name(&s.kind), GOLD, SMALL);
        }
    }

    /// `built_shape`: Platzhalter eines gebauten Gebäudes ohne Grafik (Turm mit Dach, Treppe mit Stufen).
    pub fn built_shape(&mut self, kind: &str, x: f64, w: f64, h: f64) {
        Planner::rect(&mut self.body, x, 0.0, w, h, site_color(kind));
        self.body.push(Item {
            shape: Shape::Frame,
            x,
            w,
            h,
            size: px(4.0),
            clr: DARK,
            ..Default::default()
        });
        if kind == "tower" {
            Planner::rect(
                &mut self.body,
                x,
                h - px(4.0),
                w + px(30.0),
                px(14.0),
                [111, 69, 24, 255],
            );
        }
        if kind == "stairsUp" || kind == "stairsDown" {
            for i in 0..5 {
                let f = f64::from(i);
                Planner::rect(
                    &mut self.body,
                    x - w / 2.0 + (f + 0.5) * w / 5.0,
                    0.0,
                    w / 5.0,
                    f64::from(i + 1) * h / 5.0,
                    [73, 80, 87, 255],
                );
            }
        }
    }

    /// `built_signs`: Treppen-Schild, Lebensbalken, Bögen im Regal und Bogen-Preis; `true`, wenn ein Text am Platz steht.
    pub fn built_signs(&mut self, s: &Site, info: Option<&StageInfo>, fl: SiteFlags, h: f64) -> bool {
        let mut labeled = false;
        if s.max_hp > 0.0 && s.hp < s.max_hp {
            self.bar(s.x, h + px(22.0), px(80.0), s.hp / s.max_hp, HP_GREEN);
        }
        if shelf_rack(&s.kind) == 0 {
            return labeled;
        }
        // Bogner: Bogen, Werkstatt: Hammer (B-951)
        let price = if s.kind == "workshop" {
            values()
                .buildings
                .get("workshop")
                .map(|b| b.hammer_cost.clone())
                .unwrap_or_default()
        } else {
            values().archer_cost.clone()
        };
        for i in 0..s.bows {
            // Bögen liegen sichtbar im Regal des Bogners
            self.bow(s.x - px(40.0) + i as f64 * px(30.0));
        }
        if info_tagged(info, s.id)
            && shelf_of(s) < shelf_rack(&s.kind)
            && let Some(info) = info
        {
            let mut total = cost_of(&price, "gold");
            if fl.payable {
                total = fl.gold_left + s.bow_paid_gold;
            }
            let key = TextKey {
                kind: s.kind.clone(),
                ..TextKey::new(TextWhat::Bow)
            };
            let enough = can_afford(info.stock, &price);
            self.with_text(key, |p, text| {
                p.price_cost(s.x, h + px(20.0), (total, s.bow_paid_gold), text, &price, enough);
            });
            labeled = true;
        }
        labeled
    }

    /// `bow`: Bogen im Regal der Werkstatt als Halbkreis (rechts offen) aus Linien, Mitte 60 Spiel-Pixel hoch.
    pub fn bow(&mut self, x: f64) {
        const N: i64 = 6;
        let (r, y) = (px(14.0), px(60.0));
        for i in 0..N {
            let a0 = -PI / 2.0 + PI * i as f64 / N as f64;
            let a1 = -PI / 2.0 + PI * (i + 1) as f64 / N as f64;
            self.top.push(Item {
                shape: Shape::Line,
                x: x + r * a0.cos(),
                y: y + r * a0.sin(),
                x2: x + r * a1.cos(),
                y2: y + r * a1.sin(),
                size: px(4.0),
                clr: GOLD,
                ..Default::default()
            });
        }
    }

    /// `open_site`: Bauschild oder Gerüst, Fortschritt, Preis bzw. „erst roden“ am nächsten Platz, Wartegrund.
    pub fn open_site(&mut self, w: &World, s: &Site, info: Option<&StageInfo>, fl: SiteFlags) {
        let (bw, h) = placeholder(&s.kind);
        let mut top = h;
        match site_marker(&s.state, fl.payable) {
            Marker::Sign => self.build_sign(s.x, sign_variant(&s.kind)),
            Marker::Scaffold => top = self.scaffold(&s.kind, s.x, bw, h),
            Marker::None => {}
        }
        if s.build_progress > 0.0 {
            self.bar(s.x, top + px(4.0), px(80.0), s.build_progress, [221, 161, 94, 255]);
        }
        if s.state == "unpaid"
            && let Some(info) = info.filter(|i| i.tagged(s.id))
        {
            self.site_price(w, s, info, fl, h);
        }
        let (r, st) = site_wait(&s.state, &s.upgrade, s.build_progress, info.is_some_and(|i| i.danger));
        if r != WaitReason::None {
            self.site_status(s, r, st, w.biome.depth, s.x, top + px(20.0));
        }
    }

    /// `scaffold`: Gerüst der Größenklasse, ohne Grafik Stangen und Laufbretter; liefert die Höhe für Balken darüber.
    pub fn scaffold(&mut self, kind: &str, x: f64, w: f64, h: f64) -> f64 {
        let l = scaffold_look(kind);
        if let Some((_, sh)) = self.size(l) {
            self.sprite(l, x, 0.0, 0);
            return sh;
        }
        let pole = [102, 57, 49, 255];
        for dx in [-w / 2.0 + px(2.0), 0.0, w / 2.0 - px(2.0)] {
            Planner::rect(&mut self.body, x + dx, 0.0, px(4.0), h, pole);
        }
        let mut y = px(8.0);
        while y < h {
            Planner::rect(&mut self.body, x, h - y - px(4.0), w, px(4.0), [143, 86, 59, 255]);
            y += 1.0;
        }
        h
    }

    /// `build_sign`: Brett am Stock ohne Text (B-584), Fuß bei x.
    pub fn build_sign(&mut self, x: f64, variant: i64) {
        let c = SIGN_BOARD[variant as usize];
        let (stick, bw, bh) = (px(17.0), px(34.0), px(22.0));
        Planner::rect(&mut self.body, x, 0.0, px(8.0), stick, [111, 69, 24, 255]);
        Planner::rect(&mut self.body, x, stick, bw, bh, c[0]);
        self.body.push(Item {
            shape: Shape::Frame,
            x,
            y: stick,
            w: bw,
            h: bh,
            size: px(2.0),
            clr: [61, 43, 31, 255],
            ..Default::default()
        });
        Planner::rect(&mut self.body, x, stick + bh - px(7.0), bw - px(6.0), px(4.0), c[1]);
    }

    /// `site_price`: Preis am unbezahlten Platz; stehen Bäume oder Felsen im Freiraum, „erst roden“ (B-636, B-652).
    pub fn site_price(&mut self, w: &World, s: &Site, info: &StageInfo, fl: SiteFlags, h: f64) {
        if fl.clearing {
            let key = TextKey {
                kind: s.kind.clone(),
                ..TextKey::new(TextWhat::Clearing)
            };
            self.with_text(key, |p, text| p.label_sized(s.x, h + px(20.0), text, GOLD, SMALL));
            return;
        }
        let c = build_cost(&s.kind, w.biome.depth);
        let mut total = cost_of(&c, "gold");
        if fl.payable {
            total = fl.gold_left + s.paid_gold;
        }
        let key = TextKey {
            kind: s.kind.clone(),
            n: w.biome.depth,
            ..TextKey::new(TextWhat::Price)
        };
        let enough = can_afford(info.stock, &c);
        self.with_text(key, |p, text| {
            p.price_cost(s.x, h + px(20.0), (total, s.paid_gold), text, &c, enough);
        });
    }
}
