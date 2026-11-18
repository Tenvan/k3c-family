//! Port von `engine/sim/crown_test.go` (B-930, B-931, docs/rules/kampf.md §§ 5 und 6). Tests mit `Step` (RU14),
//! `applyDamage` (RU18) oder Inseln (RU10) tragen `ignore`; die Krone selbst läuft über `step_crowns` und die Aufrufe der
//! Datei.

use super::testutil::{PROBE_GOLD, crown_world, fell, probe};
use super::{
    CROWN_ENEMY, CROWN_FLYING, CROWN_GROUND, CROWN_PLAYER, crown_direction, drop_crown_of, drop_crown_with,
    hit_coins_with, step_crowns, take_crown_or_loot,
};
use crate::enemies::testutil::spawn;
use crate::state::{Coin, Player};

const DT: f64 = 1.0 / 30.0;

#[test]
fn treffer_muenzen_fallen_als_loot_und_sind_fuer_den_getroffenen_gesperrt() {
    let (mut w, v) = crown_world();
    probe(&mut w);
    hit_coins_with(&mut w, &v, 0, v.monarch.hit_coins);
    assert_eq!(w.coins.len() as i64, v.monarch.hit_coins);
    let pid = w.players[0].id;
    assert!(w.coins[0].loot && w.coins[0].blocked_player_id == Some(pid));
    assert!(w.coins.iter().all(|c| c.blocked_until > w.time));
    assert_eq!(w.players[1].gold, PROBE_GOLD, "der Mitspieler verliert nichts");
}

#[test]
#[ignore = "wartet auf RU19 (emit)"]
fn krone_faellt_nach_hinten_weg_vom_gegner() {
    for (dx, dir) in [(1.0, -1.0), (-1.0, 1.0)] {
        let (mut w, v) = crown_world();
        let ex = w.players[0].x + dx;
        spawn(&mut w, "goblin", ex);
        assert_eq!(crown_direction(&w, 0), dir);
        drop_crown_with(&mut w, &v, 0, "goblin");
        let c = w.players[0].crown.clone().expect("Krone");
        assert_eq!(c.state, CROWN_FLYING);
        assert_eq!(c.to_x, w.players[0].x + dir * v.monarch.crown.distance_units);
    }
}

#[test]
#[ignore = "wartet auf RU19 (emit)"]
fn krone_landet_nach_der_flugzeit_und_der_flug_ist_deterministisch() {
    let land = || {
        let (mut w, v) = crown_world();
        drop_crown_with(&mut w, &v, 0, "goblin");
        let to_x = w.players[0].crown.as_ref().map(|c| c.to_x).expect("Krone");
        let mut peak = 0.0f64;
        for _ in 0..((v.monarch.crown.flight_seconds / DT) as usize + 2) {
            step_crowns(&mut w, &v, DT);
            peak = peak.max(w.players[0].crown.as_ref().map_or(0.0, |c| c.z));
        }
        (w.players[0].crown.clone().expect("Krone"), to_x, peak, v)
    };
    let (c, to_x, peak, v) = land();
    assert_eq!(c.state, CROWN_GROUND);
    assert_eq!(c.z, 0.0);
    assert!((c.x - to_x).abs() < 1e-9);
    assert!(peak >= v.monarch.crown.height_units * 0.9, "Scheitel {peak}");
    assert_eq!(land().0, c, "gleicher Ablauf, gleiche Krone");
}

#[test]
#[ignore = "wartet auf RU19 (emit)"]
fn besitzer_hebt_seine_krone_auf() {
    let (mut w, v) = crown_world();
    drop_crown_with(&mut w, &v, 0, "goblin");
    for _ in 0..((v.monarch.crown.flight_seconds / DT) as usize + 6) {
        step_crowns(&mut w, &v, DT);
    }
    w.players[0].x = w.players[0].crown.as_ref().map_or(0.0, |c| c.x);
    step_crowns(&mut w, &v, DT);
    assert!(w.players[0].crown.is_none(), "{:?}", w.players[0].crown);
}

#[test]
fn kein_raub_ohne_portale() {
    let (mut w, v) = crown_world();
    w.portals.clear();
    fell(&mut w, 0);
    let ex = w.players[0].x;
    let e = spawn(&mut w, "goblin", ex);
    take_crown_or_loot(&mut w, &v, e, None);
    let c = w.players[0].crown.as_ref().expect("Krone");
    assert!(c.state == CROWN_GROUND && w.enemies[e].crown.is_none() && !w.enemies[e].fleeing);
}

