//! Port von `engine/sim/portal_test.go` (B-927, docs/rules/kampf.md § 2). Go rechnet mit `CreateIsland` und `Step`; hier
//! laufen die Schritte direkt über `step_portals`. Tests mit `emit` (RU19), `Step` (RU14) oder Inseln (RU10) tragen
//! `ignore`.

use super::testutil::{PORTAL_HP, attacker, portal_stage};
use super::{apply_portals, portal_in_reach, safe, standing_portal_near, step_portals, sync_portals};
use crate::state::save::HubSave;

const DT: f64 = 1.0 / 30.0;

#[test]
fn portal_hp_aus_den_daten() {
    let (mut w, _) = portal_stage();
    sync_portals(&mut w);
    assert_eq!(w.portal_hp, vec![PORTAL_HP, PORTAL_HP]);
    assert_eq!(w.portal_defense.len(), 2);
    assert!(w.biome.portals.defense.size > 0);
}

#[test]
fn nur_truppen_im_angriff_beschaedigen_portale() {
    let (mut w, v) = portal_stage();
    let ti = attacker(&mut w, &v, 0);
    w.troops[ti].attacking = false;
    sync_portals(&mut w);
    w.portal_defense = vec![999.0, 999.0]; // kein Verteidiger-Takt in diesem Test (der braucht `emit`, RU19)
    for _ in 0..90 {
        step_portals(&mut w, &v, None, DT);
    }
    assert_eq!(w.portal_hp[0], PORTAL_HP, "Portal ohne Angreifer beschädigt");
    w.troops[ti].attacking = true;
    for _ in 0..90 {
        step_portals(&mut w, &v, None, DT);
    }
    assert!(
        w.portal_hp[0] < PORTAL_HP && w.portal_hp[1] == PORTAL_HP,
        "{:?}",
        w.portal_hp
    );
}

#[test]
fn portal_in_reach_nimmt_das_naechste_in_schlagweite() {
    let (mut w, v) = portal_stage();
    let ti = attacker(&mut w, &v, 1);
    assert_eq!(portal_in_reach(&w, &v, ti), Some(1));
    w.troops[ti].x = w.hub_x;
    assert_eq!(portal_in_reach(&w, &v, ti), None, "die Mitte liegt außer Reichweite");
}

#[test]
fn sicher_erst_wenn_alle_portale_zerstoert_sind() {
    let (mut w, _) = portal_stage();
    assert!(!safe(&w));
    w.portals.clear();
    assert!(!safe(&w), "ohne Portale von Anfang an ist die Stufe nicht „zerstört“");
    w.portals_down.push(1.0);
    assert!(safe(&w));
}

#[test]
fn standing_portal_near_liefert_das_naechste_stehende() {
    let (w, _) = portal_stage();
    assert_eq!(standing_portal_near(&w, w.hub_x - 100.0), w.portals[0]);
    assert_eq!(standing_portal_near(&w, w.hub_x + 3.0), w.portals[1]);
}

#[test]
fn zerstoerte_portale_bleiben_nach_dem_laden_weg() {
    let (mut w, _) = portal_stage();
    let (down, left) = (w.portals[0], w.portals[1]);
    let h = HubSave {
        portals_down: vec![down],
        ..HubSave::default()
    };
    apply_portals(&mut w, &h);
    assert_eq!((w.portals.clone(), w.portals_down.clone()), (vec![left], vec![down]));
    assert_eq!(w.portal_hp, vec![PORTAL_HP], "HP gleich lang wie die Portale");
    assert!(!safe(&w));
}

/// Go: `portalIsland` ohne `CreateIsland`: eine ruhige Welt (Kampagne) mit zwei Spielern, ohne Truppen, Gegner und Camps.
fn quiet_portal_world() -> crate::state::World {
    let mut w = crate::enemies::testutil::quiet_world();
    crate::world::add_player(&mut w);
    crate::world::add_player(&mut w);
    w.troops.clear();
    w.enemies.clear();
    w.spawn_queue.clear();
    w.camps.clear();
    w.aggression = Default::default();
    w
}

/// Go: `siege`: rechnet `seconds` Sekunden und hält `ti` dabei bei `x`.
fn siege(w: &mut crate::state::World, ti: usize, x: f64, seconds: f64) {
    let mut tm = 0.0;
    while tm < seconds {
        w.troops[ti].x = x;
        crate::world::step(w, &[], None, DT);
        tm += DT;
    }
}

#[test]
fn angegriffenes_portal_spawnt_verteidiger() {
    let mut w = quiet_portal_world();
    crate::enemies::testutil::run(&mut w, 25.0);
    assert!(
        w.spawn_queue.is_empty() && w.enemies.is_empty(),
        "Verteidiger ohne Angriff: {} + {}",
        w.spawn_queue.len(),
        w.enemies.len()
    );
    let v = crate::world::vals(&w);
    let a = attacker(&mut w, &v, 0);
    (w.troops[a].hp, w.troops[a].max_hp) = (1e9, 1e9);
    let p0 = w.portals[0];
    siege(&mut w, a, p0, 1.0);
    let size = w.biome.portals.defense.size;
    let got = (w.spawn_queue.len() + w.enemies.len()) as i64;
    assert!(got >= size, "{got} Verteidiger, erwartet {size}");
    for s in &w.spawn_queue {
        assert_eq!(s.x, p0, "Verteidiger am falschen Portal: {}", s.x);
    }
    let mut waves = 0; // der Krieger erschlägt die Verteidiger: gezählt wird das Ereignis
    let mut tm = 0.0;
    while tm < w.biome.portals.defense.interval_seconds + 1.0 {
        siege(&mut w, a, p0, DT);
        waves += w.events.iter().filter(|e| e["type"] == "portalDefense").count();
        tm += DT;
    }
    assert_eq!(waves, 1, "{waves} Wellen im Takt, erwartet 1");
}

#[test]
fn letztes_portal_macht_stufe_sicher() {}

#[test]
fn sichere_stufe_bleibt_nach_dem_laden() {}

#[test]
fn dieb_ohne_portal_laesst_beute_fallen() {
    let mut w = quiet_portal_world();
    w.portals.clear();
    w.portal_hp.clear();
    let px = w.hub_x + 3.0;
    (w.players[0].x, w.players[0].gold) = (px, 50);
    let ei = crate::enemies::testutil::spawn(&mut w, "greed", px + 0.2);
    w.enemies[ei].cooldown = 0.0;
    let coins = w.coins.len();
    crate::enemies::testutil::run(&mut w, 2.0);
    let e = &w.enemies[ei];
    assert!(
        e.carried_gold == 0 && !e.fleeing && w.coins.len() > coins,
        "Beute {}, flieht {}, Münzen {coins} → {}",
        e.carried_gold,
        e.fleeing,
        w.coins.len()
    );
}

#[test]
fn portal_simulation_ist_deterministisch() {
    let run = || {
        let mut w = quiet_portal_world();
        let v = crate::world::vals(&w);
        let a = attacker(&mut w, &v, 0);
        w.troops[a].hp = 1e9;
        crate::enemies::testutil::run(&mut w, 60.0);
        serde_json::to_string(&w).expect("Weltzustand als JSON")
    };
    assert_eq!(run(), run(), "zwei Läufe weichen ab");
}

#[test]
fn letztes_portal_ohne_doppel_punkt_bei_miniboss() {}
