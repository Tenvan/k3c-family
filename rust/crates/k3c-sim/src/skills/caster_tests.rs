//! Zauberer-Linie (Go: `skills_caster_test.go`): Wirkung, Fläche, Reichweite und Abklingzeit laut monarch.json (S1.2b, B-119).
//! Jeder Zauber meldet `skillCast` (`emit`, `unit_x`) und jeder Treffer `hit_event` (RU19); diese Tests warten darauf. Die
//! Tests ab `own_` haben kein Go-Gegenstück und prüfen Slow, Sturm-Anlage und Sturm-Dauer ohne diese Funktionen.

use super::caster::{cast_area_damage, cast_damage_over_time, cast_slow, step_storms};
use super::testutil::*;

use crate::enemies::step_enemies;
use crate::enemies::testutil::{add_player, quiet_world, test_values};
use crate::island::testutil::{add_pool_points, must_learn};
use crate::monarch::{learn_skill, skill_by_id};
use crate::state::{Enemy, World};
use crate::world::testutil::DT;
use crate::world::vals;

/// Go: `spellMult`: Wirkung von Arcane Power (+20 %) laut monarch.json.
fn spell_mult() -> f64 {
    1.0 + test_values().monarch.skills["arcanePower"].effect.spell_bonus
}

/// Go: `cooldownMult`: Wirkung von Elemental Mastery (−15 %) laut monarch.json.
fn cooldown_mult() -> f64 {
    1.0 + test_values().monarch.skills["elementalMastery"].effect.cooldown_bonus
}

fn near(a: f64, b: f64) -> bool {
    (a - b).abs() < 1e-6
}

/// Go: `lost` für einen Gegner (die Variante mit Welt und Index steht in `testutil`).
fn lost_of(e: &Enemy) -> f64 {
    e.max_hp - e.hp
}

#[test]
fn caster_fireball_und_meteor() {
    for (slot, radius, damage) in [(1, 3.0, 30.0), (4, 15.0, 200.0)] {
        let mut w = quiet_world();
        let pi = caster_player(&mut w);
        let px = w.players[pi].x;
        let target = tough_enemy(&mut w, px + 5.0);
        let tx = w.enemies[target].x;
        let (inside, out) = (tough_enemy(&mut w, tx + radius), tough_enemy(&mut w, tx + radius + 1.0));
        skills_tick(&mut w, pi, &skill_cmd(slot));
        let want = damage * spell_mult(); // mit Arcane Power: 36 bzw. 240
        let (lt, li, lo) = (
            lost_of(&w.enemies[target]),
            lost_of(&w.enemies[inside]),
            lost_of(&w.enemies[out]),
        );
        assert!(
            near(lt, want) && near(li, want) && lo == 0.0,
            "Slot {slot}: Ziel {lt}, {radius}u {li}, {}u {lo}",
            radius + 1.0
        );
    }
}

#[test]
fn caster_ice_wall() {
    let mut w = quiet_world();
    let pi = caster_player(&mut w);
    let px = w.players[pi].x;
    let target = tough_enemy(&mut w, px + 3.0);
    let tx = w.enemies[target].x;
    let (inside, out) = (tough_enemy(&mut w, tx + 3.0), tough_enemy(&mut w, tx + 4.0));
    skills_tick(&mut w, pi, &skill_cmd(2));
    assert!(
        w.enemies[target].slow == 0.5 && w.enemies[inside].slow_for == 5.0 && w.enemies[out].slow_for == 0.0,
        "Ice Wall: Ziel {}, 3u {} s, 4u {} s",
        w.enemies[target].slow,
        w.enemies[inside].slow_for,
        w.enemies[out].slow_for
    );
    w.players[pi].x = w.hub_x + 80.0; // Gegner laufen nur
    for i in 0..160 {
        let (x0, x1) = (w.enemies[inside].x, w.enemies[out].x);
        step_enemies(&mut w, None, DT);
        let ratio = (w.enemies[inside].x - x0) / (w.enemies[out].x - x1);
        assert!(
            !(i < 145 && (ratio - 0.5).abs() > 1e-9 || i > 151 && (ratio - 1.0).abs() > 1e-9),
            "Tick {i}: Geschwindigkeit verlangsamt/normal = {ratio}"
        );
    }
    let e = &w.enemies[inside];
    assert!(
        e.slow == 0.0 && e.slow_for == 0.0,
        "Verlangsamung nach 5 s: {} / {}",
        e.slow,
        e.slow_for
    );
}

