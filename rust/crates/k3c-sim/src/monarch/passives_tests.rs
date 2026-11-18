//! Passive Skills mit 2 Spielern (Go: `monarch_passives_test.go`, S1.2c, B-119/AC-03): erwartete Werte aus monarch.json
//! (vorläufige Startwerte). Tests über `step` (`pure`), `hit_event` bei Treffern und `emit` beim Wirken (RU19) tragen `ignore`.
//! Die Tests ab `own_` haben kein Go-Gegenstück und prüfen die Rechnung der Passive ohne diese Funktionen.

use super::passives::{cooldown_mult_of, damage_mult_of, each_passive, speed_of, troop_damage};
use super::step_attack;
use crate::enemies::target::{Target, TargetKind, attack};
use crate::enemies::testutil::{PROBE_GOLD, add_player, probe, quiet_world};
use crate::island::testutil::{add_pool_points, must_learn};
use crate::skills::testutil::{lost, skill_cmd, skills_tick, tough_enemy, troop_at};
use crate::state::common::apply_damage;
use crate::state::{PlayerCommand, World};
use crate::world::testutil::DT;
use crate::world::{step, vals};

use k3c_data::monarch::SkillEffect;

/// Go: `passiveWorld`: ruhige Welt mit zwei Spielern weit weg von der Burg, je 3 Pool-Punkte.
pub(super) fn passive_world() -> (World, usize, usize) {
    let mut w = quiet_world();
    let (p0, p1) = (add_player(&mut w), add_player(&mut w));
    add_pool_points(&mut w, 3);
    let x = w.hub_x + 40.0;
    w.players[p0].x = x;
    w.players[p1].x = x;
    (w, p0, p1)
}

fn step_ticks(w: &mut World, ticks: usize) {
    for _ in 0..ticks {
        step(w, &[], None, DT);
    }
}

pub(super) fn effect_of(w: &World, id: &str) -> SkillEffect {
    vals(w).monarch.skills[id].effect.clone()
}

fn near(a: f64, b: f64) -> bool {
    (a - b).abs() < 1e-6
}

/// Ein Zahlpaar je Spieler für die Basis-Prüfung: Tempo, Zauber-Faktor, Abklingzeit-Faktor.
fn basis_of(w: &World, pi: usize) -> (f64, f64, f64) {
    let v = vals(w);
    (
        speed_of(w, &v, pi),
        damage_mult_of(w, &v, pi, true),
        cooldown_mult_of(w, &v, pi),
    )
}

/// (a) Armor Aura von Spieler 0 senkt den Schaden an Truppen im Radius 8, nicht außerhalb (Monarchen haben keine Verteidigung
/// mehr).
#[test]
fn passives_armor_aura() {
    let (mut w, p0, _) = passive_world();
    must_learn(&mut w, p0, &["taunt", "shieldBash", "armorAura"]);
    let e = effect_of(&w, "armorAura");
    let (bonus, aura) = (e.defense, e.aura);
    let px = w.players[p0].x;
    let (inside, out) = (
        troop_at(&mut w, px + aura, 100.0),
        troop_at(&mut w, px - aura - 0.5, 100.0),
    );
    let (iid, oid) = (w.troops[inside].id, w.troops[out].id);
    apply_damage(&mut w, iid, 17.0);
    apply_damage(&mut w, oid, 17.0);
    assert!(
        w.troops[inside].hp == 100.0 - (17.0 - bonus) && w.troops[out].hp == 100.0 - 17.0,
        "Schaden in der Aura: HP {}, außerhalb {}",
        w.troops[inside].hp,
        w.troops[out].hp
    );
}

/// (b) Healing Aura: Spieler 1 (Heiler) heilt Truppen im Radius 8 mit 2 HP/s, Monarchen nicht (kampf.md § 5).
#[test]
fn passives_healing_aura() {
    let (mut w, p0, p1) = passive_world();
    must_learn(&mut w, p1, &["heal", "groupHeal", "healingAura"]);
    w.players[p0].x = w.players[p1].x + 8.0;
    let x1 = w.players[p1].x;
    let (inside, out) = (troop_at(&mut w, x1 - 8.0, 2.0), troop_at(&mut w, x1 + 9.0, 2.0));
    step_ticks(&mut w, 1); // Truppen laufen danach los: nur der erste Tick zählt für sie
    assert!(
        w.troops[inside].hp == 2.0 + 2.0 * DT && w.troops[out].hp == 2.0,
        "Healing Aura: Truppe 8u {}, 9u {}",
        w.troops[inside].hp,
        w.troops[out].hp
    );
}

