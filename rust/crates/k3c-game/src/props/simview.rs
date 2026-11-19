//! Naht zu den Sim-Funktionen, die erst spätere Sprints nach Rust bringen (Vorentscheidung RU16.1, Abweichung von
//! Leitfaden § 7): Die Go-Funktionen `sim.EconomyOf`, `SiteGoldLeft`, `SitePayable`, `SiteNeedsClearing`, `ShieldRack`,
//! `CombatSpots`, `NearOf`, `StateFor` und `AtLair` rechnen aus Sim-Regeln, die RU15 (combat, boss_endboss) und RU18 (economy_view) liefern.
//! Alle Zeichner und `hud` reichen `&dyn SimView` durch; RU15 und RU18 ersetzen `LiveSimView` durch die Sim und
//! löschen diese Datei samt `pending::{Economy, HubUpgrade, CombatSpot}`.
#![allow(clippy::unimplemented)] // Platzhalter: wartet auf den genannten Sprint (RU10, RU15, RU18, RU23)

use k3c_sim::state::{Player, Site, World};

use super::pending::{CombatSpot, Economy, HubUpgrade, LiveSimView, SpotState};
use k3c_sim::combat::spots::{CombatSpot as SimSpot, SpotKind};
use k3c_sim::state::site_view;
use k3c_sim::world::vals;

/// `SimView` ist die Sicht der Anzeige auf abgeleitete Sim-Werte (Signaturen wie in Go).
pub trait SimView {
    /// Go: `sim.EconomyOf`.
    fn economy_of(&self, w: &World) -> Economy;
    /// Go: `sim.SiteGoldLeft` (Rest-Gold des Platzes, bezahlbar).
    fn site_gold_left(&self, w: &World, s: &Site) -> (i64, bool);
    /// Go: `sim.SiteVisible` (steht der Platz im Zustand: Platz-Stufe höchstens Hub-Stufe + 1). Die Grundregel rechnet
    /// `k3c_sim::state::site_view` (hängt an `level_site`, RU18); eine Sicht mit Antworten (Standbild) überschreibt sie.
    fn site_visible(&self, w: &World, s: &Site) -> bool {
        site_view::site_visible(w, s)
    }
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
    /// Go: `CombatSpot.NearOf` (in Zahl-Reichweite plus einer Unit, ohne Allokation).
    fn spot_near(&self, w: &World, s: &CombatSpot, p: &Player) -> bool;
    /// Go: `CombatSpot.StateFor` (Zahlstand des Ziels für den Monarchen p).
    fn spot_state(&self, w: &World, s: &CombatSpot, p: &Player) -> SpotState;
}

impl SimView for LiveSimView {
    fn economy_of(&self, w: &World) -> Economy {
        let e = k3c_sim::economy::economy_of(w, &vals(w), w.island_ref_view().as_ref());
        Economy {
            stock_max: e.stock_max,
            purse_max: e.purse_max,
            hub_level: e.hub_level,
            hub_step: e.hub_step,
            hub_upgrade: e.hub_upgrade.map(|u| HubUpgrade {
                gold: u.gold,
                material: u.material,
                paid: u.paid,
                state: u.state,
                steps: u.steps,
            }),
            danger: e.danger,
            fighters: e.fighters,
            troop_limit: e.troop_limit,
        }
    }
    fn site_gold_left(&self, w: &World, s: &Site) -> (i64, bool) {
        k3c_sim::economy::site_gold_left(w, &vals(w), w.island_ref_view().as_ref(), s)
    }
    fn site_payable(&self, w: &World, s: &Site) -> bool {
        k3c_sim::economy::site_payable(w, &vals(w), w.island_ref_view().as_ref(), s)
    }
    fn site_needs_clearing(&self, w: &World, s: &Site) -> bool {
        k3c_sim::world::lines::site_needs_clearing(w, s)
    }
    fn shield_rack(&self, w: &World) -> (f64, i64, bool) {
        k3c_sim::combat::spots::shield_rack(w, &vals(w))
    }
    fn combat_spots(&self, w: &World) -> Vec<CombatSpot> {
        k3c_sim::combat::spots::combat_spots(w, &vals(w))
            .into_iter()
            .map(|s| CombatSpot {
                kind: s.kind.as_str().to_string(),
                side: s.side,
                x: s.x,
            })
            .collect()
    }
    fn at_lair(&self, w: &World, p: &Player) -> bool {
        k3c_sim::boss::endboss::at_lair(w, w.island_ref_view().as_ref(), p)
    }
    fn spot_near(&self, w: &World, s: &CombatSpot, p: &Player) -> bool {
        sim_spot(s).near_of(&vals(w), p)
    }
    fn spot_state(&self, w: &World, s: &CombatSpot, p: &Player) -> SpotState {
        let st = sim_spot(s).state_for(w, &vals(w), p);
        SpotState {
            price: st.price,
            paid: st.paid,
            payable: st.payable,
            near: st.near,
        }
    }
}

/// Das Zahlziel der Anzeige als Sim-Typ (`kind` ist `shield`, `trigger` oder `banner`).
fn sim_spot(s: &CombatSpot) -> SimSpot {
    let kind = match s.kind.as_str() {
        "trigger" => SpotKind::Trigger,
        "banner" => SpotKind::Banner,
        _ => SpotKind::Shield,
    };
    SimSpot {
        kind,
        side: s.side,
        x: s.x,
    }
}
