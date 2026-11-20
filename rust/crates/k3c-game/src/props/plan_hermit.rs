//! Zeichenliste des Einsiedlers (RU62.11, B-1001): Hütte mit Figur und Preis (Stufe der Tiefe 1), die Figur beim Bau im Hub
//! und das Welt-Bauwerk (Dojo, Schrein) mit Gerüst, dann Gebäude am Hub-Platz. Das Spiel zeichnet nur: Phase, Preis und
//! Fortschritt kommen aus `World::hermit` (Sim, RU62.10), der Platz aus `World::sites`. Jede Split-Screen-Hälfte zeigt, was
//! in ihrer Stufe steht: die Hütte nur in Tiefe 1, das Bauwerk nur in Tiefe 0. Die Zuordnung der Grafiken steht in
//! `data/paper.json › einsiedler` (ein Test hält Code und Datei gleich).

use k3c_sim::state::World;
use k3c_sim::state::hermit_view::HermitPhase;

use super::buildings::Look;
use super::listen::{SiteFlags, StageInfo};
use super::plan::{GOLD, Planner, STONE_GRAY, px, visible};
use super::plan_sites::placeholder;
use crate::view::cell::Camera;
use k3c_sim::state::Site;

/// Platzart des Bauwerks, wenn die Welt den Katalog nicht mitführt (`data/hermits.json › hubSite.id`).
pub const HALL_KIND: &str = "hermitHall";
/// Grafik der Hütte (eine für alle Welten).
pub const HUT_ASSET: &str = "gebaeude-paper/einsiedler-huette";
/// Alle Assets des Einsiedlers, in die Ladeliste jedes Levels aufgenommen.
pub const HERMIT_ASSETS: [&str; 5] = [
    HUT_ASSET,
    "gebaeude-paper/dojo",
    "gebaeude-paper/schrein",
    "figuren/schwertschmied",
    "figuren/priesterin",
];
const HUT_CLR: [u8; 4] = [120, 84, 52, 255];
const FIGURE_CLR: [u8; 4] = [74, 111, 165, 255];

/// `building_asset`: Grafik des Bauwerks nach Name (`dojo`, `shrine`).
pub fn building_asset(name: &str) -> Option<&'static str> {
    match name {
        "dojo" => Some("gebaeude-paper/dojo"),
        "shrine" => Some("gebaeude-paper/schrein"),
        _ => None,
    }
}

/// `figure_asset`: Grafik der Figur nach Katalog-ID (`swordsmith`, `priestess`).
pub fn figure_asset(id: &str) -> Option<&'static str> {
    match id {
        "swordsmith" => Some("figuren/schwertschmied"),
        "priestess" => Some("figuren/priesterin"),
        _ => None,
    }
}

impl Planner {
    /// `hermit_hut`: Hütte mit Figur und Preis, solange der Einsiedler wartet; danach die leere Hütte.
    pub fn hermit_hut(&mut self, w: &World, k: &Camera) {
        let Some(h) = w.hermit.as_ref() else {
            return;
        };
        let Some(hut) = h.hut.as_ref() else {
            return;
        };
        if !visible(k, hut.x, 3.0) {
            return;
        }
        let look = Look::new(HUT_ASSET, "stand");
        let top = if self.sprite(look, hut.x, 0.0, 0) {
            self.size(look).map_or(px(60.0), |(_, h)| h)
        } else {
            Planner::rect(&mut self.body, hut.x, 0.0, px(90.0), px(60.0), HUT_CLR);
            px(60.0)
        };
        if h.phase != HermitPhase::Hut {
            return;
        }
        self.hermit_figure(&h.hermit, hut.x + px(70.0));
        self.label(
            hut.x,
            top + px(8.0),
            &h.price.to_string(),
            if h.open { GOLD } else { STONE_GRAY },
        );
    }

    /// `hermit_figure`: die Figur des Einsiedlers bei `x`; ohne Sheet ein Platzhalter.
    pub fn hermit_figure(&mut self, id: &str, x: f64) {
        if let Some(a) = figure_asset(id)
            && self.sprite(Look::new(a, "idle"), x, 0.0, 0)
        {
            return;
        }
        Planner::rect(&mut self.body, x, 0.0, px(18.0), px(56.0), FIGURE_CLR);
    }

    /// `is_hall`: der Platz ist der Bauplatz des Bauwerks.
    pub fn is_hall(w: &World, s: &Site) -> bool {
        w.hermit.is_some() && s.kind == w.hermit_cfg.as_ref().map_or(HALL_KIND, |c| c.hub_site.id.as_str())
    }

    /// `hall_site`: das Bauwerk am Hub-Platz: Gerüst mit Fortschritt und der Figur, fertig das Gebäude, zerstört Trümmer;
    /// vor der Zahlung nichts (die Hütte nimmt die Münzen).
    pub fn hall_site(&mut self, w: &World, s: &Site, info: Option<&StageInfo>, fl: SiteFlags) {
        let Some(h) = w.hermit.as_ref() else {
            return;
        };
        if s.state == "built" {
            let look = building_asset(&h.building).map(|a| Look::new(a, "stand"));
            self.built_site_look(w, s, info, fl, look);
            return;
        }
        let (bw, _) = placeholder(&s.kind);
        if h.phase == HermitPhase::Walking {
            let top = self.scaffold(&s.kind, s.x, bw, placeholder(&s.kind).1);
            self.bar(s.x, top + px(4.0), px(80.0), h.progress, [221, 161, 94, 255]);
            self.hermit_figure(&h.hermit, s.x + bw / 2.0 + px(30.0));
        } else if h.phase == HermitPhase::Fallen {
            self.rubble(s.x, bw);
        }
    }
}
