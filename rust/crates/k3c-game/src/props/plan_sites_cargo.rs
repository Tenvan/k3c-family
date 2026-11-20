//! Zeichenliste der Schiffs-Fracht (RU62.2, B-997): die drei Ladeplätze am Steg (Schild am Pfahl mit Preis) und die
//! Fracht an Deck, je belegtem Platz ein Beutel, eine Kiste oder eine Truppe. Das Spiel zeichnet nur: Plätze, Fracht und
//! Ladeplätze kommen aus `World::cargo` (Sim, RU62.1). Beide Split-Screen-Hälften zeichnen denselben Stand.

use k3c_sim::state::World;
use k3c_sim::state::cargo_view::CargoEntry;

use super::nature::{anim_phase, object};
use super::plan::{GOLD, Item, Planner, Shape, WHITE, px, visible};
use crate::view::cell::Camera;

/// Breite des Ladedecks in Units (Rumpf des Schiffs, Frame 200 Spiel-Pixel breit).
const DECK_WIDTH: f64 = 4.4;
/// Höhe des Decks über dem Fuß des Schiffs in Spiel-Pixeln.
const DECK_Y_PX: f64 = 66.0;
/// Farbe einer Truppe an Deck (Platzhalter, es gibt kein eigenes Asset).
const TROOP_CLR: [u8; 4] = [74, 111, 165, 255];
/// Preis am Ladeplatz bei vollem Schiff (rot), sonst Gold.
const FULL_CLR: [u8; 4] = [230, 57, 70, 255];

/// `bay_asset`: Objekt-Asset des Ladeplatzes nach Art (`coins`, `troops`, `crates`).
pub fn bay_asset(kind: &str) -> Option<&'static str> {
    match kind {
        "coins" => Some("ladeplatz-muenzen"),
        "troops" => Some("ladeplatz-truppen"),
        "crates" => Some("ladeplatz-material"),
        _ => None,
    }
}

/// `load_asset`: Objekt-Asset einer Ladung an Deck; die Truppe hat keins (Platzhalter).
pub fn load_asset(kind: &str) -> Option<&'static str> {
    match kind {
        "coins" => Some("fracht-beutel"),
        "crate" => Some("fracht-kiste"),
        _ => None,
    }
}

/// `deck_x`: Mitte von Platz `i` von `slots` auf dem Deck des Schiffs bei `ship_x`.
pub fn deck_x(ship_x: f64, slots: i64, i: i64) -> f64 {
    let n = slots.max(1) as f64;
    ship_x - DECK_WIDTH / 2.0 + (i as f64 + 0.5) * DECK_WIDTH / n
}

impl Planner {
    /// `cargo`: Ladeplätze am Steg und Fracht an Deck. Ohne Fracht-Zustand (Welt kennt keine Fracht) nichts.
    pub fn cargo(&mut self, w: &World, k: &Camera) {
        let Some(c) = w.cargo.as_ref() else {
            return;
        };
        for b in &c.bays {
            if !visible(k, b.x, 2.0) {
                continue;
            }
            let Some(id) = bay_asset(&b.kind) else { continue };
            let look = object(id);
            let h = match self.size(look) {
                Some((_, h)) => {
                    self.sprite(look, b.x, 0.0, anim_phase(b.x, 4));
                    h
                }
                None => {
                    Planner::rect(&mut self.body, b.x, 0.0, px(34.0), px(46.0), TROOP_CLR);
                    px(46.0)
                }
            };
            let clr = if b.state == "full" { FULL_CLR } else { GOLD };
            self.label(b.x, h + px(8.0), &b.price.to_string(), clr);
        }
        let Some(ship) = w.sites.iter().find(|s| s.kind == "ship") else {
            return;
        };
        if !visible(k, ship.x, DECK_WIDTH) {
            return;
        }
        for e in &c.load {
            self.deck_item(ship.x, c.slots, e);
        }
    }

    /// `deck_item`: eine Ladung auf ihrem Platz (Index in Lade-Reihenfolge).
    fn deck_item(&mut self, ship_x: f64, slots: i64, e: &CargoEntry) {
        let x = deck_x(ship_x, slots, e.index);
        let y = px(DECK_Y_PX);
        if let Some(id) = load_asset(&e.kind)
            && self.sprite(object(id), x, y, 0)
        {
            return;
        }
        // Truppe oder fehlendes Asset: kleines Rechteck, bei Truppen mit Rahmen
        Planner::rect(&mut self.body, x, y, px(16.0), px(24.0), TROOP_CLR);
        self.body.push(Item {
            shape: Shape::Frame,
            x,
            y,
            w: px(16.0),
            h: px(24.0),
            size: px(2.0),
            clr: WHITE,
            ..Default::default()
        });
    }
}
