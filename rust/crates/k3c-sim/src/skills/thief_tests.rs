//! Dieb-Linie (Go: `skills_thief_test.go`): Backstab und Smoke Bomb laut monarch.json (B-577/AC-02). Wo ein Skill wirkt, meldet
//! er `skillCast` über `emit` und `unit_x` (RU19); ein Treffer meldet `hit_event`, `step` rechnet `pure`: diese Tests warten
//! darauf. Die Tests ab `own_` haben kein Go-Gegenstück und prüfen Blickrichtung und Stealth ohne diese Funktionen.

use super::testutil::*;
use super::thief::{behind_enemy, cast_backstab, cast_stealth, enemy_facing};
use crate::enemies::target::choose_target;
use crate::enemies::testutil::{PROBE_GOLD, add_player, probe, quiet_world, spawn};
use crate::island::testutil::{add_pool_points, must_learn};
use crate::monarch::{apply_preset, step_attack};
use crate::state::common::{apply_damage_by, sign};
use crate::state::{PlayerCommand, World};
use crate::world::testutil::DT;
use crate::world::{step, vals};

use k3c_data::monarch::SkillEffect;

/// Go: `thiefPlayer`: ein Spieler mit beiden aktiven Dieb-Skills (Slots: backstab, smokeBomb), rechts der Burg.
fn thief_player(w: &mut World) -> usize {
    let pi = add_player(w);
    add_pool_points(w, 2);
    must_learn(w, pi, &["backstab", "smokeBomb"]);
    w.players[pi].x = w.hub_x + 20.0;
    pi
}

#[test]
#[ignore = "wartet auf RU19 (emit, unit_x, hit_event)"]
fn thief_backstab_vorn_und_hinten() {
    // (Name, dx, fliehend, erwarteter Schaden)
    let cases = [
        ("vorn (Gegner blickt zur Burg)", 1.0, false, 40.0),
        ("hinten (Monarch zwischen Gegner und Portal)", -1.0, false, 80.0),
        ("fliehend blickt zum Portal", 1.0, true, 80.0),
    ];
    for (name, dx, fleeing, want) in cases {
        let mut w = quiet_world();
        let pi = thief_player(&mut w);
        let px = w.players[pi].x;
        let ei = tough_enemy(&mut w, px + dx);
        (w.enemies[ei].home_x, w.enemies[ei].fleeing) = (w.hub_x + 60.0, fleeing);
        skills_tick(&mut w, pi, &skill_cmd(1));
        assert!(
            lost(&w, ei) == want && w.players[pi].cooldowns[0] == 12.0,
            "{name}: Schaden {} (erwartet {want}), CD {:?}",
            lost(&w, ei),
            w.players[pi].cooldowns
        );
    }
}

#[test]
#[ignore = "wartet auf RU19 (emit, unit_x: castFailed beim ersten Druck)"]
fn thief_backstab_ohne_ziel_keine_abklingzeit() {
    let mut w = quiet_world();
    let pi = thief_player(&mut w);
    let px = w.players[pi].x;
    let far = tough_enemy(&mut w, px + 2.5);
    skills_tick(&mut w, pi, &skill_cmd(1));
    assert!(
        lost(&w, far) == 0.0 && w.players[pi].cooldowns.is_empty(),
        "Backstab ohne Gegner in 2u: Schaden {}, CD {:?}",
        lost(&w, far),
        w.players[pi].cooldowns
    );
}

