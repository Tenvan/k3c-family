//! Heiler-Linie (Go: `skills_healer_test.go`): Wirkung, Fläche, Reichweite und Abklingzeit laut monarch.json (S1.2b, B-119).
//! Jeder Skill meldet `skillCast` über `emit` und `unit_x` (RU19), ein Treffer `hit_event`, `step` rechnet `pure`: diese Tests
//! warten darauf. Die Tests ab `own_` haben kein Go-Gegenstück und prüfen die Wirkungen ohne diese Funktionen.

use super::healer::{cast_group_heal, cast_heal, cast_revive_crown};
use super::tank::cast_shield;
use super::testutil::*;
use crate::crown::testutil::fell;
use crate::enemies::testutil::{PROBE_GOLD, add_player, probe, quiet_world, test_values};
use crate::island::testutil::{add_pool_points, must_learn};
use crate::monarch::skill_by_id;
use crate::state::common::apply_damage;
use crate::state::{PlayerCommand, World};
use crate::world::testutil::DT;
use crate::world::{step, vals};

use k3c_data::monarch::SkillEffect;

/// Go: `healerSkills`: die sieben Skills für alle vier aktiven Heiler-Skills (Slots: heal, groupHeal, divineShield,
/// resurrection).
const HEALER_SKILLS: [&str; 7] = [
    "heal",
    "groupHeal",
    "healingAura",
    "blessings",
    "divineShield",
    "holyGround",
    "resurrection",
];

fn healer_player(w: &mut World) -> usize {
    let pi = add_player(w);
    add_pool_points(w, 7);
    must_learn(w, pi, &HEALER_SKILLS);
    pi
}

fn effect(range: f64, radius: f64, amount: f64) -> SkillEffect {
    SkillEffect {
        range,
        radius,
        amount,
        ..SkillEffect::default()
    }
}

#[test]
fn healer_heal() {
    let mut w = quiet_world();
    let pi = healer_player(&mut w);
    let px = w.players[pi].x;
    // far: schwächer, aber außer Reichweite
    let (weak, weaker, far) = (
        troop_at(&mut w, px + 6.0, 3.0),
        troop_at(&mut w, px - 5.0, 2.0),
        troop_at(&mut w, px + 7.0, 1.0),
    );
    skills_tick(&mut w, pi, &skill_cmd(1));
    let t = &w.troops;
    assert!(
        t[weaker].hp == 10.0 && t[weak].hp == 3.0 && t[far].hp == 1.0 && w.players[pi].cooldowns[0] == 10.0,
        "Heal: schwächste Truppe {}, andere {}, außer Reichweite {}, CD {:?}",
        t[weaker].hp,
        t[weak].hp,
        t[far].hp,
        w.players[pi].cooldowns
    );
    step_idle(&mut w, pi, 301); // 10 s
    w.troops[weak].hp = 8.0;
    skills_tick(&mut w, pi, &skill_cmd(1));
    assert_eq!(w.troops[weak].hp, w.troops[weak].max_hp, "Heal über MaxHP");
}

#[test]
fn healer_group_heal() {
    let mut w = quiet_world();
    let pi = healer_player(&mut w);
    let px = w.players[pi].x;
    let (inside, out) = (troop_at(&mut w, px - 10.0, 2.0), troop_at(&mut w, px + 11.0, 2.0));
    skills_tick(&mut w, pi, &skill_cmd(2));
    assert!(
        w.troops[inside].hp == 10.0 && w.troops[out].hp == 2.0 && w.players[pi].cooldowns[1] == 30.0,
        "Group Heal: Truppen {}/{}, CD {:?}",
        w.troops[inside].hp,
        w.troops[out].hp,
        w.players[pi].cooldowns
    );
}

/// Divine Shield fängt Treffer ohne Goldverlust ab, bis die Schild-Punkte aufgebraucht sind (kampf.md § 5).
#[test]
fn healer_divine_shield() {
    let mut w = quiet_world();
    let pi = healer_player(&mut w);
    probe(&mut w, &[pi]);
    skills_tick(&mut w, pi, &skill_cmd(3));
    let p = &w.players[pi];
    assert!(
        p.shield == 100.0 && p.shield_for == 10.0 && p.cooldowns[2] == 20.0,
        "Divine Shield: {} für {} s, CD {:?}",
        p.shield,
        p.shield_for,
        p.cooldowns
    );
    let id = p.id;
    apply_damage(&mut w, id, 60.0);
    apply_damage(&mut w, id, 60.0); // zweiter Treffer: Schild fast leer, auch der Rest wird abgefangen
    let p = &w.players[pi];
    assert!(
        p.shield == 0.0 && p.gold == PROBE_GOLD,
        "Schild {}, Gold {}",
        p.shield,
        p.gold
    );
    apply_damage(&mut w, id, 15.0); // Schild leer: eine Münze
    let want = PROBE_GOLD - test_values().monarch.hit_coins;
    assert_eq!(w.players[pi].gold, want, "Gold nach Treffer ohne Schild");
}

