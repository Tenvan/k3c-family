//! Zeichenliste des Reittier-Gatters (RU62.9, B-1000): das Gatter mit dem Welt-Reittier und dem Preis, solange mindestens
//! ein gesteuerter Monarch das Tier noch nicht hat, sonst leer. Das Spiel zeichnet nur: Ort, Preis und Tier kommen aus
//! `World::mount_gate` (Sim, RU62.8), der Besitz aus `Player::mount`. Das Tier steht als Platzhalter (Körper, Kopf), weil
//! das Reittier ein Rig ohne Standbild-Sheet ist; der Platzhalter ist Gold für den Preis.

use k3c_sim::state::World;

use super::nature::object;
use super::plan::{GOLD, Planner, px, visible};
use crate::view::cell::Camera;

/// Farbe des Platzhalter-Tiers (Körper) und Gatter-Fallback.
const BEAST_CLR: [u8; 4] = [139, 94, 60, 255];
const GATE_CLR: [u8; 4] = [120, 84, 52, 255];

/// `gate_open`: Hat noch ein gesteuerter Monarch das Welt-Reittier nicht? Ohne Monarchen (Menü, Vorschau) gilt es als offen.
pub fn gate_open(w: &World, mount: &str) -> bool {
    w.players.is_empty() || w.players.iter().any(|p| p.mount != mount)
}

impl Planner {
    /// `mount_gate`: Gatter, Tier und Preis. Ohne Gatter im Zustand (Welt ohne Welt-Reittier, Unterwelt) nichts.
    pub fn mount_gate(&mut self, w: &World, k: &Camera) {
        let Some(g) = w.mount_gate.as_ref() else {
            return;
        };
        if !visible(k, g.x, 3.0) {
            return;
        }
        let look = object("gatter");
        let h = match self.size(look) {
            Some((_, h)) => {
                self.sprite(look, g.x, 0.0, 0);
                h
            }
            None => {
                Planner::rect(&mut self.body, g.x, 0.0, px(168.0), px(40.0), GATE_CLR);
                px(40.0)
            }
        };
        if !gate_open(w, &g.mount) {
            return;
        }
        Planner::rect(&mut self.body, g.x, px(8.0), px(70.0), px(34.0), BEAST_CLR);
        Planner::rect(&mut self.body, g.x + px(40.0), px(30.0), px(22.0), px(26.0), BEAST_CLR);
        self.label(g.x, h + px(8.0), &g.price.to_string(), GOLD);
    }
}