/// Unsichtbar: kein Ziel (auch nicht verspottet), kein Schaden, nach 5 s wieder sichtbar; Abklingzeit 25 s.
#[test]
#[ignore = "wartet auf RU19 (emit, unit_x, hit_event)"]
fn thief_smoke_bomb_unsichtbar() {
    let mut w = quiet_world();
    let pi = thief_player(&mut w);
    let other = add_player(&mut w);
    let px = w.players[pi].x;
    w.players[other].x = px + 1.4;
    let ei = spawn(&mut w, "skeleton", px + 1.0);
    let dir = sign(w.hub_x - w.enemies[ei].x);
    let (pid, oid) = (w.players[pi].id, w.players[other].id);
    (w.enemies[ei].taunt_id, w.enemies[ei].taunt_for) = (pid, 10.0);
    skills_tick(&mut w, pi, &skill_cmd(2));
    let p = &w.players[pi];
    assert!(
        p.stealth_for == 5.0 && p.cooldowns[1] == 25.0,
        "Smoke Bomb: StealthFor {}, CD {:?}",
        p.stealth_for,
        p.cooldowns
    );
    let v = vals(&w);
    let c = choose_target(&w, &v, ei, dir, None);
    assert!(
        c.as_ref().is_some_and(|c| c.id == oid),
        "Gegner zielt auf {c:?}, erwartet den sichtbaren Spieler {oid}"
    );
    probe(&mut w, &[pi]);
    apply_damage_by(&mut w, pid, 50.0, "skeleton");
    assert_eq!(w.players[pi].gold, PROBE_GOLD, "Unsichtbarer verliert Gold");
    step_idle(&mut w, pi, 151); // 151 Ticks > 5 s
    assert_eq!(w.players[pi].stealth_for, 0.0, "nach 5 s noch unsichtbar");
    let c = choose_target(&w, &v, ei, dir, None);
    assert!(
        c.as_ref().is_some_and(|c| c.id == pid),
        "nach 5 s: Ziel {c:?}, erwartet den Dieb {pid} (näher, verspottet)"
    );
}

/// Ein eigener Schlag oder Backstab beendet die Unsichtbarkeit vorzeitig (Annahme B-577).
#[test]
#[ignore = "wartet auf RU19 (emit, unit_x, hit_event)"]
fn thief_smoke_bomb_endet_bei_angriff() {
    let mut w = quiet_world();
    let pi = thief_player(&mut w);
    let px = w.players[pi].x;
    let en = tough_enemy(&mut w, px + 1.0);
    skills_tick(&mut w, pi, &skill_cmd(2));
    let v = vals(&w);
    let attack = PlayerCommand {
        attack: true,
        ..PlayerCommand::default()
    };
    step_attack(&mut w, &v, pi, &attack, DT);
    assert!(
        w.players[pi].stealth_for == 0.0 && lost(&w, en) != 0.0,
        "Schlag aus der Unsichtbarkeit: StealthFor {}, Schaden {}",
        w.players[pi].stealth_for,
        lost(&w, en)
    );
    w.players[pi].cooldowns[1] = 0.0;
    skills_tick(&mut w, pi, &skill_cmd(2));
    skills_tick(&mut w, pi, &skill_cmd(1));
    let p = &w.players[pi];
    assert!(
        p.stealth_for == 0.0 && p.cooldowns[0] == 12.0,
        "Backstab aus der Unsichtbarkeit: StealthFor {}, CD {:?}",
        p.stealth_for,
        p.cooldowns
    );
}

/// Zwei Diebe im selben Tick: Wirkung und Abklingzeit je Spieler getrennt.
#[test]
#[ignore = "wartet auf RU19 (pure, emit, unit_x, hit_event)"]
fn thief_zwei_spieler_im_selben_tick() {
    let mut w = quiet_world();
    let (a, b) = (add_player(&mut w), add_player(&mut w));
    add_pool_points(&mut w, 2);
    must_learn(&mut w, a, &["backstab", "smokeBomb"]);
    must_learn(&mut w, b, &["backstab", "smokeBomb"]);
    (w.players[a].x, w.players[b].x) = (w.hub_x + 40.0, w.hub_x + 80.0);
    let bx = w.players[b].x;
    let near_b = tough_enemy(&mut w, bx + 1.0);
    let cmd = |s| PlayerCommand {
        skill: s,
        ..PlayerCommand::default()
    };
    step(&mut w, &[cmd(2), cmd(1)], None, DT);
    let (pa, pb) = (&w.players[a], &w.players[b]);
    assert!(
        pa.stealth_for > 0.0 && pb.stealth_for == 0.0 && lost(&w, near_b) == 40.0,
        "Stealth A {}, B {}, Backstab-Schaden {}",
        pa.stealth_for,
        pb.stealth_for,
        lost(&w, near_b)
    );
    assert!(
        pa.cooldowns == [0.0, 25.0, 0.0, 0.0] && pb.cooldowns == [12.0, 0.0, 0.0, 0.0],
        "Abklingzeiten nicht je Spieler: A {:?}, B {:?}",
        pa.cooldowns,
        pb.cooldowns
    );
}