#[test]
#[ignore = "wartet auf RU19 (emit)"]
fn gegner_hebt_krone_auf_flieht_und_traegt_sie_als_kopie() {
    let (mut w, v) = crown_world();
    fell(&mut w, 0);
    let ex = w.players[0].x;
    let e = spawn(&mut w, "goblin", ex);
    w.enemies[e].home_x = w.portals[0];
    take_crown_or_loot(&mut w, &v, e, None);
    let c = w.players[0].crown.as_ref().expect("Krone");
    assert_eq!((c.state.as_str(), c.carrier), (CROWN_ENEMY, w.enemies[e].id));
    assert!(w.enemies[e].fleeing);
    assert_eq!(w.enemies[e].crown.as_ref().map(|c| c.owner), Some(0));
}

#[test]
#[ignore = "wartet auf RU19 (emit)"]
fn traeger_stirbt_die_krone_faellt_an_seiner_stelle() {
    let (mut w, v) = crown_world();
    fell(&mut w, 0);
    let ex = w.players[0].x;
    let e = spawn(&mut w, "goblin", ex);
    take_crown_or_loot(&mut w, &v, e, None);
    let mut dead = std::mem::take(&mut w.enemies[e]);
    dead.x = ex + 7.0;
    drop_crown_of(&mut w, &mut dead);
    let c = w.players[0].crown.as_ref().expect("Krone");
    assert_eq!((c.state.as_str(), c.x, c.carrier), (CROWN_GROUND, ex + 7.0, 0));
    assert!(dead.crown.is_none());
}

#[test]
#[ignore = "wartet auf RU19 (emit)"]
fn mitspieler_traegt_die_krone_zum_besitzer_zurueck() {
    let (mut w, v) = crown_world();
    fell(&mut w, 0);
    let cx = w.players[0].x - 10.0;
    w.players[0].x = cx;
    if let Some(c) = w.players[0].crown.as_mut() {
        c.x = cx;
    }
    w.players[1].x = cx + 1.0;
    step_crowns(&mut w, &v, DT);
    let c = w.players[0].crown.as_ref().expect("Krone");
    assert_eq!(c.state, CROWN_PLAYER, "der Mitspieler hebt auf");
    assert_eq!(c.carrier, w.players[1].id);
    w.players[1].x = cx + 0.5; // beim Besitzer angekommen
    for _ in 0..3 {
        step_crowns(&mut w, &v, DT);
    }
    assert!(w.players[0].crown.is_none(), "{:?}", w.players[0].crown);
}

#[test]
#[ignore = "wartet auf RU19 (emit)"]
fn eine_krone_je_monarch_ein_traeger_nimmt_nur_eine() {
    let (mut w, v) = crown_world();
    let id = w.new_id();
    let x = w.players[0].x;
    w.players.push(Player {
        id,
        index: 2,
        x,
        ..Default::default()
    });
    w.players[0].x = x - 30.0;
    w.players[1].x = x + 30.0;
    fell(&mut w, 0);
    fell(&mut w, 1);
    for p in 0..2 {
        if let Some(c) = w.players[p].crown.as_mut() {
            c.x = x;
        }
    }
    step_crowns(&mut w, &v, DT);
    let (a, b) = (
        w.players[0].crown.clone().expect("a"),
        w.players[1].crown.clone().expect("b"),
    );
    assert_eq!((a.state.as_str(), a.carrier), (CROWN_PLAYER, id));
    assert_eq!(b.state, CROWN_GROUND);
}

#[test]
fn dieb_raubt_treffermuenze_und_flieht() {
    let (mut w, v) = crown_world();
    probe(&mut w);
    let x = w.players[0].x + 20.0;
    for loot in [true, false] {
        let id = w.new_id();
        w.coins.push(Coin {
            id,
            x,
            loot,
            ..Default::default()
        });
    }
    let thief = v
        .enemies
        .iter()
        .find(|(_, d)| d.traits.iter().any(|t| t == "stealsGold"))
        .map(|(k, _)| k.clone())
        .expect("Dieb");
    let e = spawn(&mut w, &thief, x);
    w.enemies[e].home_x = w.portals[0];
    take_crown_or_loot(&mut w, &v, e, None);
    assert_eq!(w.enemies[e].carried_gold, 1);
    assert!(w.enemies[e].fleeing);
    assert_eq!(w.coins.len(), 1);
    assert!(!w.coins[0].loot);
}

#[test]
#[ignore = "wartet auf RU14 (Step) und RU18 (applyDamage)"]
fn treffer_kostet_muenze_krone_faellt_ohne_gold() {}

#[test]
#[ignore = "wartet auf RU14 (Step, stepPlayers)"]
fn ohne_krone_nur_laufen() {}

#[test]
#[ignore = "wartet auf RU14 (Step, game_over)"]
fn kronenraub_ist_game_over() {}

#[test]
#[ignore = "wartet auf RU10 (Island) und RU14"]
fn kronenraub_stuft_mit_insel_over() {}

#[test]
#[ignore = "wartet auf RU14 (Step) und RU10 (Island, Defeat-Modi)"]
fn burgfall_beendet_das_spiel_nicht() {}