/// (c) Thick Skin und Regeneration: Werte des Besitzers, Spieler 1 bleibt unverändert.
#[test]
fn passives_tank_besitzer() {
    let (mut w, p0, p1) = passive_world();
    add_pool_points(&mut w, 2);
    must_learn(&mut w, p0, &["taunt", "shieldBash", "thickSkin", "regeneration"]);
    probe(&mut w, &[p0, p1]);
    let every = effect_of(&w, "thickSkin").free_hit_every;
    let hit_coins = vals(&w).monarch.hit_coins;
    let (id0, id1) = (w.players[p0].id, w.players[p1].id);
    for _ in 0..every {
        apply_damage(&mut w, id0, 5.0);
        apply_damage(&mut w, id1, 5.0);
    }
    assert!(
        w.players[p0].gold == PROBE_GOLD - (every - 1) * hit_coins
            && w.players[p1].gold == PROBE_GOLD - every * hit_coins,
        "Thick Skin: P0 Gold {} (jeder {every}. Treffer gratis), P1 {}",
        w.players[p0].gold,
        w.players[p1].gold
    );
    assert_eq!(w.cycle.phase, "day", "Phase {:?}, erwartet Tag", w.cycle.phase);
    (w.players[p0].gold, w.players[p1].gold) = (10, 10);
    w.coins.clear();
    step_ticks(&mut w, (5.0 / DT) as usize + 2); // 5 s × 0,2 Münzen/s
    assert!(
        w.players[p0].gold == 11 && w.players[p1].gold == 10,
        "Regeneration (Gold-Rinnsal): P0 {}, P1 {}",
        w.players[p0].gold,
        w.players[p1].gold
    );
}

/// (c) Arcane Power (+20 % Zauberschaden) und Elemental Mastery (−15 % Abklingzeit).
#[test]
fn passives_zauberer() {
    let (mut w, p0, p1) = passive_world();
    must_learn(&mut w, p0, &["fireball", "iceWall", "arcanePower"]);
    must_learn(&mut w, p1, &["fireball", "iceWall", "elementalMastery"]);
    let px = w.players[p0].x;
    let e = tough_enemy(&mut w, px + 5.0);
    skills_tick(&mut w, p0, &skill_cmd(1));
    assert!(
        near(lost(&w, e), 36.0) && w.players[p0].cooldowns[0] == 8.0,
        "Arcane Power: Schaden {}, CD {:?}",
        lost(&w, e),
        w.players[p0].cooldowns
    );
    skills_tick(&mut w, p1, &skill_cmd(1));
    assert!(
        near(lost(&w, e), 66.0) && near(w.players[p1].cooldowns[0], 6.8),
        "Elemental Mastery: Schaden {}, CD {:?}",
        lost(&w, e),
        w.players[p1].cooldowns
    );
}

/// (c) Blessings: Tempo und Schlagschaden +10 % für Monarchen im Radius 8.
#[test]
fn passives_blessings() {
    let (mut w, p0, p1) = passive_world();
    must_learn(&mut w, p1, &["heal", "groupHeal", "blessings"]);
    w.players[p0].x = w.players[p1].x - 8.0;
    let px = w.players[p0].x;
    let e = tough_enemy(&mut w, px - 1.0);
    let v = vals(&w);
    let cmd = PlayerCommand {
        attack: true,
        ..PlayerCommand::default()
    };
    step_attack(&mut w, &v, p0, &cmd, DT);
    assert!(
        near(speed_of(&w, &v, p0), 5.5) && near(lost(&w, e), 11.0),
        "Blessings in 8u: Tempo {}, Schlag {}",
        speed_of(&w, &v, p0),
        lost(&w, e)
    );
    w.players[p0].x = w.players[p1].x - 9.0;
    assert!(
        speed_of(&w, &v, p0) == v.monarch.base.speed && damage_mult_of(&w, &v, p0, true) == 1.0,
        "Blessings in 9u: Tempo {}, Faktor {}",
        speed_of(&w, &v, p0),
        damage_mult_of(&w, &v, p0, true)
    );
}