#[test]
fn caster_lightning_storm() {
    let mut w = quiet_world();
    let pi = caster_player(&mut w);
    let px = w.players[pi].x;
    let target = tough_enemy(&mut w, px + 3.0);
    let tx = w.enemies[target].x;
    let (inside, out) = (tough_enemy(&mut w, tx + 5.0), tough_enemy(&mut w, tx + 6.0));
    skills_tick(&mut w, pi, &skill_cmd(3));
    assert!(
        w.storms.len() == 1 && w.storms[0].owner == w.players[pi].id,
        "Sturm: {:?}",
        w.storms
    );
    for _ in 0..75 {
        // 2,5 s
        step_storms(&mut w, DT);
    }
    let (li, lo) = (lost_of(&w.enemies[inside]), lost_of(&w.enemies[out]));
    assert!(
        near(li, 25.0 * spell_mult()) && lo == 0.0,
        "nach 2,5 s: 5u {li}, 6u {lo}"
    );
    for _ in 0..125 {
        // weit über 5 s hinaus
        step_storms(&mut w, DT);
    }
    let (lt, li, lo) = (
        lost_of(&w.enemies[target]),
        lost_of(&w.enemies[inside]),
        lost_of(&w.enemies[out]),
    );
    assert!(
        near(lt, 50.0 * spell_mult()) && near(li, 50.0 * spell_mult()) && lo == 0.0 && w.storms.is_empty(),
        "nach 5 s: Ziel {lt}, 5u {li}, 6u {lo}, Stürme {}",
        w.storms.len()
    );
}

/// `TestCasterAbklingzeitUndOhneZiel`: je Zauber (c) kein Gegner in Range: keine Wirkung, keine Abklingzeit; (b) zweiter
/// Einsatz in der Abklingzeit ohne Wirkung, nach `cooldown` wieder möglich.
#[test]
fn caster_abklingzeit_und_ohne_ziel() {
    for slot in 1..=4_usize {
        let mut w = quiet_world();
        let pi = caster_player(&mut w);
        let s = skill_by_id(&vals(&w), &w.players[pi].slots[slot - 1]).expect("Skill");
        let cd = s.cooldown * cooldown_mult(); // mit Elemental Mastery
        let px = w.players[pi].x;
        let far = tough_enemy(&mut w, px + s.effect.range + 0.5);
        let effect = |w: &World| lost_of(&w.enemies[far]) + w.enemies[far].slow_for + w.storms.len() as f64;
        skills_tick(&mut w, pi, &skill_cmd(slot as i64));
        assert!(
            effect(&w) == 0.0 && w.players[pi].cooldowns.is_empty(),
            "{} ohne Ziel: Wirkung {}, CD {:?}",
            s.id,
            effect(&w),
            w.players[pi].cooldowns
        );
        w.enemies[far].x = px + 1.0;
        skills_tick(&mut w, pi, &skill_cmd(slot as i64));
        let first = effect(&w);
        assert!(
            first != 0.0 && w.players[pi].cooldowns[slot - 1] == cd,
            "{}: Wirkung {first}, CD {:?} (erwartet {cd})",
            s.id,
            w.players[pi].cooldowns
        );
        w.enemies[far].slow_for = 0.0;
        w.storms.clear();
        let before = effect(&w);
        skills_tick(&mut w, pi, &skill_cmd(slot as i64));
        assert_eq!(effect(&w), before, "{}: zweiter Einsatz in der Abklingzeit wirkt", s.id);
        step_idle(&mut w, pi, (cd / DT).ceil() as usize);
        skills_tick(&mut w, pi, &skill_cmd(slot as i64));
        assert_eq!(
            w.players[pi].cooldowns[slot - 1],
            cd,
            "{}: nach {cd} s kein neuer Einsatz, CD {:?}",
            s.id,
            w.players[pi].cooldowns
        );
    }
}

