//! Naht zu den Sim-Funktionen, die erst spätere Sprints nach Rust bringen (Vorentscheidung RU16.1, Abweichung von
//! Leitfaden § 7): Die Go-Funktionen `sim.EconomyOf`, `SiteGoldLeft`, `SitePayable`, `SiteNeedsClearing`, `ShieldRack`,
//! `CombatSpots` und `AtLair` rechnen aus Sim-Regeln, die RU15 (combat, boss_endboss) und RU18 (economy_view) liefern.
//! Alle Zeichner und `hud` reichen `&dyn SimView` durch; RU15 und RU18 ersetzen `PendingSimView` durch die Sim und
//! löschen diese Datei samt `pending::{Economy, HubUpgrade, CombatSpot}`.

use k3c_sim::state::{Player, Site, World};

use super::pending::{CombatSpot, Economy, PendingSimView};

/// `SimView` ist die Sicht der Anzeige auf abgeleitete Sim-Werte (Signaturen wie in Go).
pub trait SimView {
    /// Go: `sim.EconomyOf`.
    fn economy_of(&self, w: &World) -> Economy;
    /// Go: `sim.SiteGoldLeft` (Rest-Gold des Platzes, bezahlbar).
    fn site_gold_left(&self, w: &World, s: &Site) -> (i64, bool);
    /// Go: `sim.SitePayable`.
    fn site_payable(&self, w: &World, s: &Site) -> bool;
    /// Go: `sim.SiteNeedsClearing`.
    fn site_needs_clearing(&self, w: &World, s: &Site) -> bool;
    /// Go: `sim.ShieldRack` (x, stack, visible).
    fn shield_rack(&self, w: &World) -> (f64, i64, bool);
    /// Go: `sim.CombatSpots`.
    fn combat_spots(&self, w: &World) -> Vec<CombatSpot>;
    /// Go: `sim.AtLair`.
    fn at_lair(&self, w: &World, p: &Player) -> bool;
}

impl SimView for PendingSimView {
    fn economy_of(&self, _w: &World) -> Economy {
        unimplemented!("RU18: economy_of")
    }
    fn site_gold_left(&self, _w: &World, _s: &Site) -> (i64, bool) {
        unimplemented!("RU18: site_gold_left")
    }
    fn site_payable(&self, _w: &World, _s: &Site) -> bool {
        unimplemented!("RU18: site_payable")
    }
    fn site_needs_clearing(&self, _w: &World, _s: &Site) -> bool {
        unimplemented!("RU18: site_needs_clearing")
    }
    fn shield_rack(&self, _w: &World) -> (f64, i64, bool) {
        unimplemented!("RU15: shield_rack")
    }
    fn combat_spots(&self, _w: &World) -> Vec<CombatSpot> {
        unimplemented!("RU15: combat_spots")
    }
    fn at_lair(&self, _w: &World, _p: &Player) -> bool {
        unimplemented!("RU15: at_lair")
    }
}
