//! Port von `native/k3c-game/props/plan.go` (Teil 2: Burg): `castle`, `castle_shape`, `castle_sign`, `bits_arr`.

use k3c_sim::state::World;

use super::buildings::{HUB_SCAFFOLD, Look, hub_look_of, stage_anim};
use super::listen::StageInfo;
use super::plan::{DARK, HP_GREEN, Item, Planner, STONE_GRAY, Shape, px, visible};
use super::sites::{
    TextKey, TextWhat, WaitReason, covers_arr, hub_material, hub_title, missing_arr, offers_hub_upgrade, res_list,
    wait_of, wait_text,
};
use crate::view::cell::Camera;

impl Planner {
    /// `castle`: Look der Wachstumsreihe, Gerüst davor, Schmuck der Ausbauschritte; Balken und Ausbau-Schild darüber.
    pub fn castle(&mut self, w: &World, info: Option<&StageInfo>, k: &Camera) {
        let x = w.castle.x;
        if !visible(k, x, 6.0) {
            return;
        }
        let (lv, step) = (w.hub_level.max(1), w.hub_step.max(1));
        let state = info.map_or("", |i| i.hub.state.as_str());
        let hl = hub_look_of(lv, step, state);
        let mut main = hl.main;
        if self.size(main).is_none() {
            main = Look::new("gebaeude-paper/burg", stage_anim(lv)); // fehlt der Look, die Burg der Hub-Stufe
        }
        let h = if let Some((_, h)) = self.size(main) {
            self.sprite(main, x, 0.0, 0);
            Some(h)
        } else {
            self.castle_shape(x);
            None
        };
        if hl.scaffold {
            self.sprite(HUB_SCAFFOLD, x, 0.0, 0);
        }
        if h.is_some() && hl.overlay.ok() {
            self.sprite(hl.overlay, x, 0.0, 0);
        }
        let c = &w.castle;
        if c.max_hp > 0.0 && c.hp < c.max_hp {
            self.bar(x, px(310.0 - 4.0), px(240.0), c.hp / c.max_hp, HP_GREEN);
        }
        if let Some(info) = info {
            self.castle_sign(w, info, x, h.unwrap_or_else(|| px(290.0)) + px(20.0));
        }
    }

    /// `castle_shape`: Platzhalter-Burg ohne Grafik (siteView.ts › drawCastle).
    pub fn castle_shape(&mut self, x: f64) {
        Planner::rect(&mut self.body, x, 0.0, px(300.0), px(260.0), STONE_GRAY);
        self.body.push(Item {
            shape: Shape::Frame,
            x,
            w: px(300.0),
            h: px(260.0),
            size: px(4.0),
            clr: DARK,
            ..Default::default()
        });
        Planner::rect(&mut self.body, x, 0.0, px(60.0), px(80.0), DARK);
        for dx in [-120.0, -40.0, 40.0, 120.0] {
            Planner::rect(&mut self.body, x + px(dx), px(260.0), px(40.0), px(30.0), STONE_GRAY);
        }
    }

    /// `castle_sign`: läuft der Ausbau, der Wartegrund; sonst am nächsten Preisschild das Angebot (B-591).
    pub fn castle_sign(&mut self, w: &World, info: &StageInfo, x: f64, y: f64) {
        let hub = &info.hub;
        if !hub.has {
            return;
        }
        let r = wait_of(&hub.state, info.danger);
        if r != WaitReason::None {
            let mut need = hub.material;
            let m = missing_arr(hub.material, info.stock);
            if hub.state == "waitingNextIsland" && bits_arr(m) != 0 {
                need = m; // Material-Tor: nur nennen, was fehlt (sonst alle)
            }
            let mut key = TextKey::new(TextWhat::HubStatus);
            key.state.clone_from(&hub.state);
            key.n = info.hub_step * 100 + hub.steps;
            key.m = i64::from(r as u8) * 100 + bits_arr(need);
            let mut texts = std::mem::take(&mut self.texts);
            let text = texts.get_or_insert_with(key, || {
                format!(
                    "{}: {}",
                    hub_title(info.hub_step, hub.steps),
                    wait_text(r, &hub.state, &res_list(&hub_material(need)))
                )
            });
            self.status(x, y, r, text);
            self.texts = texts;
            return;
        }
        if !info.tagged(w.castle.id) || !offers_hub_upgrade(hub.has, &hub.state, hub.gold, hub.paid) {
            return;
        }
        let mut key = TextKey::new(TextWhat::HubPrice);
        key.n = info.hub_step * 100 + hub.steps;
        key.m = info.hub_level;
        let mut texts = std::mem::take(&mut self.texts);
        let text = texts.get_or_insert_with(key, || hub_title(info.hub_step, hub.steps));
        let cost = hub_material(hub.material);
        self.price_cost(
            x,
            y,
            (hub.gold, hub.paid),
            text,
            &cost,
            covers_arr(info.stock, hub.material),
        );
        self.texts = texts;
    }
}

/// `bits_arr` kennzeichnet, welche Materialien eine Liste nennt (Schlüssel der Texte).
pub fn bits_arr(c: [i64; 5]) -> i64 {
    let mut n = 0;
    for (i, v) in c.iter().enumerate() {
        if *v > 0 {
            n |= 1 << i;
        }
    }
    n
}
