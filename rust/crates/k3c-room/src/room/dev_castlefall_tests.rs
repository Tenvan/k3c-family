//! Test aus `dev_castlefall_test.go` (WZ14.5, B-468/AC-05): Die Dev-Aktion `castleFall` löst den Stufenverlust in der Stufe
//! des Monarchen aus, über Gerät und `/dm`.
use super::dev_tests::{act, act_slot, dev, dev_room};
use super::testutil::rm;
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
        assert_eq!(t.kind, "vagrant", "Truppe {} überlebt den Burgfall", t.kind);
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
