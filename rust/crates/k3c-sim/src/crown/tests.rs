//! Port von `engine/sim/crown_test.go` (B-930, B-931, docs/rules/kampf.md §§ 5 und 6). Tests über `step` (RU19), `emit`
//! oder Inseln (RU10) tragen `ignore`; die Krone selbst läuft über `step_crowns`.

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
#[ignore = "wartet auf RU18 (step_players): Go-Test TestKroneMitspielerBringtZurueck läuft über Step, der Rust-Rumpf nicht"]
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

/// Go: `crownWorld` mit `quietWorld` (für die Tests über `step`): zwei Monarchen weit weg von der Burg, Portal bei Hub + 90.
fn quiet_crown_world() -> crate::state::World {
    let mut w = crate::enemies::testutil::quiet_world();
    let (p0, p1) = (crate::world::add_player(&mut w), crate::world::add_player(&mut w));
    (w.players[p0].x, w.players[p1].x) = (w.hub_x + 30.0, w.hub_x + 34.0);
    w.portals = vec![w.hub_x + 90.0];
    w
}

/// Go: `steps`: rechnet `seconds` lang mit den Eingaben `cmds`.
fn steps(w: &mut crate::state::World, cmds: &[crate::state::PlayerCommand], seconds: f64) {
    for _ in 0..(seconds / DT).ceil() as usize {
        crate::world::step(w, cmds, None, DT);
    }
}

/// Go: `TestKroneTrefferKostetMuenzeUndSieFaelltZuBoden` und `TestKroneFaelltOhneGoldNachHinten` (AC-01, AC-02, B-930, B-931):
/// Ein Treffer kostet `hitCoins` Münzen, die als Loot-Münze zu Boden fallen; ohne Gold fällt die Krone nach hinten, weg vom
/// Gegner, deterministisch.
#[test]
#[ignore = "wartet auf RU19 (hit_event, emit, unit_x, pure)"]
fn treffer_kostet_muenze_krone_faellt_ohne_gold() {
    use crate::state::common::apply_damage;
    let mut w = quiet_crown_world();
    let hit_coins = crate::world::vals(&w).monarch.hit_coins;
    for p in &mut w.players {
        p.gold = PROBE_GOLD;
    }
    w.coins.clear();
    let id0 = w.players[0].id;
    apply_damage(&mut w, id0, 40.0);
    assert!(
        w.players[0].gold == PROBE_GOLD - hit_coins && w.players[1].gold == PROBE_GOLD && w.players[0].crown.is_none(),
        "Gold {}/{}, Krone {:?}",
        w.players[0].gold,
        w.players[1].gold,
        w.players[0].crown
    );
    assert!(
        w.coins.len() as i64 == hit_coins && w.coins[0].loot && w.coins[0].blocked_player_id == Some(id0),
        "Münzen am Boden: {:?}",
        w.coins
    );
    steps(&mut w, &[], 0.5);
    assert_eq!(
        w.players[0].gold,
        PROBE_GOLD - hit_coins,
        "der Getroffene sammelt die Münze sofort wieder ein"
    );
    w.players[1].x = w.coins[0].x; // der Mitspieler darf sie aufsammeln
    steps(&mut w, &[], 0.1);
    assert_eq!(
        w.players[1].gold,
        PROBE_GOLD + 1,
        "Mitspieler sammelt die Münze nicht auf"
    );
    // ohne Gold fällt die Krone nach hinten (AC-02)
    let distance = crate::world::vals(&w).monarch.crown.distance_units;
    for (dx, dir) in [(1.0, -1.0), (-1.0, 1.0)] {
        let mut w = quiet_crown_world();
        w.players[0].gold = 0;
        let px = w.players[0].x;
        spawn(&mut w, "goblin", px + dx);
        let id = w.players[0].id;
        apply_damage(&mut w, id, 5.0);
        let c0 = w.players[0].crown.clone();
        assert!(
            c0.as_ref()
                .is_some_and(|c| c.state == CROWN_FLYING && c.to_x == px + dir * distance),
            "Gegner bei {dx}: Krone {c0:?}"
        );
    }
}

