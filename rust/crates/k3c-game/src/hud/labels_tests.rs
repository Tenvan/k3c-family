//! Port von `native/k3c-game/hud/labels_test.go`.
#![allow(clippy::unimplemented)] // Platzhalter: wartet auf den genannten Sprint (RU10, RU15, RU18, RU23)

use super::state::SeatState;
use k3c_sim::state::{Merchant, Troop, World};

/// Review-Befund: Labels außerhalb des Bilds verbrauchen das Budget `MAX_LABELS` nicht; der Händler steht vorn und
/// fehlt auch bei vielen Berufen nicht. Braucht `visible_labels` und `MAX_LABELS` (drawworld.go, RU23).
#[test]
#[ignore = "wartet auf RU23"]
fn visible_labels_budget_after_culling() {
    unimplemented!("RU23: visible_labels, MAX_LABELS (Go: TestVisibleLabelsBudgetAfterCulling)");
}

/// Teil von `visible_labels_budget_after_culling`, der ohne `visible_labels` läuft: Der Händler steht vorn (ohne
/// Berufssymbol), dahinter je ein Label pro Bauern mit Beruf.
#[test]
fn citizen_labels_haendler_vorn() {
    let mut st = SeatState::default();
    let mut w = World {
        merchant: Some(Merchant {
            resource: "wood".to_string(),
            ..Default::default()
        }),
        ..Default::default()
    };
    for _ in 0..5 {
        w.troops.push(Troop {
            profession: "miner".to_string(),
            ..Default::default()
        });
    }
    w.troops.push(Troop::default());
    st.citizen_labels("de", &w);
    assert_eq!(st.labels.len(), 6, "Händler und fünf Berufe, Bauer ohne Beruf nicht");
    assert_eq!(st.labels[0].dot, 0, "Händler zuerst");
    assert!(
        st.labels[1..].iter().all(|l| l.dot == 0x8d99ae),
        "Berufssymbol des Bergmanns"
    );
}
