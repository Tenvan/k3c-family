//! Prüfen und Laden je Ladeplatz (RU62.1): Münzen-Platz belegt einen Platz mit einem Beutel, Truppen-Platz ruft die
//! nächststehende Truppe der Stufe an Bord, Material-Platz beauftragt einen Bauern mit einer Kiste. Abgelehnt wird ohne
//! Abzug: Laderaum voll, Vorrat unter einer Kiste, keine Truppe auf der Stufe.

use k3c_data::cargo::CargoFile;

use super::{Bay, CargoItem, CrateOrder, KIND_COINS, KIND_CRATE, KIND_TROOP};
use crate::island::Island;
use crate::island::storage::stock_field;
use crate::unit::warrior::is_fighter;
use crate::world::vals;

/// Warum ein Einwurf abgelehnt wird.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Refusal {
    /// Alle Plätze belegt oder reserviert (Ereignis `cargoFull`).
    Full,
    /// Keine Truppe der Stufe zum Verladen.
    NoTroop,
    /// Der größte Vorrat liegt unter einer Kiste.
    NoStock,
}

impl Refusal {
    /// Name im Ereignis `cargoRejected`.
    pub fn name(self) -> &'static str {
        match self {
            Refusal::Full => "full",
            Refusal::NoTroop => "noTroop",
            Refusal::NoStock => "noStock",
        }
    }
}

/// Das Material mit dem größten Vorrat der Insel (bei Gleichstand das erste in der Reihenfolge Holz, Stein, Kupfer, Eisen,
/// Kristall) und seine Menge.
pub fn largest_stock(isl: &Island) -> (&'static str, i64) {
    let s = &isl.stock;
    let mut best = ("wood", s.wood);
    for (name, n) in [
        ("stone", s.stone),
        ("copper", s.copper),
        ("iron", s.iron),
        ("crystal", s.crystal),
    ] {
        if n > best.1 {
            best = (name, n);
        }
    }
    best
}

/// Die Truppe der Stufe `wi`, die dem Ladeplatz bei `x` am nächsten steht (Kämpfer, keine Bürger); bei Gleichstand die
/// vorn in der Liste. Liefert den Platz in `stages[wi].troops`.
pub fn nearest_fighter(isl: &Island, wi: usize, x: f64) -> Option<usize> {
    let w = &isl.stages[wi];
    let v = vals(w);
    let mut best: Option<(usize, f64)> = None;
    for (i, t) in w.troops.iter().enumerate() {
        if !is_fighter(&v.troops, &t.kind) {
            continue;
        }
        let d = (t.x - x).abs();
        if best.is_none_or(|(_, bd)| d < bd) {
            best = Some((i, d));
        }
    }
    best.map(|(i, _)| i)
}

/// Prüft, ob am Ladeplatz `bay` (bei `x`) geladen werden kann; ohne Abzug.
pub fn check(isl: &Island, wi: usize, slots: i64, bay: Bay, x: f64) -> Result<(), Refusal> {
    if isl.cargo.taken() as i64 >= slots {
        return Err(Refusal::Full);
    }
    match bay {
        Bay::Coins => Ok(()),
        Bay::Troops => nearest_fighter(isl, wi, x).map(|_| ()).ok_or(Refusal::NoTroop),
        Bay::Crates => {
            let units = isl.cargo.cfg.as_ref().map_or(0, |c| c.crate_units);
            if largest_stock(isl).1 >= units {
                Ok(())
            } else {
                Err(Refusal::NoStock)
            }
        }
    }
}

/// Lädt am Ladeplatz `bay` (die Münzen sind bezahlt, `check` ist grün). Liefert die Ladung für das Ereignis; bei der Kiste
/// ist es der Auftrag (die Kiste liegt erst bei Ankunft am Steg an Bord).
pub fn perform(isl: &mut Island, wi: usize, cfg: &CargoFile, bay: Bay, x: f64) -> Option<CargoItem> {
    match bay {
        Bay::Coins => {
            let item = CargoItem {
                kind: KIND_COINS.to_string(),
                amount: cfg.purse_coins,
                ..CargoItem::default()
            };
            isl.cargo.load.push(item.clone());
            Some(item)
        }
        Bay::Troops => {
            let ti = nearest_fighter(isl, wi, x)?;
            let t = isl.stages[wi].troops.remove(ti);
            let item = CargoItem {
                kind: KIND_TROOP.to_string(),
                id: t.kind,
                amount: 1,
                hp: t.hp,
            };
            isl.cargo.load.push(item.clone());
            Some(item)
        }
        Bay::Crates => {
            let (res, have) = largest_stock(isl);
            if have < cfg.crate_units {
                return None;
            }
            *stock_field(&mut isl.stock, res)? -= cfg.crate_units;
            isl.cargo.orders.push(CrateOrder {
                resource: res.to_string(),
                units: cfg.crate_units,
                carrier: 0,
                eta: 0.0,
            });
            isl.sync_stock();
            Some(CargoItem {
                kind: KIND_CRATE.to_string(),
                id: res.to_string(),
                amount: cfg.crate_units,
                hp: 0.0,
            })
        }
    }
}
