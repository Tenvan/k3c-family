//! Tests zu `bot_learn.rs` (Port von `engine/balance/bot_learn_test.go`).

use k3c_sim::monarch::available_points;

use crate::bot_learn::bot_learn;
use crate::testutil::scenario_island;

/// B-647 Weg B: Ein Bot mit freien Skill-Punkten lernt in der Reihenfolge der Tank-Linie, nie mehr als er Punkte hat.
#[test]
fn bot_lernt_skills_in_reihenfolge() {
    let mut isl = scenario_island("1", 1);
    bot_learn(&mut isl);
    assert!(
        isl.stages[0].players[0].skills.is_empty(),
        "ohne Punkte gelernt: {:?}",
        isl.stages[0].players[0].skills
    );
    isl.skill_pool = 3;
    let pool = isl.skill_pool;
    for s in &mut isl.stages {
        s.skill_points = pool;
    }
    bot_learn(&mut isl);
    let want = ["taunt", "shieldBash", "regeneration"];
    assert_eq!(isl.stages[0].players[0].skills, want, "gelernt");
    let left = available_points(&isl.stages[0], 0);
    assert_eq!(left, 0, "Punkte übrig: {left}");
    bot_learn(&mut isl); // ein zweiter Aufruf ändert nichts
    assert_eq!(
        isl.stages[0].players[0].skills.len(),
        3,
        "zweiter Aufruf lernte weiter: {:?}",
        isl.stages[0].players[0].skills
    );
}
