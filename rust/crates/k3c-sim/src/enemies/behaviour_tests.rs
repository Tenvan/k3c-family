//! Verhalten aller Gegner aus data/enemies.json (Go: `enemies_behaviour_test.go`, K1.3, B-013, gegner.md § 2). Die Tests
//! laufen über alle Gegner mit dem Trait, neue Arten in den Daten sind damit automatisch geprüft.

use super::testutil::*;
use super::{step_enemies, target::TargetKind};
use crate::state::World;
use k3c_data::enemies::KNOWN_TRAITS;

/// Go: `traitTests`: getestetes Verhalten je bekanntem Trait.
const TRAIT_TESTS: [(&str, &str); 13] = [
    ("stealsGold", "gegner_steals_gold"),
    ("fleesAtHalfHp", "gegner_flees_at_half_hp"),
    ("ignoresWalls", "gegner_ignores_walls_laeuft_an_der_mauer_vorbei"),
    ("flying", "gegner_traits_getestet (nur mit ignoresWalls)"),
    ("ranged", "gegner_ranged"),
    ("prefersTroops", "gegner_prefers_troops"),
    ("prefersTowers", "gegner_prefers_towers"),
    ("prefersMonarch", "gegner_prefers_monarch"),
    ("prefersBuildings", "gegner_gebaeude_als_ziel"),
    ("aoe", "gegner_flaechenschlag"),
    ("swarm", "gegner_schwarm_platz"),
    ("phases", "gegner_phasen"),
    ("kiting", "gegner_kiting"),
];

/// Go: `kindsWith`: die Arten mit dem Trait, sortiert (Datenmenge wie `enemyData`).
fn kinds_with(w: &World, tr: &str) -> Vec<String> {
    let v = w.vals.clone().expect("Wertesatz");
    let out: Vec<String> = v
        .enemies
        .iter()
        .filter(|(_, d)| d.traits.iter().any(|t| t == tr))
        .map(|(k, _)| k.clone())
        .collect();
    assert!(!out.is_empty(), "kein Gegner mit {tr}");
    out // BTreeMap liefert die Arten schon sortiert
}

/// Datentest: Jedes bekannte Trait hat ein getestetes Verhalten, `flying` steht nur mit `ignoresWalls`.
#[test]
fn gegner_traits_getestet() {
    for tr in KNOWN_TRAITS {
        assert!(
            TRAIT_TESTS.iter().any(|(t, n)| *t == tr && !n.is_empty()),
            "Trait {tr} ohne Test"
        );
    }
    let w = bare_world();
    let v = w.vals.clone().expect("Wertesatz");
    for kind in kinds_with(&w, "flying") {
        assert!(
            v.enemies[&kind].traits.iter().any(|t| t == "ignoresWalls"),
            "{kind} fliegt ohne ignoresWalls"
        );
    }
}

/// Gegner ohne Traits laufen auf die Burg zu und schlagen eine Truppe in Reichweite mit dem Schaden aus den Daten.
#[test]
#[ignore = "wartet auf RU14"]
fn gegner_ohne_traits_nahkampf() {
    let probe_world = bare_world();
    let v = probe_world.vals.clone().expect("Wertesatz");
    for (kind, d) in &v.enemies {
        if !d.traits.is_empty() {
            continue;
        }
        let mut w = quiet_world();
        let x = w.hub_x - 30.0;
        let e = spawn(&mut w, kind, x);
        step_for(&mut w, 1.0);
        let at = w.enemies[e].x + 1.0;
        let tr = warrior_at(&mut w, at);
        w.enemies[e].cooldown = 0.0;
        step_enemies(&mut w, None, DT);
        assert!(
            w.enemies[e].x > x && w.troops[tr].hp == w.troops[tr].max_hp - d.damage,
            "{kind}"
        );
    }
}

/// `stealsGold`: klaut dem nächsten von zwei Spielern Gold, flieht damit und verschwindet im Portal.
#[test]
#[ignore = "wartet auf RU14"]
fn gegner_steals_gold() {
    for kind in kinds_with(&bare_world(), "stealsGold") {
        let mut w = quiet_world();
        let (p1, p2) = (add_player(&mut w), add_player(&mut w));
        let at = w.hub_x - 40.0;
        let e = spawn(&mut w, &kind, at);
        w.enemies[e].x = w.hub_x - 30.0;
        w.players[p1].x = w.enemies[e].x + 0.5;
        w.players[p2].x = w.enemies[e].x + 1.0;
        let (g1, g2) = (w.players[p1].gold, w.players[p2].gold);
        let steal = w.vals.as_ref().map_or(0, |v| v.waves.steal_gold);
        step_enemies(&mut w, None, DT);
        assert!(
            w.players[p1].gold == g1 - steal && w.enemies[e].carried_gold == steal && w.enemies[e].fleeing,
            "{kind}"
        );
        step_for(&mut w, 5.0);
        assert!(
            w.enemies.is_empty() && w.players[p2].gold == g2 && w.players[p1].crown.is_none(),
            "{kind}"
        );
    }
}