/// (d) Guardian: Truppen im Radius 5 um den Besitzer nehmen 20 % weniger Schaden.
#[test]
fn passives_guardian() {
    let (mut w, p0, _) = passive_world();
    must_learn(&mut w, p0, &["taunt", "shieldBash", "guardian"]);
    let px = w.players[p0].x;
    let (inside, out) = (troop_at(&mut w, px + 5.0, 10.0), troop_at(&mut w, px - 6.0, 10.0));
    let (iid, oid) = (w.troops[inside].id, w.troops[out].id);
    apply_damage(&mut w, iid, 5.0);
    apply_damage(&mut w, oid, 5.0);
    assert!(
        w.troops[inside].hp == 6.0 && w.troops[out].hp == 5.0,
        "Guardian: 5u {}, 6u {}",
        w.troops[inside].hp,
        w.troops[out].hp
    );
}

/// (d) Frost Armor: Ein Gegner, der den Besitzer im Nahkampf trifft, läuft 2 s mit Faktor 0,7; Spieler 1 ohne Passiv nicht.
#[test]
fn passives_frost_armor() {
    let (mut w, p0, p1) = passive_world();
    must_learn(&mut w, p0, &["fireball", "iceWall", "frostArmor"]);
    let (x0, x1) = (w.players[p0].x, w.players[p1].x);
    let (e0, e1) = (tough_enemy(&mut w, x0 + 1.0), tough_enemy(&mut w, x1 - 1.0));
    let v = vals(&w);
    let target = |w: &World, pi: usize| Target {
        id: w.players[pi].id,
        x: w.players[pi].x,
        kind: TargetKind::Player,
        player: Some(pi),
    };
    let (t0, t1) = (target(&w, p0), target(&w, p1));
    attack(&mut w, &v, e0, &t0, None);
    attack(&mut w, &v, e1, &t1, None);
    let (a, b) = (&w.enemies[e0], &w.enemies[e1]);
    assert!(
        a.slow == 0.7 && a.slow_for == 2.0 && b.slow_for == 0.0,
        "Frost Armor: {} für {} s, ohne Passiv {} s",
        a.slow,
        a.slow_for,
        b.slow_for
    );
    (w.enemies[e0].slow, w.enemies[e0].slow_for) = (0.5, 4.0); // stärkere Ice Wall bleibt
    attack(&mut w, &v, e0, &t0, None);
    let a = &w.enemies[e0];
    assert!(
        a.slow == 0.5 && a.slow_for == 4.0,
        "Ice Wall überschrieben: {} für {} s",
        a.slow,
        a.slow_for
    );
}

/// (d) Holy Ground: Truppen im Burgradius werden geheilt, egal wo der Heiler steht; Monarchen haben keine HP.
#[test]
fn passives_holy_ground() {
    let (mut w, _, p1) = passive_world();
    must_learn(&mut w, p1, &["heal", "groupHeal", "holyGround"]);
    let cx = w.castle.x;
    let tr = troop_at(&mut w, cx, 2.0);
    step_ticks(&mut w, 1);
    assert_eq!(w.troops[tr].hp, 2.0 + DT, "Holy Ground: Truppe {}", w.troops[tr].hp);
}

/// Go: `echoRun`: Zauberer mit gelerntem Passiv wirkt 50-mal Fireball (Abklingzeit zurückgesetzt); Ergebnis: Schaden am Ziel.
fn echo_run(passive: &str) -> f64 {
    let (mut w, p0, _) = passive_world();
    must_learn(&mut w, p0, &["fireball", "iceWall", passive]);
    let px = w.players[p0].x;
    let e = crate::enemies::spawn_scaled(&mut w, "skeleton", px + 5.0, 1000.0, 1.0, None);
    for _ in 0..50 {
        w.players[p0].cooldowns.clear();
        skills_tick(&mut w, p0, &skill_cmd(1));
    }
    lost(&w, e)
}

/// (e) Spell Echo: gleicher Seed gleiches Ergebnis, Echos treten auf; ohne Spell Echo bleibt `w.rng` unberührt.
#[test]
fn passives_spell_echo() {
    let (a, b) = (echo_run("spellEcho"), echo_run("spellEcho"));
    assert!(
        a == b && a > 50.0 * 30.0 && a < 100.0 * 30.0,
        "Spell Echo: Lauf 1 {a}, Lauf 2 {b} (50 Zauber à 30)"
    );
    let (mut w, p0, _) = passive_world();
    must_learn(&mut w, p0, &["fireball", "iceWall", "frostArmor"]);
    let px = w.players[p0].x;
    tough_enemy(&mut w, px + 5.0);
    let before = w.rng.clone();
    skills_tick(&mut w, p0, &skill_cmd(1));
    assert!(
        w.rng == before && w.players[p0].cooldowns[0] != 0.0,
        "ohne Spell Echo Zufallswert verbraucht oder kein Zauber"
    );
}