/// Go: `TestKroneOhneKroneNurLaufen`: Ohne Krone kann der Monarch nur laufen und eine Krone aufheben: kein Schlag, kein Skill,
/// kein Zahlen, keine Münzen.
#[test]
#[ignore = "wartet auf RU19 (pure, emit, unit_x)"]
fn ohne_krone_nur_laufen() {
    use crate::enemies::target::choose_target;
    use crate::state::PlayerCommand;
    let mut w = quiet_crown_world();
    w.players[0].gold = 5;
    fell(&mut w, 0);
    let px = w.players[0].x;
    if let Some(c) = w.players[0].crown.as_mut() {
        c.x = px + 100.0;
    }
    let e = spawn(&mut w, "goblin", px + 1.0);
    let cmd = PlayerCommand {
        move_x: 1.0,
        attack: true,
        pay: true,
        skill: 1,
        ..PlayerCommand::default()
    };
    let id = w.new_id();
    w.coins.push(Coin {
        id,
        x: px,
        ..Default::default()
    });
    let hp = w.enemies[e].hp;
    steps(&mut w, &[cmd, PlayerCommand::default()], 1.0);
    let p = &w.players[0];
    assert!(
        p.x > px && w.enemies[e].hp == hp && !p.paying && p.gold == 5,
        "x {px} → {}, Gegner-HP {hp} → {}, zahlt {}, Gold {}",
        p.x,
        w.enemies[e].hp,
        p.paying,
        p.gold
    );
    let v = crate::world::vals(&w);
    let dir = crate::state::common::sign(w.hub_x - w.enemies[e].x);
    let id0 = w.players[0].id;
    let target = choose_target(&w, &v, e, dir, None);
    assert!(
        target.is_none_or(|t| t.id != id0),
        "ein Monarch ohne Krone ist Ziel eines Gegners"
    );
}

/// Go: `TestKroneRaubIstGameOver` (AC-02, B-931): Ein Gegner hebt die Krone auf, trägt sie ins Portal: Kronenraub, Game Over in
/// jedem Grad.
#[test]
#[ignore = "wartet auf RU19 (pure, emit, unit_x)"]
fn kronenraub_ist_game_over() {
    let mut w = quiet_crown_world();
    fell(&mut w, 0);
    let px = w.players[0].x;
    let e = spawn(&mut w, "goblin", px + 0.5);
    w.players[0].x -= 20.0; // der Besitzer steht nicht an der Krone
    w.enemies[e].home_x = w.portals[0];
    let mut over = false;
    for _ in 0..30 * 60 {
        crate::world::step(&mut w, &[], None, DT);
        let carried = w.players[0].crown.as_ref().is_some_and(|c| c.state == CROWN_ENEMY);
        assert!(
            !(carried && !w.enemies.get(e).is_some_and(|en| en.fleeing)),
            "Träger flieht nicht"
        );
        over = over || w.events.iter().any(|ev| ev["type"] == "gameOver");
        if over {
            break;
        }
    }
    assert!(over, "kein Game Over nach Kronenraub; Krone {:?}", w.players[0].crown);
}

/// Go: `TestKroneRaubStuftMitInselOver`, ohne `Island`: Die Stufe meldet dem `IslandRef`, dass die Insel vorbei ist.
#[test]
#[ignore = "wartet auf RU19 (emit, unit_x: crown_stolen und game_over melden Ereignisse)"]
fn kronenraub_stuft_mit_insel_over() {
    use crate::state::IslandRef;
    let mut w = crate::enemies::testutil::quiet_world();
    let pi = crate::world::add_player(&mut w);
    fell(&mut w, pi);
    let px = w.players[pi].x;
    let ei = spawn(&mut w, "goblin", px);
    let eid = w.enemies[ei].id;
    let crown = w.players[pi].crown.clone().map(|mut c| {
        c.state = CROWN_ENEMY.into();
        c.carrier = eid;
        c
    });
    let e = &mut w.enemies[ei];
    (e.crown, e.fleeing, e.home_x) = (crown, true, e.x);
    let mut isl = IslandRef::default();
    crate::enemies::leave_through_portal(&mut w, Some(&mut isl));
    assert!(isl.over, "Island.Over nach Kronenraub nicht gesetzt");
}

/// Go: `TestBurgfallBeendetDasSpielNicht` (AC-03, B-931): Burgfall beendet das Spiel nicht mehr, in keinem Niederlage-Modus.
/// Ohne `Island`: Die Stufe ruft `castle_fallen` (wie `step`, sobald die Burg-HP 0 sind) mit dem Bezug `IslandRef`.
#[test]
fn burgfall_beendet_das_spiel_nicht() {
    use crate::island::options::{DEFEAT_NAMES, IslandOptions, validate_options};
    use crate::state::IslandRef;
    for mode in DEFEAT_NAMES {
        let mut w = crate::enemies::testutil::quiet_world();
        crate::world::add_player(&mut w);
        let mut isl = IslandRef {
            defeat_resources: mode == "resources",
            ..IslandRef::default()
        };
        w.castle.hp = 0.0;
        crate::world::castle_fallen(&mut w, Some(&mut isl));
        let game_over = w.events.iter().any(|ev| ev["type"] == "gameOver");
        assert!(!isl.over && !game_over, "Modus {mode}: Burgfall beendet das Spiel");
    }
    let lost = IslandOptions {
        grade: "normal".into(),
        goal: "endboss".into(),
        defeat: "lost".into(),
    };
    assert!(
        validate_options(&lost).is_err(),
        "der Modus lost (Komplett verloren) darf nicht mehr gelten"
    );
}
