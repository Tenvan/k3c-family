//! Port von `native/k3c-game/props/plan_combat.go`. Zeichenliste der Kampfregeln (KS5, B-934, docs/rules/kampf.md):
//! HP-Balken über jedem stehenden Portal, gefallene Kronen (fliegend im Bogen, liegend am Boden) und die Zahlstellen der
//! Kampfvariante (Schild, Auslöser, Banner) mit Preis. Kronen auf Köpfen (Monarch, Gegner, Mitspieler) zeichnet actors.
//! Krone und Zahlstellen sind Papierschnitt-Icons (B-940, Gruppe icons); nur der Portal-Balken bleibt gezeichnet. Die
//! Zahlstellen tragen keinen Text, nur Preis-Münzen.

use k3c_sim::state::World;
use k3c_sim::state::shared::{CROWN_FLYING, CROWN_GROUND};

use super::buildings::Look;
use super::listen::StageInfo;
use super::nature::{IDLE, object};
use super::pending::CombatSpot;
use super::plan::{Item, Planner, Shape, px, visible};
use super::simview::SimView;
use crate::view::cell::Camera;

/// `SPOTS_EVERY`: so selten (Sekunden Spielzeit) liest die Anzeige die Zahlstellen neu (`sim.CombatSpots` legt eine
/// Liste an). Go: `spotsEvery`.
pub const SPOTS_EVERY: f64 = 0.5;

/// Farben der Zahlstellen und Kronen.
pub const PORTAL_RED: [u8; 4] = [230, 57, 70, 255];

/// Breite des Portal-Balkens (Go: `portalBarW`).
fn portal_bar_w() -> f64 {
    px(80.0)
}

/// Abstand des Balkens über dem Portal-Bild (Go: `portalAbove`).
fn portal_above() -> f64 {
    px(14.0)
}

/// Bilder der Krone und der Zahlstellen (data/paper.json › objekte), einmal angelegt.
pub const CROWN_LOOK: Look = Look::new("icons/icon-krone", IDLE);
pub const SHIELD_LOOK: Look = Look::new("icons/zahlstelle-schild", IDLE);
/// Horn auf einem Pfahl vor dem Tor (KS7, B-941)
pub const TRIGGER_LOOK: Look = Look::new("objekte/horn-pfahl", IDLE);
pub const BANNER_LOOK: Look = Look::new("icons/zahlstelle-banner", IDLE);
/// Gestell neben der Burg, ab Hub-Stufe 2 (KS7, B-942)
pub const RACK_LOOK: Look = Look::new("objekte/schild-gestell", IDLE);

/// Schildstapel auf dem Gestell: Höhe des Ablagebretts (Go: `rackTop`).
fn rack_top() -> f64 {
    px(21.0)
}

/// Abstand je Schild im Stapel (Go: `rackLayer`).
fn rack_layer() -> f64 {
    px(5.0)
}

/// `COMBAT_ASSETS` sind die Icons der Kampfregeln, die props in jedem Level anmeldet (`level_assets`).
pub const COMBAT_ASSETS: [&str; 5] = [
    CROWN_LOOK.asset,
    SHIELD_LOOK.asset,
    TRIGGER_LOOK.asset,
    BANNER_LOOK.asset,
    RACK_LOOK.asset,
];

/// `portal_down`: Das Portal an x ist zerstört (`World::portals_down`), sein Bild verschwindet.
pub fn portal_down(w: &World, x: f64) -> bool {
    w.portals_down.iter().any(|d| (d - x).abs() < 0.01)
}

impl Planner {
    /// `combat` legt Portal-Balken, Kronen am Boden bzw. in der Luft und Zahlstellen an.
    pub fn combat(&mut self, w: &World, info: Option<&StageInfo>, k: &Camera, view: &dyn SimView) {
        self.portal_bars(w, k);
        for o in &w.players {
            if let Some(c) = &o.crown
                && (c.state == CROWN_FLYING || c.state == CROWN_GROUND)
                && visible(k, c.x, 2.0)
            {
                self.fallen_crown(c.x, c.z);
            }
        }
        let Some(info) = info else {
            return;
        };
        for s in &info.spots {
            if visible(k, s.x, 3.0) {
                self.spot(w, s, view);
            }
        }
    }

    /// `portal_bars`: HP jedes stehenden Portals (`w.portal_hp` läuft mit `w.portals`, Index für Index).
    pub fn portal_bars(&mut self, w: &World, k: &Camera) {
        if w.portal_max_hp <= 0.0 {
            return;
        }
        let h = self.size(object("portal")).map_or_else(|| px(220.0), |(_, h)| h);
        for (i, &x) in w.portals.iter().enumerate() {
            if i < w.portal_hp.len() && visible(k, x, 4.0) {
                self.bar(
                    x,
                    h + portal_above(),
                    portal_bar_w(),
                    w.portal_hp[i] / w.portal_max_hp,
                    PORTAL_RED,
                );
            }
        }
    }

    /// `fallen_crown`: Krone-Icon mit Fuß bei (x, z), z = Höhe in Units; liegt über den Figuren (top).
    pub fn fallen_crown(&mut self, x: f64, z: f64) {
        if self.size(CROWN_LOOK).is_some() {
            self.top.push(Item {
                shape: Shape::Sprite,
                look: CROWN_LOOK,
                x,
                y: z,
                ..Default::default()
            });
        }
    }

    /// `spot`: Icon der Zahlstelle (Schild an der Burg, Auslöser und Banner am Tor) und, steht ein Monarch mit Krone in
    /// Reichweite, der Preis als Münz-Plätze (gefüllt = bezahlt).
    pub fn spot(&mut self, w: &World, s: &CombatSpot, view: &dyn SimView) {
        match s.kind.as_str() {
            "shield" => self.shield_rack(w, view),
            "trigger" => {
                self.sprite(TRIGGER_LOOK, s.x, 0.0, 0);
            }
            _ => {
                self.sprite(BANNER_LOOK, s.x, 0.0, 0);
            }
        }
        // Preis und Zahlstand nur für den Monarchen dieser Split-Hälfte
        let Some(v) = self.viewer.and_then(|i| w.players.get(i)) else {
            return;
        };
        // erst die billige Entfernung, `spot_state` alloziert
        if v.crown.is_some() || !view.spot_near(w, s, v) {
            return;
        }
        let st = view.spot_state(w, s, v);
        if st.payable {
            self.price(s.x, px(86.0), st.price, st.paid, "", true);
        }
    }

    /// `shield_rack`: Gestell neben der Burg mit den gebauten, noch nicht abgeholten Schilden als Stapel
    /// (`sim.ShieldRack`, B-942); ohne Gestell-Bild bleibt das Schild-Icon als Ersatz.
    pub fn shield_rack(&mut self, w: &World, view: &dyn SimView) {
        let (x, stack, ok) = view.shield_rack(w);
        if !ok {
            return;
        }
        if !self.sprite(RACK_LOOK, x, 0.0, 0) {
            self.sprite(SHIELD_LOOK, x, 0.0, 0);
            return;
        }
        for i in 0..stack {
            self.sprite(SHIELD_LOOK, x, rack_top() + i as f64 * rack_layer(), 0);
        }
    }
}
