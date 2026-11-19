//! Spielstand-Teil der Fracht (RU62.1, `ISLAND_SAVE_VERSION` 10, Muster `island/save_v8.rs`): Fracht an Bord und offene
//! Träger-Aufträge. Laufende Einwürfe zählen nicht (die Münzen sind beim Speichern noch beim Spieler oder gelten als
//! abgebrochen); der Träger eines Auftrags wird beim Laden neu gesucht, `eta` bleibt.

use serde::{Deserialize, Serialize};

use super::{Cargo, CargoItem, CrateOrder, KIND_COINS, KIND_CRATE, KIND_TROOP};
use crate::island::storage::stock_field;
use crate::state::Fehler;
use k3c_data::Data;
use k3c_data::hub::Stock;

/// Eine Ladung im Spielstand.
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
#[serde(default)]
pub struct CargoItemSave {
    #[serde(rename = "kind")]
    pub kind: String,
    #[serde(rename = "id", skip_serializing_if = "String::is_empty")]
    pub id: String,
    #[serde(rename = "amount")]
    pub amount: i64,
    #[serde(rename = "hp", skip_serializing_if = "is_zero_f64")]
    pub hp: f64,
}

/// Ein offener Träger-Auftrag im Spielstand.
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
#[serde(default)]
pub struct CrateOrderSave {
    #[serde(rename = "resource")]
    pub resource: String,
    #[serde(rename = "units")]
    pub units: i64,
    #[serde(rename = "eta", skip_serializing_if = "is_zero_f64")]
    pub eta: f64,
}

/// Fracht und Aufträge im Spielstand (`IslandSave::cargo`; fehlt, wenn nichts an Bord ist).
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
#[serde(default)]
pub struct CargoSave {
    #[serde(rename = "load", skip_serializing_if = "Vec::is_empty")]
    pub load: Vec<CargoItemSave>,
    #[serde(rename = "orders", skip_serializing_if = "Vec::is_empty")]
    pub orders: Vec<CrateOrderSave>,
}

fn is_zero_f64(v: &f64) -> bool {
    *v == 0.0
}

/// Der Teil des Spielstands (`None`, wenn nichts an Bord ist).
pub fn to_save(c: &Cargo) -> Option<CargoSave> {
    if c.load.is_empty() && c.orders.is_empty() {
        return None;
    }
    Some(CargoSave {
        load: c
            .load
            .iter()
            .map(|i| CargoItemSave {
                kind: i.kind.clone(),
                id: i.id.clone(),
                amount: i.amount,
                hp: i.hp,
            })
            .collect(),
        orders: c
            .orders
            .iter()
            .map(|o| CrateOrderSave {
                resource: o.resource.clone(),
                units: o.units,
                eta: o.eta,
            })
            .collect(),
    })
}

/// `restore`, wenn der Stand Fracht trägt.
pub fn restore_opt(c: &mut Cargo, s: Option<&CargoSave>, data: &Data) -> Result<(), Fehler> {
    s.map_or(Ok(()), |s| restore(c, s, data))
}

/// Prüft den Teil gegen die Daten und setzt ihn in `c` ein (die Insel muss Fracht tragen: `c.cfg` gesetzt).
pub fn restore(c: &mut Cargo, s: &CargoSave, data: &Data) -> Result<(), Fehler> {
    let fail = |m: String| Fehler::Spielstand(m);
    if c.cfg.is_none() {
        return Err(fail("spielstand: Fracht in einer Welt ohne Fracht".to_string()));
    }
    let known = |res: &str| stock_field(&mut Stock::default(), res).is_some();
    let mut load = Vec::new();
    for i in &s.load {
        let ok = match i.kind.as_str() {
            KIND_COINS => i.amount > 0,
            KIND_TROOP => data.troops.contains_key(&i.id) && i.amount == 1,
            KIND_CRATE => known(&i.id) && i.amount > 0,
            _ => false,
        };
        if !ok {
            return Err(fail(format!("spielstand: Fracht {:?} {:?} ungültig", i.kind, i.id)));
        }
        load.push(CargoItem {
            kind: i.kind.clone(),
            id: i.id.clone(),
            amount: i.amount,
            hp: i.hp,
        });
    }
    let mut orders = Vec::new();
    for o in &s.orders {
        if !known(&o.resource) || o.units <= 0 || o.eta < 0.0 {
            return Err(fail(format!("spielstand: Träger-Auftrag {:?} ungültig", o.resource)));
        }
        orders.push(CrateOrder {
            resource: o.resource.clone(),
            units: o.units,
            carrier: 0,
            eta: o.eta,
        });
    }
    // mehr Fracht als Plätze: der Laderaum kann nicht mehr dastehen, der Takt gibt den Überschuss zurück (trim_overflow)
    c.load = load;
    c.orders = orders;
    Ok(())
}
