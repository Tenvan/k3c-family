//! Tests zu `can_learn`, `can_respec` und `learnable_skills` (Go: `monarch_can_test.go`, SK1.1, B-270): gleiche Regeln wie die
//! Aktionen, aber ohne Wirkung. Gos `copyPlayer` (Kopie mit eigenen Skill- und Slot-Listen) ist hier eine Kopie der Welt.

use super::*;
use crate::enemies::testutil::{add_player, quiet_world};
use crate::island::testutil::{add_pool_points, must_learn};

#[test]
fn can_learn_wie_learn_skill_ohne_seiteneffekt() {
    // (Name, Skill, zusätzliche Pool-Punkte vor dem Fall, erwarteter Fehler)
    let cases = [
        ("unbekannt", "gibtEsNicht", 5, true),
        ("schon gelernt", "taunt", 5, true),
        ("keine Punkte", "shieldBash", 0, true),
        ("Tier fehlt", "ironWall", 5, true),
        ("erlaubt", "shieldBash", 5, false),
    ];
    for (name, id, points, want_err) in cases {
        let mut w = quiet_world();
        let pi = add_player(&mut w);
        let v = vals(&w);
        add_pool_points(&mut w, 1);
        must_learn(&mut w, pi, &["taunt"]);
        add_pool_points(&mut w, points);
        let (skills, slots) = (w.players[pi].skills.clone(), w.players[pi].slots.clone());
        let can = can_learn(&w, &v, pi, id);
        let p = &w.players[pi];
        assert!(
            p.skills == skills && p.slots == slots,
            "{name}: CanLearn änderte den Spieler"
        );
        let mut copy = w.clone();
        let got = learn_skill(&mut copy, &v, pi, id);
        assert_eq!(can, got, "{name}: CanLearn {can:?}, LearnSkill {got:?}");
        assert_eq!(
            can.is_err(),
            want_err,
            "{name}: Fehler {can:?}, erwartet Fehler: {want_err}"
        );
    }
}

#[test]
fn can_respec_wie_respec_ohne_seiteneffekt() {
    // (Name, Phase, Abstand zur Burg, Skills gelernt, erwarteter Fehler)
    let cases = [
        ("Nacht", "night", 0.0, true, true),
        ("fern der Burg", "day", 10.0, true, true),
        ("Tag an der Burg", "day", 0.0, true, false),
        ("ohne gelernte Skills", "day", 0.0, false, false),
    ];
    for (name, phase, dx, learn, want_err) in cases {
        let mut w = quiet_world();
        let pi = add_player(&mut w);
        let v = vals(&w);
        add_pool_points(&mut w, 2);
        if learn {
            must_learn(&mut w, pi, &["heal", "groupHeal"]);
        }
        w.cycle.phase = phase.into();
        w.players[pi].x = w.castle.x + dx;
        let p = &w.players[pi];
        let (skills, slots, base) = (p.skills.clone(), p.slots.clone(), p.skill_base);
        let can = can_respec(&w, &v, pi);
        let p = &w.players[pi];
        assert!(
            p.skills == skills && p.slots == slots && p.skill_base == base,
            "{name}: CanRespec änderte den Spieler"
        );
        let mut copy = w.clone();
        let got = respec(&mut copy, &v, pi);
        assert_eq!(can, got, "{name}: CanRespec {can:?}, Respec {got:?}");
        assert_eq!(
            can.is_err(),
            want_err,
            "{name}: Fehler {can:?}, erwartet Fehler: {want_err}"
        );
    }
}

/// `learnable_skills`: 1 Punkt, nichts gelernt → nur Tier 1, also taunt ja, ironWall (Tier 3) nein; ohne Punkt leer
/// (B-285/AC-01).
#[test]
fn learnable_skills_tier_gating() {
    let mut w = quiet_world();
    let pi = add_player(&mut w);
    let v = vals(&w);
    let got = learnable_skills(&w, &v, pi);
    assert!(got.is_empty(), "ohne Punkt: {got:?}");
    add_pool_points(&mut w, 1);
    let got = learnable_skills(&w, &v, pi);
    let sorted = got.windows(2).all(|p| p[0] <= p[1]);
    let ok = got.iter().any(|s| s == "taunt") && !got.iter().any(|s| s == "ironWall") && sorted;
    assert!(ok, "1 Punkt: {got:?}");
    for id in &got {
        let tier = skill_by_id(&v, id).map_or(0, |s| s.tier);
        assert_eq!(tier, 1, "{id} ist Tier {tier}");
    }
}
