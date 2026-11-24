//! Port von `native/k3c-game/hud/seekarte_input.go`: Eingabe der Seekarte als Teil des Overlays (`overlay.rs`).
//! Links/Rechts bewegen im Spiel den Monarchen, darum zählen die Flanken nur, solange der Monarch am Steg steht
//! (`harbor_reach`); der Monarch läuft beim Blättern mit, die Karte bleibt in jeder Zelle sichtbar. A ist im Spiel auch
//! Bezahlen: Ein Bestätigen am Steg verbraucht die Eingabe des Spielers (`Overlay::consumed`), damit nichts mitbezahlt
//! wird, und zwar bis er A loslässt: Bezahlen ist ein gehaltener Befehl (B-1006). B bleibt unbelegt.

use std::sync::OnceLock;

use super::Hud;
use super::texts::tr;
use crate::view::frame::{Cue, CueKind, Frame, Seat};
use crate::view::input::Input;
use crate::view::session::Session;
use k3c_data::Source;
use k3c_sim::state::{Player, World};

/// SHIP_KIND ist die Art des Schiffs am Steg (sim ship.go); der Steg liegt am Schiff.
pub const SHIP_KIND: &str = "ship";

/// DEFAULT_REACH gilt, wenn data/hub.json nicht lesbar ist (Wert von `hub.travel.rangeUnits`).
pub const DEFAULT_REACH: f64 = 3.0;

/// harbor_reach ist `hub.travel.rangeUnits` aus data/hub.json: derselbe Radius, in dem die Simulation Spieler am Steg
/// zählt (island_switch.go). Wer dort steht, bedient die Karte; wer außerhalb steht, hindert den Wechsel ohnehin.
pub fn harbor_reach() -> f64 {
    static REACH: OnceLock<f64> = OnceLock::new();
    *REACH.get_or_init(read_reach)
}

fn read_reach() -> f64 {
    let raw = match Source::from_env().read("hub.json") {
        Ok(raw) => raw,
        Err(e) => {
            k3c_platform::log::warn(
                "💥 hub.json nicht gelesen, Steg-Radius der Seekarte Rückfall",
                &[("err", &e)],
            );
            return DEFAULT_REACH;
        }
    };
    match k3c_data::hub::load(&raw) {
        Ok(hub) if hub.travel.range_units > 0.0 => hub.travel.range_units,
        Ok(_) => {
            k3c_platform::log::warn(
                "💥 hub.travel.rangeUnits unbrauchbar, Steg-Radius der Seekarte Rückfall",
                &[],
            );
            DEFAULT_REACH
        }
        Err(e) => {
            k3c_platform::log::warn(
                "💥 hub.travel.rangeUnits unbrauchbar, Steg-Radius der Seekarte Rückfall",
                &[("err", &e)],
            );
            DEFAULT_REACH
        }
    }
}

/// at_harbor meldet, ob der Monarch `m` in der Stufe `w` am Schiff steht.
pub fn at_harbor(w: &World, m: &Player) -> bool {
    let r = harbor_reach();
    w.sites.iter().any(|s| s.kind == SHIP_KIND && (m.x - s.x).abs() <= r)
}

impl Hud {
    /// update_sea bedient die Seekarte des Spielers `seat`; Monarch ohne Krone, getrenntes Gerät oder fern vom Steg:
    /// keine Eingabe. `busy`: das Skill-Menü hat die Eingabe schon verbraucht, dann wird nur die Sperre geführt.
    /// Rückgabe: Eingabe verbraucht (Bestätigen am Steg, auch bei Ablehnung, und danach, solange der Spieler A hält:
    /// `SeaView::hold`, B-1006).
    pub fn update_sea(&mut self, input: &Input, f: &Frame, s: &mut dyn Session, seat: &Seat, busy: bool) -> bool {
        let Some(p) = usize::try_from(seat.player).ok().filter(|p| *p < self.seats.len()) else {
            return false;
        };
        let (di, ok) = input.of(seat.device);
        let world = f.stage(seat.stage).and_then(|g| g.world.as_deref());
        let sea = &mut self.seats[p].sea;
        let at = match (f.monarch(seat.player), world) {
            (Some(m), Some(w)) => m.crown.is_none() && at_harbor(w, m),
            _ => false,
        };
        if !sea.open || !ok || seat.lost || !at {
            sea.hold = false; // Steg verlassen, Krone verloren, Gerät getrennt, Karte zu: Sperre endet
            return false;
        }
        sea.hold = sea.hold && di.command.pay; // losgelassen: Sperre endet, A zahlt wieder
        if busy {
            return sea.hold;
        }
        let r = sea.step(&di.actions);
        let mut cue = |kind: CueKind, text: &str| {
            self.cue_buf.push(Cue {
                kind,
                player: seat.player,
                text: text.to_string(),
                ..Cue::default()
            });
        };
        if r.moved {
            cue(CueKind::Move, "");
        }
        if r.denied {
            sea.hold = true; // wie eine abgelehnte Wahl: A zahlt nicht mit, bis es losgelassen ist
            cue(CueKind::Denied, tr(&self.lang, "sea.denied"));
            return true;
        }
        if r.pick.is_empty() {
            return sea.hold;
        }
        sea.hold = true; // gilt, bis ein Update A nicht mehr gehalten sieht
        if let Err(e) = s.choose_island(seat.player, &r.pick) {
            k3c_platform::log::info("🚫 Seekarte abgelehnt", &[("player", &seat.player), ("err", &e)]);
            // Grund steht im Log, der Cue in der Sprache des Spiels
            cue(CueKind::Denied, tr(&self.lang, "sea.denied"));
            return true;
        }
        cue(CueKind::Confirm, "");
        true
    }
}
