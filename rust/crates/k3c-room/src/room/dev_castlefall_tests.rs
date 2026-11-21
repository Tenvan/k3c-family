//! Test aus `dev_castlefall_test.go` (WZ14.5, B-468/AC-05): Die Dev-Aktion `castleFall` löst den Stufenverlust in der Stufe
//! des Monarchen aus, über Gerät und `/dm`.
use super::dev_tests::{act, act_slot, dev, dev_room};
use super::testutil::{create_room_in, einzel_manager, rm};
use crate::fehler::Fehler;

#[test]
fn dev_burgfall() {
    let (mut m, code, _i) = dev_room();
    let r = rm(&mut m, &code);
    let idx = r.devices["a"].slots[&0] as i64;
    let stage = r.isl.stage_of(idx).expect("Stufe");
    for p in &mut r.isl.stages[stage].players {
        p.gold = 10;
    }
    assert_eq!(dev(&mut m, &code, &act_slot("castleFall", 0, 0)), Ok(()));
    let r = rm(&mut m, &code);
    for p in &r.isl.stages[stage].players {
        assert_eq!(p.gold, 5, "Gold von Monarch {} nach Burgfall, erwartet 5", p.index);
    }
    for t in &r.isl.stages[stage].troops {
        assert!(t.is_wild(), "Truppe {} überlebt den Burgfall", t.kind);
    }
    assert_eq!(
        dev(&mut m, &code, &act("castleFall")),
        Err(Fehler::BadRequest),
        "ohne slot"
    );
    let page = act_slot("castleFall", idx, 0);
    assert_eq!(
        m.with_room(&code, |r, env| r.dev_page(env, &page)).expect("Raum"),
        Ok(()),
        "über /dm"
    );
    assert!(
        m.log.lines.iter().any(|l| l.contains("aktion=castleFall")),
        "Log ohne aktion=castleFall:\n{:?}",
        m.log.lines
    );
}

/// RU36.3 (B-1157): `readyToSail` kommt im Raum an; auf der letzten Insel der Welt gibt es keine Abfahrt (abgelehnt, die
/// Insel bleibt unverändert). Schiff und Wechselpunkt prüft `k3c-sim` (`dev/sail_tests.rs`).
#[test]
fn dev_abfahrbereit_nur_mit_naechster_insel() {
    let (mut m, _dir) = einzel_manager("abfahrt"); // Welt mit einer Insel
    let (code, _i) = create_room_in(&mut m, "einzel", "a", 1, "dev", 0, &[0, 1]);
    assert_eq!(dev(&mut m, &code, &act("readyToSail")), Err(Fehler::BadRequest));
    assert!(!rm(&mut m, &code).isl.endboss_defeated);
}