#[test]
fn caster_gating_meteor() {
    let mut w = quiet_world();
    let pi = add_player(&mut w);
    let v = vals(&w);
    add_pool_points(&mut w, 7);
    must_learn(&mut w, pi, &MAGE_SKILLS[..5]); // fünf Skills der Linie, Tier 4 verlangt sechs
    assert!(
        learn_skill(&mut w, &v, pi, "meteor").is_err(),
        "Meteor ohne sechs Skills der Linie gelernt"
    );
    w.players[pi].slots.push("meteor".into()); // erzwungen im Slot, aber nicht gelernt
    let px = w.players[pi].x;
    let ei = tough_enemy(&mut w, px + 1.0);
    skills_tick(&mut w, pi, &skill_cmd(4));
    assert!(
        lost_of(&w.enemies[ei]) == 0.0 && w.players[pi].cooldowns.is_empty(),
        "Meteor ohne Lernen: Schaden {}, CD {:?}",
        lost_of(&w.enemies[ei]),
        w.players[pi].cooldowns
    );
}

/// Zwei Zauberer, ein Gegnerfeld: Slow wirkt vom Besitzer unabhängig und nur im Radius um das Ziel.
#[test]
fn own_slow_wirkt_im_radius_um_das_ziel_fuer_zwei_zauberer() {
    let mut w = quiet_world();
    let (p0, p1) = (caster_player(&mut w), add_player(&mut w));
    let (x0, x1) = (w.hub_x + 40.0, w.hub_x + 120.0);
    (w.players[p0].x, w.players[p1].x) = (x0, x1);
    let a = tough_enemy(&mut w, x0 + 3.0);
    let a_in = tough_enemy(&mut w, x0 + 6.0);
    let a_out = tough_enemy(&mut w, x0 + 7.0);
    let b = tough_enemy(&mut w, x1 + 2.0);
    let e = k3c_data::monarch::SkillEffect {
        r#type: "slow".into(),
        radius: 3.0,
        range: 10.0,
        factor: 0.5,
        duration: 5.0,
        ..Default::default()
    };
    assert!(cast_slow(&mut w, p0, &e));
    assert!(w.enemies[a].slow == 0.5 && w.enemies[a_in].slow_for == 5.0 && w.enemies[a_out].slow_for == 0.0);
    assert_eq!(w.enemies[b].slow_for, 0.0, "der zweite Zauberer wurde nicht gewirkt");
    assert!(cast_slow(&mut w, p1, &e));
    assert!(w.enemies[b].slow == 0.5 && w.enemies[b].slow_for == 5.0);
}

#[test]
fn own_ohne_ziel_keine_wirkung_und_sturm_wird_angelegt_und_endet() {
    let mut w = quiet_world();
    let pi = caster_player(&mut w);
    let e = k3c_data::monarch::SkillEffect {
        r#type: "damageOverTime".into(),
        radius: 5.0,
        range: 10.0,
        per_second: 25.0,
        duration: 0.5,
        ..Default::default()
    };
    assert!(!cast_area_damage(&mut w, pi, &e) && !cast_slow(&mut w, pi, &e) && !cast_damage_over_time(&mut w, pi, &e));
    assert!(w.storms.is_empty(), "ohne Ziel kein Sturm");
    let px = w.players[pi].x;
    tough_enemy(&mut w, px + 20.0); // außerhalb der Range
    assert!(!cast_damage_over_time(&mut w, pi, &e));
    let far = tough_enemy(&mut w, px + 4.0);
    assert!(cast_damage_over_time(&mut w, pi, &e));
    let owner = w.players[pi].id;
    assert!(w.storms.len() == 1 && w.storms[0].owner == owner && w.storms[0].x == w.enemies[far].x);
    // der Sturm läuft aus; ohne Gegner im Radius ist keine Meldung nötig (die Gegner stehen außerhalb)
    w.enemies.clear();
    for _ in 0..16 {
        step_storms(&mut w, DT);
    }
    assert!(w.storms.is_empty(), "Sturm nach seiner Dauer entfernt: {:?}", w.storms);
}
