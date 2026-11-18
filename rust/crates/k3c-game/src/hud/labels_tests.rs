//! Port von `native/k3c-game/hud/labels_test.go`.

use super::drawworld::visible_labels;
use super::paint::MAX_LABELS;
use super::state::{Label, SeatState};
use crate::view::cell::{Cell, new_camera, rect};
use k3c_sim::state::{Merchant, Troop, World};

/// Review-Befund: Labels außerhalb des Bilds verbrauchen das Budget `MAX_LABELS` nicht; der Händler steht vorn und
/// fehlt auch bei vielen Berufen nicht.
#[test]
fn visible_labels_budget_after_culling() {
    let r = rect(0, 0, 1280, 720);
    let c = Cell {
        index: 0,
        player: 0,
        rect: r,
        camera: new_camera(r, 0.0, 32.0), // sichtbar etwa −20 … 20 Units
    };
    let label = |x: f64, text: &str| Label {
        x,
        text: text.to_string(),
        ..Label::default()
    };
    let mut labels: Vec<Label> = (0..2 * MAX_LABELS).map(|i| label(500.0 + i as f64, "fern")).collect();
    labels.extend((0..3).map(|i| label(f64::from(i), "nah")));
    let got = visible_labels(Vec::new(), &labels, &c, 1.0);
    assert!(got.len() == 3 && labels[got[0].i].text == "nah", "sichtbar {got:?}");
    let mut st = SeatState::default();
    let mut w = World {
        merchant: Some(Merchant {
            resource: "wood".to_string(),
            ..Default::default()
        }),
        ..Default::default()
    };
    for _ in 0..MAX_LABELS + 5 {
        w.troops.push(Troop {
            profession: "miner".to_string(),
            ..Default::default()
        });
    }
    st.citizen_labels("de", &w);
    let got = visible_labels(Vec::new(), &st.labels, &c, 1.0);
    assert!(
        got.len() == MAX_LABELS && got[0].i == 0 && st.labels[0].dot == 0,
        "Händler nicht vorn bzw. Budget {}",
        got.len()
    );
}

/// Der Bau der Labels allein (ohne Sichtprüfung): Der Händler steht vorn (ohne Berufssymbol), dahinter je ein Label pro
/// Bauern mit Beruf.
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