/// `fleesAtHalfHp`: Bei halber HP kämpft der Gegner noch, darunter flieht er ohne Angriff und verschwindet im Portal.
#[test]
#[ignore = "wartet auf RU14"]
fn gegner_flees_at_half_hp() {
    for kind in kinds_with(&bare_world(), "fleesAtHalfHp") {
        let mut w = quiet_world();
        let at = w.hub_x - 40.0;
        let e = spawn(&mut w, &kind, at);
        w.enemies[e].x = w.hub_x - 30.0;
        w.enemies[e].hp = w.enemies[e].max_hp / 2.0;
        let at = w.enemies[e].x + 0.5;
        let tr = warrior_at(&mut w, at);
        step_enemies(&mut w, None, DT);
        assert!(
            !w.enemies[e].fleeing && w.troops[tr].hp != w.troops[tr].max_hp,
            "{kind} bei halber HP"
        );
        w.enemies[e].hp = w.enemies[e].max_hp / 2.0 - 1.0;
        w.enemies[e].cooldown = 0.0;
        w.troops[tr].hp = w.troops[tr].max_hp;
        step_enemies(&mut w, None, DT);
        assert!(w.enemies[e].fleeing && w.troops[tr].hp == w.troops[tr].max_hp && w.enemies[e].x < w.hub_x - 30.0);
        step_for(&mut w, 5.0);
        assert!(w.enemies.is_empty(), "{kind} nicht im Portal verschwunden");
    }
}

/// `ranged`: Fernkämpfer schießen auf eine Truppe außerhalb der Nahkampf-Reichweite, das Geschoss trägt ihren Schaden.
#[test]
#[ignore = "wartet auf RU14"]
fn gegner_ranged() {
    for kind in kinds_with(&bare_world(), "ranged") {
        let mut w = quiet_world();
        let at = w.hub_x - 30.0;
        let e = spawn(&mut w, &kind, at);
        let at = w.enemies[e].x + 5.0;
        let tr = warrior_at(&mut w, at);
        step_enemies(&mut w, None, DT);
        let tid = w.troops[tr].id;
        assert!(w.projectiles.len() == 1 && w.projectiles[0].target_id == tid, "{kind}");
        assert_eq!(w.projectiles[0].damage, w.enemies[e].damage);
        // `stepProjectiles` (RU15) trifft den Krieger; der Treffer wird dort geprüft.
    }
}

/// `prefersTroops`: greift die Truppe an, obwohl zwei Spieler näher stehen.
#[test]
#[ignore = "wartet auf RU14"]
fn gegner_prefers_troops() {
    for kind in kinds_with(&bare_world(), "prefersTroops") {
        let mut w = quiet_world();
        let (p1, p2) = (add_player(&mut w), add_player(&mut w));
        let at = w.hub_x - 30.0;
        let e = spawn(&mut w, &kind, at);
        w.players[p1].x = w.enemies[e].x + 0.3;
        w.players[p2].x = w.enemies[e].x + 0.5;
        let at = w.enemies[e].x + 1.2;
        let tr = warrior_at(&mut w, at);
        probe(&mut w, &[p1, p2]);
        step_enemies(&mut w, None, DT);
        assert!(
            w.troops[tr].hp != w.troops[tr].max_hp && !was_hit(&w, p1) && !was_hit(&w, p2),
            "{kind}"
        );
    }
}

/// `prefersMonarch`: greift den nächsten Spieler an, obwohl eine Truppe näher steht.
#[test]
#[ignore = "wartet auf RU14"]
fn gegner_prefers_monarch() {
    for kind in kinds_with(&bare_world(), "prefersMonarch") {
        let mut w = quiet_world();
        let (p1, p2) = (add_player(&mut w), add_player(&mut w));
        let at = w.hub_x - 30.0;
        let e = spawn(&mut w, &kind, at);
        let at = w.enemies[e].x + 0.3;
        let tr = warrior_at(&mut w, at);
        w.players[p1].x = w.enemies[e].x + 1.0;
        w.players[p2].x = w.enemies[e].x + 1.3;
        probe(&mut w, &[p1, p2]);
        step_enemies(&mut w, None, DT);
        assert!(
            was_hit(&w, p1) && !was_hit(&w, p2) && w.troops[tr].hp == w.troops[tr].max_hp,
            "{kind}"
        );
    }
}

/// `prefersTowers`: schießt auf den gebauten Turm, obwohl zwei Spieler näher stehen.
#[test]
#[ignore = "wartet auf RU14"]
fn gegner_prefers_towers() {
    for kind in kinds_with(&bare_world(), "prefersTowers") {
        let mut w = quiet_world();
        let (p1, p2) = (add_player(&mut w), add_player(&mut w));
        let tower = build_site(&mut w, "tower");
        let dir = crate::state::common::sign(w.hub_x - w.sites[tower].x);
        let at = w.sites[tower].x - dir * 6.0;
        let e = spawn(&mut w, &kind, at);
        w.players[p1].x = w.enemies[e].x + dir * 2.0;
        w.players[p2].x = w.enemies[e].x + dir * 3.0;
        step_enemies(&mut w, None, DT);
        let tid = w.sites[tower].id;
        assert!(w.projectiles.len() == 1 && w.projectiles[0].target_id == tid, "{kind}");
    }
}

/// Die Zielart-Texte stimmen mit Go überein (Ereignisse und Tests lesen sie).
#[test]
fn ziel_arten_haben_die_go_texte() {
    use TargetKind::*;
    let all = [Player, Troop, Merchant, Wall, Castle, Site, Building];
    let texts: Vec<&str> = all.iter().map(|k| k.as_str()).collect();
    assert_eq!(
        texts,
        ["player", "troop", "merchant", "wall", "castle", "site", "building"]
    );
}
