//! Eigene Tests zu den Passiven mit 2 Spielern (kein Go-Gegenstück): Heilung, Regeneration und Frost Armor, ohne `step`,
//! `hit_event` und `emit` (RU19).

use super::passives::{frost_armor_hit, step_passives};
use super::passives_tests::{effect_of, passive_world};
use crate::island::testutil::{add_pool_points, must_learn};
use crate::skills::testutil::{tough_enemy, troop_at};

/// Zwei Heiler: Healing Aura wirkt im Radius um den Besitzer, Holy Ground im Burgradius; geheilt wird nie über `max_hp`.
#[test]
fn own_step_passives_heilt_truppen_je_besitzer() {
    let (mut w, p0, p1) = passive_world();
    add_pool_points(&mut w, 4);
    must_learn(&mut w, p0, &["heal", "groupHeal", "healingAura"]);
    must_learn(
        &mut w,
        p1,
        &[
            "heal",
            "groupHeal",
            "healingAura",
            "blessings",
            "divineShield",
            "holyGround",
        ],
    );
    let (x0, cx) = (w.players[p0].x, w.castle.x);
    w.players[p1].x = x0 + 200.0;
    let (near0, far0) = (troop_at(&mut w, x0 + 5.0, 2.0), troop_at(&mut w, x0 + 50.0, 2.0));
    let (at_castle, full) = (troop_at(&mut w, cx, 2.0), troop_at(&mut w, x0, 10.0));
    let per_second = effect_of(&w, "healingAura").per_second;
    step_passives(&mut w, None, 1.0);
    assert!(
        w.troops[near0].hp > 2.0,
        "Healing Aura von Spieler 0 heilt nahe Truppen"
    );
    assert_eq!(w.troops[far0].hp, 2.0, "weit entfernt wird nicht geheilt");
    assert!(
        w.troops[at_castle].hp >= 2.0 + per_second.min(1.0),
        "Holy Ground heilt im Burgradius, auch wenn der Heiler fern steht"
    );
    assert_eq!(w.troops[full].hp, 10.0, "nie über max_hp");
    w.troops[near0].hp = 0.0;
    step_passives(&mut w, None, 1.0);
    assert_eq!(w.troops[near0].hp, 0.0, "Gefallene werden nicht geheilt");
}

/// Regeneration (Gold-Rinnsal): nur tagsüber, Bruchteile sammeln sich, je Spieler getrennt; nie über den Geldsack.
#[test]
fn own_regeneration_sammelt_bruchteile_je_spieler() {
    let (mut w, p0, p1) = passive_world();
    add_pool_points(&mut w, 2);
    must_learn(&mut w, p0, &["taunt", "shieldBash", "thickSkin", "regeneration"]);
    (w.players[p0].gold, w.players[p1].gold) = (10, 10);
    let per_second = effect_of(&w, "regeneration").per_second;
    assert!(
        per_second > 0.0 && per_second < 1.0,
        "Rinnsal: weniger als eine Münze je Sekunde"
    );
    w.cycle.phase = "night".into();
    step_passives(&mut w, None, 1.0 / per_second);
    assert_eq!(w.players[p0].gold, 10, "nachts kein Rinnsal");
    w.cycle.phase = "day".into();
    step_passives(&mut w, None, 0.5 / per_second);
    assert_eq!(w.players[p0].gold, 10, "ein halber Taler ist noch keine Münze");
    step_passives(&mut w, None, 0.5 / per_second);
    assert_eq!(
        (w.players[p0].gold, w.players[p1].gold),
        (11, 10),
        "eine volle Münze für Spieler 0, Spieler 1 bleibt"
    );
    w.players[p0].gold = 1_000_000;
    step_passives(&mut w, None, 100.0 / per_second);
    assert_eq!(w.players[p0].gold, 1_000_000, "über dem Geldsack kein Rinnsal");
}

/// Frost Armor: eine stärkere laufende Verlangsamung bleibt, eine schwächere wird ersetzt; ohne Passiv nichts.
#[test]
fn own_frost_armor_ersetzt_nur_schwaechere_verlangsamung() {
    let (mut w, p0, p1) = passive_world();
    must_learn(&mut w, p0, &["fireball", "iceWall", "frostArmor"]);
    let (x0, x1) = (w.players[p0].x, w.players[p1].x);
    let (e0, e1) = (tough_enemy(&mut w, x0 + 1.0), tough_enemy(&mut w, x1 - 1.0));
    frost_armor_hit(&mut w, p0, e0);
    frost_armor_hit(&mut w, p1, e1);
    assert!(w.enemies[e0].slow == 0.7 && w.enemies[e0].slow_for == 2.0 && w.enemies[e1].slow_for == 0.0);
    (w.enemies[e0].slow, w.enemies[e0].slow_for) = (0.5, 4.0);
    frost_armor_hit(&mut w, p0, e0);
    assert!(
        w.enemies[e0].slow == 0.5 && w.enemies[e0].slow_for == 4.0,
        "stärkere Ice Wall bleibt"
    );
}