/// (f) Ohne gelernte Passive liefern die Hilfsfunktionen die Basiswerte.
#[test]
fn passives_ohne_basiswerte() {
    let (mut w, p0, p1) = passive_world();
    must_learn(&mut w, p0, &["taunt", "shieldBash"]);
    let base = vals(&w).monarch.base.speed * vals(&w).monarch.mount.speed_factor;
    for pi in [p0, p1] {
        assert_eq!(
            basis_of(&w, pi),
            (base, 1.0, 1.0),
            "Spieler {} weicht von der Basis ab",
            w.players[pi].index
        );
    }
}

/// Alle zwölf Passive stehen mit Wirkung und als vorläufig markiert in monarch.json.
#[test]
fn passives_daten() {
    let v = vals(&quiet_world());
    let mut n = 0;
    for s in &v.skills.list {
        if s.kind != "passive" || s.line == "thief" || s.line == "purse" {
            continue; // Geldsack: purse_test.go
        }
        n += 1;
        assert!(
            s.provisional && !s.effect.r#type.is_empty(),
            "{}: provisional {}, effect {:?}",
            s.id,
            s.provisional,
            s.effect.r#type
        );
    }
    assert_eq!(n, 12, "{n} Passive statt 12");
}

/// Zwei Monarchen mit verschiedenen Passiven im selben Tick: Blessings wirkt in der Aura des Besitzers (Radius 8), Thick Skin,
/// Armor Aura und Guardian nur für Truppen im Radius; `troop_damage` rechnet die Summen je Besitzer.
#[test]
fn own_troop_damage_und_stat_summen_fuer_zwei_spieler() {
    let (mut w, p0, p1) = passive_world();
    add_pool_points(&mut w, 4);
    must_learn(&mut w, p0, &["taunt", "shieldBash", "armorAura", "guardian"]);
    must_learn(&mut w, p1, &["heal", "groupHeal", "blessings"]);
    let (px, aura) = (w.players[p0].x, effect_of(&w, "armorAura").aura);
    let (inside, out) = (
        troop_at(&mut w, px + 4.0, 100.0),
        troop_at(&mut w, px + aura + 20.0, 100.0),
    );
    let bonus = effect_of(&w, "armorAura").defense;
    let cut = effect_of(&w, "guardian").reduction;
    // innen: Verteidigung zuerst (mindestens 1), dann Guardian-Abzug
    assert!(near(
        troop_damage(&w, inside, 17.0),
        (17.0 - bonus).max(1.0) * (1.0 - cut)
    ));
    assert_eq!(
        troop_damage(&w, inside, 0.5),
        1.0 * (1.0 - cut),
        "Verteidigung lässt mindestens 1"
    );
    assert_eq!(troop_damage(&w, out, 17.0), 17.0, "außerhalb beider Radien unverändert");
    // Blessings des zweiten Spielers wirken auf den ersten nur innerhalb des Radius
    let v = vals(&w);
    w.players[p0].x = w.players[p1].x + 4.0;
    assert!(near(
        speed_of(&w, &v, p0),
        v.monarch.base.speed * v.monarch.mount.speed_factor * 1.1
    ));
    assert!(near(damage_mult_of(&w, &v, p0, false), 1.1) && near(speed_of(&w, &v, p1), speed_of(&w, &v, p0)));
    w.players[p0].x = w.players[p1].x + 9.0;
    assert_eq!(damage_mult_of(&w, &v, p0, false), 1.0);
    // Tote Besitzer (ohne Krone) wirken nicht
    crate::crown::testutil::fell(&mut w, p1);
    w.players[p0].x = w.players[p1].x + 1.0;
    assert_eq!(damage_mult_of(&w, &v, p0, false), 1.0, "ohne Krone wirkt kein Passiv");
    let mut besitzer = 0;
    each_passive(&w, &mut |o, _| {
        assert_eq!(o.id, w.players[p0].id);
        besitzer += 1;
    });
    assert_eq!(besitzer, 2, "nur die zwei Passive von Spieler 0 (armorAura, guardian)");
}
