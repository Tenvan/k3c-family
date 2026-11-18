//! Ausbilden und Tauschen für die Aktionsliste (Port von `engine/sim/action_view.go`): nur lesend aus dem Zahlziel
//! abgelesen, das ein Takt A am Ort des Spielers bezahlen würde (`find_pay_target`); nichts gespeichert, Welt-JSON und
//! Golden bleiben gleich.

use super::pending::{profession_of, values};
use super::{IslandRef, Player, World};
use crate::economy::{PayTarget, find_pay_target, offer_target};
use crate::island::storage::{resource_full, stock_get};

/// Go: `TrainAt`: der Beruf (miner, builder, craftsman), den der Spieler an seinem Ort ausbilden kann: Das Zahlziel ist
/// ein bezahlbares Berufs-Angebot und der Spieler hat Gold; sonst "".
pub fn train_at(w: &World, p: &Player, isl: Option<&IslandRef>) -> String {
    let v = values(w);
    let Some(PayTarget::Offer { site, idx }) = find_pay_target(w, &v, isl, p).filter(|_| p.gold > 0) else {
        return String::new();
    };
    let Some(offer) = offer_target(w, &v, site, idx) else {
        return String::new();
    };
    if profession_of(&v.troops, offer.kind()).is_none() {
        return String::new(); // Elite-Angebot der Schmiede: kein Beruf
    }
    offer.kind().to_string()
}

/// Go: `TradeAt`: die Seite des Händlers ("buy", "sell"), an der der Spieler jetzt tauschen kann, mit dem Material des
/// Händlers: Kaufen braucht Gold und Platz im Lager, Verkaufen genug Material im Vorrat; sonst `("", "")`.
#[allow(clippy::expect_used)] // STAB(B-1076): Go-Nil-Dereferenz, bis RU22 den Fehlerweg festlegt
pub fn trade_at(w: &World, p: &Player, isl: Option<&IslandRef>) -> (String, String) {
    let none = (String::new(), String::new());
    let v = values(w);
    let side = match find_pay_target(w, &v, isl, p) {
        Some(PayTarget::Merchant(side)) if !side.is_empty() => side,
        _ => return none,
    };
    let m = v.economy.merchant.clone();
    let mer = w.merchant.as_ref().expect("nil pointer dereference: w.merchant");
    if side == "sell" {
        match stock_get(&w.stock, &mer.resource) {
            Some(f) if f >= m.material => {}
            _ => return none,
        }
    } else if p.gold <= 0 || mer.buy_paid >= m.gold || resource_full(w, &v, isl, &mer.resource) {
        return none;
    }
    (side, mer.resource.clone())
}