/// Das Preset Dieb lernt beide Skills (B-577/AC-01).
#[test]
fn thief_preset_lernt_beide_skills() {
    let mut w = quiet_world();
    let pi = add_player(&mut w);
    w.players[pi].skills.clear();
    w.players[pi].slots.clear();
    add_pool_points(&mut w, 2);
    apply_preset(&mut w, pi, "thief").expect("Preset Dieb");
    let p = &w.players[pi];
    let want = ["backstab", "smokeBomb"];
    assert!(
        p.skills == want && p.slots == want,
        "Preset Dieb: Skills {:?}, Slots {:?}",
        p.skills,
        p.slots
    );
}

/// Blickrichtung: Fliehende blicken zu ihrem Portal, alle anderen zur Burg; dieselbe Stelle zählt als vorn.
#[test]
fn own_blickrichtung_und_ruecken_fuer_zwei_diebe() {
    let mut w = quiet_world();
    let (a, b) = (add_player(&mut w), add_player(&mut w));
    let hub = w.hub_x;
    let ei = tough_enemy(&mut w, hub + 50.0);
    assert_eq!(enemy_facing(&w, &w.enemies[ei]), -1.0, "Marsch zur Burg: nach links");
    assert!(behind_enemy(&w, &w.enemies[ei], hub + 51.0) && !behind_enemy(&w, &w.enemies[ei], hub + 49.0));
    assert!(
        !behind_enemy(&w, &w.enemies[ei], hub + 50.0),
        "dieselbe Stelle zählt als vorn"
    );
    (w.enemies[ei].fleeing, w.enemies[ei].home_x) = (true, hub + 90.0);
    assert_eq!(enemy_facing(&w, &w.enemies[ei]), 1.0, "Fliehende blicken zum Portal");
    // Backstab: Dieb A vorn, Dieb B hinten; der Rücken-Schaden hängt am Wirkenden, nicht am Gegner.
    w.enemies[ei].fleeing = false;
    (w.players[a].x, w.players[b].x) = (hub + 49.0, hub + 51.0);
    assert!(!behind_enemy(&w, &w.enemies[ei], w.players[a].x) && behind_enemy(&w, &w.enemies[ei], w.players[b].x));
    let e = SkillEffect {
        range: 2.0,
        damage: 40.0,
        back_damage: 80.0,
        ..SkillEffect::default()
    };
    let (far_id, _) = (w.enemies[ei].id, ());
    w.enemies[ei].x = hub + 1000.0;
    assert!(
        !cast_backstab(&mut w, a, &e),
        "ohne Gegner in Range keine Wirkung (id {far_id})"
    );
}

#[test]
fn own_stealth_nimmt_den_groesseren_wert() {
    let mut w = quiet_world();
    let (a, b) = (add_player(&mut w), add_player(&mut w));
    w.players[a].stealth_for = 8.0;
    let e = SkillEffect {
        duration: 5.0,
        ..SkillEffect::default()
    };
    assert!(cast_stealth(&mut w, a, &e) && cast_stealth(&mut w, b, &e));
    assert!(w.players[a].stealth_for == 8.0 && w.players[b].stealth_for == 5.0);
}
