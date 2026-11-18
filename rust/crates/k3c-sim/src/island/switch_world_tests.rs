//! Inselwechsel in einer Welt mit zwei Inseln (Go: `island_switch_world_test.go`, WZ18.3, B-488/AC-03,
//! docs/rules/welten.md § 6): mit 2 Spielern zur nächsten Insel der Welt-Datei, Spielstand mit ihrer Insel-ID; die letzte
//! Insel gilt je Welt. Die drei Go-Tests bauen eine Insel aus der Welt-Datei `nordmeer` (`load_world` RU10.5,
//! `create_world_with` RU14) und tragen `ignore`.
use super::switch_testutil::*;
use super::testutil::*;
use super::{Island, next_island};

/// Go: `nordmeerSwitch`: beide Spieler am Wechselpunkt von Insel 1 der Test-Welt, bis der Wechsel bereit ist; dann die
/// nächste Insel nach `next_def`.
fn nordmeer_switch(data: &k3c_data::Data) -> (Island, Island) {
    let w = nordmeer_loaded(data);
    let mut cur = nordmeer_island(data, &w, 2);
    assert!(
        !cur.is_last() && cur.world_id() == "nordmeer" && cur.island_id() == "island1",
        "Insel 1: letzte {}, Welt {:?}, Insel {:?}",
        cur.is_last(),
        cur.world_id(),
        cur.island_id()
    );
    let ev = step_seconds(&mut cur, data, data.hub.travel.seconds + 0.5);
    assert!(
        cur.switch_ready && count(&ev, "islandSwitch") == 1 && count(&ev, "victory") == 0,
        "bereit {}, {}x islandSwitch, {}x victory",
        cur.switch_ready,
        count(&ev, "islandSwitch"),
        count(&ev, "victory")
    );
    let def = cur.next_def().expect("nächste Insel");
    assert_eq!(def.id, "eisinsel", "nächste Insel");
    let next = next_island(data, &cur, &def).expect("nächste Insel bauen");
    (cur, next)
}

#[test]
#[ignore = "wartet auf RU18 (speed_of)"]
fn inselwechsel_in_welt_mit_zwei_spielern() {
    let data = test_data();
    let (_, next) = nordmeer_switch(&data);
    let first = &next.stages[0];
    assert!(
        first.players.len() == 2 && first.players[0].index == 0 && first.players[1].index == 1,
        "Spieler auf Insel 2: {}",
        first.players.len()
    );
    let s = next.to_save("2026-10-19T20:00:00Z").expect("Spielstand");
    assert!(
        s.world == "nordmeer" && s.island_id == "eisinsel" && next.number == 1,
        "Spielstand: Welt {:?}, Insel {:?}, Nummer {}",
        s.world,
        s.island_id,
        next.number
    );
}

#[test]
#[ignore = "wartet auf RU18 (speed_of)"]
fn inselwechsel_letzte_insel_der_welt() {
    let data = test_data();
    let (_, mut next) = nordmeer_switch(&data);
    assert!(
        next.next_def().is_none() && next.is_last(),
        "Insel 2 ist die letzte der Welt"
    );
    next.endboss_defeated = true; // letzte Insel der Welt: kein Wechselpunkt, Sieg
    let ev = step_seconds(&mut next, &data, 3.0);
    assert!(
        count(&ev, "islandGateOpen") == 0 && !next.switch_ready && count(&ev, "victory") == 1,
        "letzte Insel: {}x islandGateOpen, bereit {}, {}x victory",
        count(&ev, "islandGateOpen"),
        next.switch_ready,
        count(&ev, "victory")
    );
}

/// `next_island` baut nur die nächste Insel der Welt (WZ18.4, Review WZ18.3): eine andere Insel ist ein Fehler.
#[test]
fn inselwechsel_nur_zur_naechsten_insel() {
    let data = test_data();
    let w = nordmeer_loaded(&data);
    let isl = nordmeer_island(&data, &w, 1);
    assert!(
        next_island(&data, &isl, &w.islands[0]).is_err(),
        "Wechsel auf Insel 1 statt Insel 2 angenommen"
    );
}
