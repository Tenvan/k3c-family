//! Zustand der Fracht für die Anzeige (RU62.1): `World::cargo` in allen Stufen der Insel, dazu die nutzbaren Ladeplätze der
//! Hafen-Stufe (`World::cargo_bays`), damit Spieler dort in die Fracht zahlen statt Münzen auf den Boden zu werfen.

use k3c_data::cargo::CargoFile;

use super::load::check;
use super::step::Ship;
use super::{Bay, Cargo};
use crate::island::Island;
use crate::state::cargo_view::{CargoBay, CargoBayState, CargoEntry, CargoOrderState, CargoState};

/// Baut den Zustand der Fracht. Ladeplätze nennt er nur, solange sie nutzbar sind.
pub fn build_state(isl: &Island, ship: &Ship, cfg: &CargoFile, slots: i64, active: bool) -> CargoState {
    let c: &Cargo = &isl.cargo;
    let mut st = CargoState {
        slots,
        used: c.load.len() as i64,
        reserved: c.orders.len() as i64,
        hold: ship.hold,
        ..CargoState::default()
    };
    for (i, it) in c.load.iter().enumerate() {
        st.load.push(CargoEntry {
            kind: it.kind.clone(),
            id: it.id.clone(),
            amount: it.amount,
            index: i as i64,
        });
    }
    for o in &c.orders {
        st.orders.push(CargoOrderState {
            resource: o.resource.clone(),
            units: o.units,
            carrier: o.carrier,
            eta: o.eta,
        });
    }
    if active {
        for bay in Bay::ALL {
            let x = ship.x + bay.offset(cfg);
            let state = match check(isl, ship.wi, slots, bay, x) {
                Ok(()) => "ready",
                Err(super::load::Refusal::Full) => "full",
                Err(_) => "empty",
            };
            st.bays.push(CargoBayState {
                kind: bay.name().to_string(),
                x,
                price: bay.price(cfg),
                state: state.to_string(),
            });
        }
    }
    st
}

/// Veröffentlicht den Zustand und die nutzbaren Ladeplätze in den Stufen der Insel.
pub fn publish(isl: &mut Island, ship: &Ship, cfg: &CargoFile, slots: i64, active: bool) {
    let st = (active || !isl.cargo.load.is_empty() || !isl.cargo.orders.is_empty())
        .then(|| build_state(isl, ship, cfg, slots, active));
    for (i, w) in isl.stages.iter_mut().enumerate() {
        w.cargo.clone_from(&st);
        w.cargo_bays.clear();
        if active && i == ship.wi {
            for bay in Bay::ALL {
                w.cargo_bays.push(CargoBay {
                    x: ship.x + bay.offset(cfg),
                    range: cfg.bay_range_units,
                });
            }
        }
    }
}
