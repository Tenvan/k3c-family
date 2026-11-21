//! Fälle aus src/scenes/render/priceTagRules.test.ts, siteStatusView.test.ts und hud/economyLabels.

use std::sync::atomic::{AtomicUsize, Ordering};

use super::data::values;
use super::sites::*;

fn c(id: i64, x: f64) -> Candidate {
    Candidate { id, x }
}

#[test]
fn nearest_tags_faelle() {
    type Case = (&'static str, Vec<Candidate>, Vec<f64>, Vec<i64>);
    let cases: [Case; 6] = [
        (
            "nur das nächste",
            vec![c(1, 14.0), c(2, 11.0), c(3, 8.5)],
            vec![10.0],
            vec![2],
        ),
        (
            "gleicher Abstand: kleinere Nummer",
            vec![c(7, 12.0), c(4, 8.0)],
            vec![10.0],
            vec![4],
        ),
        (
            "zwei Spieler je ein Schild",
            vec![c(1, 5.0), c(2, 30.0)],
            vec![6.0, 29.0],
            vec![1, 2],
        ),
        ("außer Reichweite", vec![c(1, 40.0)], vec![10.0], vec![]),
        ("gefallener Monarch fehlt", vec![c(2, 11.0)], vec![], vec![]),
        (
            "zwei Spieler am selben Platz",
            vec![c(3, 10.0)],
            vec![9.0, 11.0],
            vec![3],
        ),
    ];
    for (name, cands, players, want) in cases {
        let mut got = Vec::new();
        nearest_tags(&mut got, &cands, &players);
        assert_eq!(got, want, "{name}");
    }
}

#[test]
fn offers_price_faelle() {
    let b = &values().buildings;
    let rack = b["bowyer"].bow_rack;
    let hammer = b["workshop"].hammer_rack;
    assert!(
        rack != 0 && hammer != 0,
        "Bogen-Regal des Bogners oder Hammer-Regal der Werkstatt fehlt in data/buildings.json"
    );
    let cases = [
        ("wall", "unpaid", false, 0, false),
        ("wall", "unpaid", true, 0, true),
        ("wall", "built", true, 0, false),
        ("bowyer", "built", false, 0, true),
        ("bowyer", "built", false, rack, false),
        ("workshop", "built", false, 0, true),
        ("workshop", "built", false, hammer, false),
        ("smithy", "built", false, 0, false),
        ("wall", "waitingWorker", true, 0, false),
    ];
    for (kind, state, payable, bows, want) in cases {
        assert_eq!(
            offers_price(kind, state, payable, bows),
            want,
            "{kind} {state} {payable} {bows}"
        );
    }
    assert!(
        offers_hub_upgrade(true, "", 50, 0)
            && !offers_hub_upgrade(true, "waitingMaterial", 50, 0)
            && !offers_hub_upgrade(true, "", 50, 50)
            && !offers_hub_upgrade(false, "", 50, 0),
        "Ausbau-Angebot der Burg nur mit offenem Gold und ohne laufenden Ausbau (B-591)"
    );
}

#[test]
fn site_wait_faelle() {
    use WaitReason::*;
    let cases = [
        ("waitingWorker", "", 0.0, false, Worker),
        ("waitingWorker", "", 0.0, true, Danger),
        ("waitingMaterial", "", 0.0, false, Material),
        ("unpaid", "", 0.0, false, None),
        ("built", "", 0.0, false, None),
        ("waitingWorker", "", 0.4, false, None),
        ("waitingWorker", "", 0.4, true, Danger),
        ("built", "waitingWorker", 0.0, false, Worker),
        ("built", "waitingNextIsland", 0.0, false, Material),
    ];
    for (state, upgrade, progress, danger, want) in cases {
        assert_eq!(
            site_wait(state, upgrade, progress, danger).0,
            want,
            "{state} {upgrade} {progress} {danger}"
        );
    }
    assert_eq!([None as u8, Worker as u8, Material as u8, Danger as u8], [0, 1, 2, 3]);
}

fn key(what: TextWhat, kind: &str, state: &str, n: i64, m: WaitReason) -> TextKey {
    TextKey {
        what,
        kind: kind.to_string(),
        state: state.to_string(),
        n,
        m: i64::from(m as u8),
    }
}

#[test]
fn texte() {
    use TextWhat::*;
    use WaitReason as W;
    let cases = [
        (key(Price, "wall", "", 0, W::None), "Mauer"),
        (key(Price, "stairsUp", "", 1, W::None), "Treppe hoch"),
        (key(Price, "stairsUp", "", 0, W::None), "Treppe hoch"),
        (key(Clearing, "tower", "", 0, W::None), "Turm: erst roden"),
        (
            key(Status, "storage", "waitingWorker", 0, W::Worker),
            "Lager: wartet auf Bauer",
        ),
        (
            key(Status, "wall", "waitingMaterial", 0, W::Material),
            "Mauer: wartet auf Material (Holz)",
        ),
        (
            key(Status, "wall", "waitingMaterial", 2, W::Material),
            "Mauer: wartet auf Material (Holz)",
        ),
        (
            key(Status, "wall", "waitingWorker", 0, W::Danger),
            "Mauer: wartet: Gefahr",
        ),
        (key(Bow, "", "", 0, W::None), "Bogen"),
        (key(Name, "ship", "", 0, W::None), "Schiff"),
    ];
    for (k, want) in cases {
        assert_eq!(site_text(&k), want, "{k:?}");
    }
    assert!(
        hub_title(2, 3) == "2/3" && hub_title(0, 0) == "Ausbauen",
        "Titel des Ausbau-Schilds (B-592)"
    );
    assert_eq!(
        wait_text(W::Material, "waitingNextIsland", "Kupfer"),
        "Tor-Material fehlt (Kupfer)"
    );
}

static CALLS: AtomicUsize = AtomicUsize::new(0);

fn counting(_: &TextKey) -> String {
    CALLS.fetch_add(1, Ordering::SeqCst);
    "x".to_string()
}

#[test]
fn material_rechnung() {
    let (need, stock) = ([0, 100, 40, 0, 0], [0, 120, 10, 0, 0]);
    assert_eq!(missing_arr(need, stock), [0, 0, 40, 0, 0], "fehlend");
    assert!(
        !covers_arr(stock, need) && covers_arr([0, 100, 40, 0, 0], need),
        "covers_arr"
    );
    let afford = k3c_data::buildings::Cost {
        stone: 100,
        gold: 99,
        ..Default::default()
    };
    let copper = k3c_data::buildings::Cost {
        copper: 11,
        ..Default::default()
    };
    assert!(
        can_afford(stock, &afford) && !can_afford(stock, &copper),
        "can_afford zählt Material, nicht Gold"
    );
    assert_eq!(
        price_label("Ausbau 1/3", &hub_material([0, 100, 0, 0, 0])),
        "Ausbau 1/3 · 100 Stein"
    );
    let mut tx = Texts::default();
    let k = TextKey::new(TextWhat::Name);
    tx.get(&k, counting);
    tx.get(&k, counting);
    assert_eq!(CALLS.load(Ordering::SeqCst), 1, "Text mehrfach gebaut");
}
