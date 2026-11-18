//! Spielstand Version 5 und 6 (Port von `engine/sim/island_save_v5.go`, K2.3b, B-130, B-103, B-373): besiegte Bosse, Münzzähler
//! der Siegvariante `gold`, Sieg schon gemeldet, Händler. Ältere Versionen laden mit leerer Bossliste und Zählern 0 (die
//! Felder fehlen dort). `validate_progress` steht bei den Lese-Funktionen (`save_parse.rs`, seit RU10.2).
//!
//! Gos Paket-Variable `bosses` ist der Parameter `data`.

use k3c_data::Data;

use super::{Island, IslandSave, MerchantSave};
use crate::state::pending::hub_of;
use crate::state::shared::Merchant;

/// Go: `bossByID(id).Kind == "end"`: Ist `id` ein Endboss? Eine unbekannte ID (Go: `nil`, nach `validate_progress` nie)
/// gilt nicht als Endboss.
fn is_end_boss(data: &Data, id: &str) -> bool {
    let b = &data.bosses;
    b.bosses
        .iter()
        .chain(&b.end_bosses)
        .find(|x| x.id == id)
        .is_some_and(|x| x.kind == "end")
}

/// Go: `restoreProgress` setzt besiegte Bosse und Zähler; der Endboss gilt als besiegt, wenn ein Endboss in der Liste
/// steht (eine Quelle der Wahrheit). Der Händler kommt in die Stufe der Tiefe `hub.merchant.onlyDepth`.
pub fn restore_progress(isl: &mut Island, s: &IslandSave, data: &Data) {
    isl.defeated_bosses = s.defeated_bosses.clone();
    isl.endboss_defeated = isl.defeated_bosses.iter().any(|id| is_end_boss(data, id));
    isl.gold_collected = s.gold_collected;
    isl.won = s.won;
    isl.merchant_visits = s.merchant_visits; // Version 6 (B-373); ältere Stände: 0, kein Händler
    let Some(m) = &s.merchant else {
        return;
    };
    for w in &mut isl.stages {
        if w.biome.depth == hub_of(w).merchant.only_depth {
            w.merchant = Some(Merchant {
                resource: m.resource.clone(),
                leaves: m.leaves,
                buy_paid: m.buy_paid,
                raid: m.raid,
                hp: m.hp,
                max_hp: m.max_hp,
            });
        }
    }
}

/// Go: `savedMerchant` ist der anwesende Händler der Insel (höchstens einer, in Tiefe `hub.merchant.onlyDepth`) oder `None`.
pub fn saved_merchant(isl: &Island) -> Option<MerchantSave> {
    isl.stages.iter().find_map(|w| {
        w.merchant.as_ref().map(|m| MerchantSave {
            resource: m.resource.clone(),
            leaves: m.leaves,
            buy_paid: m.buy_paid,
            raid: m.raid,
            hp: m.hp,
            max_hp: m.max_hp,
        })
    })
}