/// Resurrection bringt gefallene Kronen im Radius sofort zu ihrem Monarchen zurück (Umdeutung, kampf.md § 5).
#[test]
fn healer_resurrection() {
    let mut w = quiet_world();
    let pi = healer_player(&mut w);
    let (q, r) = (add_player(&mut w), add_player(&mut w));
    skills_tick(&mut w, pi, &skill_cmd(4)); // (c) keine Krone im Radius
    let cd = &w.players[pi].cooldowns;
    assert!(
        cd.is_empty() || cd[3] == 0.0,
        "Resurrection ohne gefallene Krone: CD {cd:?}"
    );
    let px = w.players[pi].x;
    (w.players[q].x, w.players[r].x) = (px + 15.0, px + 16.0);
    fell(&mut w, q);
    fell(&mut w, r);
    skills_tick(&mut w, pi, &skill_cmd(4));
    assert!(
        w.players[q].crown.is_none() && w.players[r].crown.is_some() && w.players[pi].cooldowns[3] == 180.0,
        "Resurrection: Krone Q {:?}, R (außer Reichweite) {:?}, CD {:?}",
        w.players[q].crown,
        w.players[r].crown,
        w.players[pi].cooldowns
    );
}

/// `TestHealerAbklingzeit` (b): zweiter Einsatz in der Abklingzeit ohne Wirkung, nach `cooldown` wieder möglich.
#[test]
fn healer_abklingzeit() {
    for slot in 1..=4_usize {
        let mut w = quiet_world();
        let pi = healer_player(&mut w);
        let q = add_player(&mut w);
        let px = w.players[pi].x;
        w.players[q].x = px + 1.0;
        let tr = troop_at(&mut w, px + 1.0, 1.0);
        let s = skill_by_id(&vals(&w), &w.players[pi].slots[slot - 1]).expect("Skill");
        let reset = |w: &mut World| {
            (w.players[pi].shield, w.players[pi].shield_for, w.troops[tr].hp) = (0.0, 0.0, 1.0);
            fell(w, q);
        };
        // ohne Wirkung: 1
        let effect =
            |w: &World| w.players[pi].shield + w.troops[tr].hp + if w.players[q].crown.is_none() { 1.0 } else { 0.0 };
        reset(&mut w);
        skills_tick(&mut w, pi, &skill_cmd(slot as i64));
        assert!(
            effect(&w) != 1.0 && w.players[pi].cooldowns[slot - 1] == s.cooldown,
            "{}: Wirkung {}, CD {:?}",
            s.id,
            effect(&w),
            w.players[pi].cooldowns
        );
        reset(&mut w);
        skills_tick(&mut w, pi, &skill_cmd(slot as i64));
        assert_eq!(effect(&w), 1.0, "{}: zweiter Einsatz in der Abklingzeit wirkt", s.id);
        step_idle(&mut w, pi, (s.cooldown / DT).ceil() as usize);
        skills_tick(&mut w, pi, &skill_cmd(slot as i64));
        assert!(
            effect(&w) != 1.0 && w.players[pi].cooldowns[slot - 1] == s.cooldown,
            "{}: nach {} s kein neuer Einsatz, CD {:?}",
            s.id,
            s.cooldown,
            w.players[pi].cooldowns
        );
    }
}

