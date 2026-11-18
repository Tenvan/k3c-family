//! Tests der Abfragen aus `data.rs` gegen `data/bosses.json` (Lader aus `k3c-data`); Go hat dafür keinen eigenen Test.

use super::data::{boss_by_id, end_boss, mini_boss};
use super::testutil::boss_values;

#[test]
fn mini_boss_je_stufe_der_insel_eins() {
    let v = boss_values();
    let want = ["goblinLeader", "trollKing", "ratQueen", "lavaGolem", "shardTitan"];
    for (depth, id) in want.iter().enumerate() {
        let b = mini_boss(&v.bosses, depth as i64).expect("Miniboss der Stufe");
        assert_eq!((b.id.as_str(), b.kind.as_str()), (*id, "mini"));
    }
}

#[test]
fn mini_boss_ohne_eintrag_ist_none() {
    let v = boss_values();
    assert!(mini_boss(&v.bosses, 99).is_none());
    assert!(mini_boss(&v.bosses, -1).is_none());
}

#[test]
fn boss_by_id_sucht_erst_minibosse_dann_endbosse() {
    let v = boss_values();
    assert_eq!(
        boss_by_id(&v.bosses, "goblinLeader").map(|b| b.kind.as_str()),
        Some("mini")
    );
    let end = v.bosses.end_bosses.first().expect("mindestens ein Endboss");
    assert_eq!(boss_by_id(&v.bosses, &end.id).map(|b| b.kind.as_str()), Some("end"));
    assert!(boss_by_id(&v.bosses, "gibtEsNicht").is_none());
    assert!(boss_by_id(&v.bosses, "").is_none());
}

#[test]
fn end_boss_nach_tiefe() {
    let v = boss_values();
    for b in &v.bosses.end_bosses {
        assert_eq!(end_boss(&v.bosses, b.depth).map(|e| e.id.as_str()), Some(b.id.as_str()));
    }
    assert!(end_boss(&v.bosses, 99).is_none());
}
