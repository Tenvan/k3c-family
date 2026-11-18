//! Go: `events_merchant_test.go` (K3.2, B-131/AC-01, AC-02): Händler-Überfall. Die Tests rechnen `merchant_raid_start` und
//! `merchant_raid_end` direkt auf einer Händler-Stufe mit dem Bezug `IslandRef` (Besuchszähler); die Läufe über ganze
//! Inseln warten auf `decode_world_story`.

use serde_json::json;

use super::merchant::{merchant_raid_end, merchant_raid_start};
use super::values::night;
use crate::state::{IslandRef, Merchant, World};
use crate::world::Options;
use crate::world::testutil::new_world;

fn raid_world() -> World {
    let mut w = new_world(0, "haendler", Options::default());
    w.troops.clear();
    w.camps.clear();
    w.pickups.clear();
    w.portals = vec![5.0, 95.0];
    w.merchant = Some(Merchant {
        resource: "wood".into(),
        hp: 10.0,
        max_hp: 10.0,
        ..Default::default()
    });
    w
}

fn started(w: &World) -> bool {
    w.events
        .iter()
        .any(|e| e["type"] == "eventStarted" && e["event"] == "merchantRaid")
}

/// (a) Besuch 1 bis 3 ohne Überfall, Besuch 4 mit, Besuch 8 wieder; der Rhythmus kommt aus den Daten.
#[test]
fn ueberfall_jeder_vierte_besuch() {
    let every = night(&raid_world(), "merchantRaid").expect("merchantRaid").every_visits;
    for visit in 1..=2 * every {
        let mut w = raid_world();
        let isl = IslandRef {
            merchant_visits: visit,
            players: 2,
            ..Default::default()
        };
        merchant_raid_start(&mut w, Some(&isl));
        let want = visit % every == 0;
        let raid = w.merchant.as_ref().is_some_and(|m| m.raid);
        assert!(
            started(&w) == want && raid == want && (!w.spawn_queue.is_empty()) == want,
            "Besuch {visit}"
        );
    }
}

/// (b) Ohne Insel oder Händler kein Überfall.
#[test]
fn ueberfall_nur_bei_besuch_auf_einer_insel() {
    let mut w = raid_world();
    merchant_raid_start(&mut w, None);
    assert!(!started(&w), "ohne Insel kein Überfall (Golden-Läufe bleiben gleich)");
    let mut w = raid_world();
    w.merchant = None;
    let isl = IslandRef {
        merchant_visits: 4,
        ..Default::default()
    };
    merchant_raid_start(&mut w, Some(&isl));
    assert!(!started(&w), "ohne Händler kein Überfall");
}

/// (c) Der Händler überlebt bis zur Abreise: `rewardMaterial` seines Materials im Vorrat, eventEnded mit protected.
#[test]
fn belohnung_bei_schutz() {
    let mut w = raid_world();
    let reward = night(&w, "merchantRaid").expect("merchantRaid").reward_material;
    w.merchant.as_mut().expect("Händler").raid = true;
    merchant_raid_end(&mut w, None, true);
    assert_eq!(w.stock.wood, reward);
    assert!(w.events.iter().any(|e| e["type"] == "eventEnded"
        && e["protected"] == json!(true)
        && e["amount"] == json!(reward)
        && e["resource"] == "wood"));
}

/// (d) Der Händler flieht: keine Belohnung, eventEnded ohne Schutz.
#[test]
fn keine_belohnung_bei_flucht() {
    let mut w = raid_world();
    w.merchant.as_mut().expect("Händler").raid = true;
    merchant_raid_end(&mut w, None, false);
    assert_eq!(w.stock.wood, 0);
    assert!(
        w.events
            .iter()
            .any(|e| e["type"] == "eventEnded" && e["protected"] == json!(false) && e["amount"] == json!(0))
    );
}

/// Ohne Überfall endet nichts.
#[test]
fn ohne_ueberfall_kein_ende() {
    let mut w = raid_world();
    merchant_raid_end(&mut w, None, true);
    assert!(w.events.is_empty() && w.stock.wood == 0);
}