/// `TestHealerHeiltZauberer` (d): Zauberer (Spieler 0) und Heiler (Spieler 1) im selben Kampf über `step`. Mit Passiven:
/// Fireball 30 × (1 + Arcane Power 0,2 + Blessings 0,1) = 39; die Heiler-Linie heilt Truppen, nicht den Zauberer.
#[test]
fn healer_heilt_zauberer() {
    let mut w = quiet_world();
    let (mage, healer) = (add_player(&mut w), add_player(&mut w));
    add_pool_points(&mut w, 7);
    must_learn(&mut w, mage, &MAGE_SKILLS);
    must_learn(&mut w, healer, &HEALER_SKILLS);
    w.players[mage].x = w.hub_x + 40.0;
    w.players[healer].x = w.players[mage].x + 2.0;
    let mx = w.players[mage].x;
    let ei = crate::enemies::spawn_scaled(&mut w, "skeleton", mx + 6.0, 10.0, 1.0, None);
    let tr = troop_at(&mut w, mx, 2.0);
    let cmd = |s| PlayerCommand {
        skill: s,
        ..PlayerCommand::default()
    };
    step(&mut w, &[cmd(1), cmd(1)], None, DT);
    let (e, t) = (&w.enemies[ei], &w.troops[tr]);
    assert!(
        ((e.max_hp - e.hp) - 39.0).abs() < 1e-6 && t.hp == t.max_hp,
        "Fireball {}, Truppe {} (Heal des Heilers)",
        e.max_hp - e.hp,
        t.hp
    );
    let cooldown_mult = 1.0 + test_values().monarch.skills["elementalMastery"].effect.cooldown_bonus;
    let (m, h) = (&w.players[mage], &w.players[healer]);
    assert!(
        m.cooldowns[0] == 8.0 * cooldown_mult && h.cooldowns[0] == 10.0 && m.cooldowns[1] == 0.0,
        "Abklingzeiten Zauberer {:?}, Heiler {:?}",
        m.cooldowns,
        h.cooldowns
    );
}

/// Zwei Heiler am selben Ort: Heal nimmt die Truppe mit dem kleinsten Anteil, bei Gleichstand die kleinere ID; Group Heal
/// heilt jeden Besitzer unabhängig im eigenen Radius und nie über `max_hp`.
#[test]
fn own_heal_waehlt_kleinsten_anteil_dann_kleinere_id_fuer_zwei_heiler() {
    let mut w = quiet_world();
    let (a, b) = (add_player(&mut w), add_player(&mut w));
    (w.players[a].x, w.players[b].x) = (100.0, 300.0);
    let (t1, t2, t3) = (
        troop_at(&mut w, 101.0, 5.0),
        troop_at(&mut w, 102.0, 5.0),
        troop_at(&mut w, 99.0, 9.0),
    );
    let far = troop_at(&mut w, 301.0, 1.0);
    let e = effect(10.0, 5.0, 4.0);
    assert!(cast_heal(&mut w, a, &e));
    assert!(
        w.troops[t1].hp == 9.0 && w.troops[t2].hp == 5.0,
        "Gleichstand: die kleinere ID: {}/{}",
        w.troops[t1].hp,
        w.troops[t2].hp
    );
    assert_eq!(w.troops[far].hp, 1.0, "der zweite Heiler hat noch nicht gewirkt");
    assert!(cast_heal(&mut w, b, &e));
    assert_eq!(w.troops[far].hp, 5.0);
    assert!(cast_group_heal(&mut w, a, &e) && w.troops[t2].hp == 9.0 && w.troops[t3].hp == 10.0);
    assert_eq!(w.troops[far].hp, 5.0, "Group Heal wirkt nur im Radius des Besitzers");
    for t in &mut w.troops {
        t.hp = t.max_hp;
    }
    assert!(!cast_heal(&mut w, a, &e), "niemand verletzt: keine Wirkung");
    w.troops[t1].hp = 0.0; // gefallen
    assert!(!cast_heal(&mut w, a, &e), "Gefallene werden nicht geheilt");
}

#[test]
fn own_shield_nur_groesserer_ersetzt_und_resurrection_ohne_krone_wirkt_nicht() {
    let mut w = quiet_world();
    let (a, b) = (add_player(&mut w), add_player(&mut w));
    let e = SkillEffect {
        hp: 100.0,
        duration: 10.0,
        ..SkillEffect::default()
    };
    assert!(cast_shield(&mut w, a, &e));
    assert!(w.players[a].shield == 100.0 && w.players[a].shield_for == 10.0 && w.players[b].shield == 0.0);
    let weaker = SkillEffect {
        hp: 50.0,
        duration: 99.0,
        ..SkillEffect::default()
    };
    assert!(cast_shield(&mut w, a, &weaker), "die Abklingzeit startet in jedem Fall");
    assert!(
        w.players[a].shield == 100.0 && w.players[a].shield_for == 10.0,
        "ein kleinerer Schild ersetzt nicht"
    );
    let r = effect(0.0, 20.0, 0.0);
    assert!(!cast_revive_crown(&mut w, a, &r), "ohne Krone im Radius keine Wirkung");
    fell(&mut w, b);
    let far_x = w.players[a].x + 30.0;
    w.players[b].x = far_x;
    if let Some(c) = w.players[b].crown.as_mut() {
        c.x = far_x;
    }
    assert!(!cast_revive_crown(&mut w, a, &r), "Krone außer Reichweite");
    assert!(w.players[b].crown.is_some());
}
