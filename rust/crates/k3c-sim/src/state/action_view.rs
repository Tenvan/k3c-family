//! Ausbilden und Tauschen für die Aktionsliste (Port von `engine/sim/action_view.go`): nur lesend aus dem Zahlziel
//! abgelesen, das ein Takt A am Ort des Spielers bezahlen würde (`find_pay_target`); nichts gespeichert, Welt-JSON und
//! Golden bleiben gleich.

use super::pending::{find_pay_target, profession_of, values};
use super::{IslandRef, Player, World};
use crate::island::storage::{resource_full, stock_get};

/// Go: `TrainAt`: der Beruf (miner, builder, craftsman), den der Spieler an seinem Ort ausbilden kann: Das Zahlziel ist
/// ein bezahlbares Berufs-Angebot und der Spieler hat Gold; sonst "".
pub fn train_at(w: &World, p: &Player) -> String {
    let t = find_pay_target(w, p);
    let Some(offer) = t.as_ref().and_then(|t| t.offer.as_ref()).filter(|_| p.gold > 0) else {
        return String::new();
    };
    if profession_of(&values(w).troops, offer.kind()).is_none() {
        return String::new(); // Elite-Angebot der Schmiede: kein Beruf
    }
    offer.kind().to_string()
}

/// Go: `TradeAt`: die Seite des Händlers ("buy", "sell"), an der der Spieler jetzt tauschen kann, mit dem Material des
/// Händlers: Kaufen braucht Gold und Platz im Lager, Verkaufen genug Material im Vorrat; sonst `("", "")`.
#[allow(clippy::expect_used)] // STAB(B-1076): Go-Nil-Dereferenz, bis RU22 den Fehlerweg festlegt
pub fn trade_at(w: &World, p: &Player, isl: Option<&IslandRef>) -> (String, String) {
    let none = (String::new(), String::new());
    let t = match find_pay_target(w, p) {
        Some(t) if !t.merchant.is_empty() => t,
        _ => return none,
    };
    let m = values(w).economy.merchant.clone();
    let mer = w.merchant.as_ref().expect("nil pointer dereference: w.merchant");
    if t.merchant == "sell" {
        match stock_get(&w.stock, &mer.resource) {
            Some(f) if f >= m.material => {}
            _ => return none,
        }
    } else if p.gold <= 0 || mer.buy_paid >= m.gold || resource_full(w, &values(w), isl, &mer.resource) {
        return none;
    }
    (t.merchant, mer.resource.clone())
}
