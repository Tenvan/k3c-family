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

/// Go: `portalIsland`: Insel mit einer Stufe (zwei Spieler, zwei Portale), ohne Truppen, Gegner und Camps.
fn portal_island() -> (crate::island::Island, k3c_data::Data) {
    use crate::island::testutil::{ISLAND_SPEED, standard_world, test_data};
    let data = test_data();
    let mut isl = crate::island::create_island(
        &data,
        &standard_world(&data),
        "portal",
        &[0],
        ISLAND_SPEED,
        Default::default(),
    )
    .expect("Insel");
    crate::island::add_island_player(&mut isl, 0);
    crate::island::add_island_player(&mut isl, 0);
    let w = &mut isl.stages[0];
    w.troops.clear();
    w.enemies.clear();
    w.spawn_queue.clear();
    w.camps.clear();
    w.aggression = Default::default();
    assert!(
        w.players.len() == 2 && w.portals.len() == 2,
        "{} Spieler, {} Portale",
        w.players.len(),
        w.portals.len()
    );
    (isl, data)
}

/// Go: `attacker` auf der Stufe 0 der Insel: ein Krieger im Angriff vor Portal `i`.
fn island_attacker(isl: &mut crate::island::Island, data: &k3c_data::Data, i: usize) -> usize {
    let v = crate::world::vals(&isl.stages[0]);
    let _ = data;
    attacker(&mut isl.stages[0], &v, i)
}

fn run_island(isl: &mut crate::island::Island, data: &k3c_data::Data, seconds: f64) {
    crate::island::testutil::run_island(isl, data, &[], seconds);
}

#[test]
fn letztes_portal_macht_stufe_sicher() {
    let (mut isl, data) = portal_island();
    let pool = isl.skill_pool;
    island_attacker(&mut isl, &data, 0);
    isl.stages[0].portal_hp[0] = 1.0;
    run_island(&mut isl, &data, 1.0);
    let w = &isl.stages[0];
    assert!(
        w.portals.len() == 1 && w.portals_down.len() == 1 && !safe(w),
        "nach dem ersten Portal: {:?} / {:?}",
        w.portals,
        w.portals_down
    );
    assert_eq!(isl.skill_pool, pool, "Skill-Punkt schon beim ersten Portal");
    let a = island_attacker(&mut isl, &data, 0);
    let w = &mut isl.stages[0];
    w.troops[a].x = w.portals[0];
    w.portal_hp[0] = 1.0;
    run_island(&mut isl, &data, 1.0);
    let mini = crate::world::vals(&isl.stages[0]).monarch.skill_point_sources.miniboss;
    assert!(
        safe(&isl.stages[0]) && isl.skill_pool == pool + mini,
        "sicher {}, Pool {pool} → {}",
        safe(&isl.stages[0]),
        isl.skill_pool
    );
    let w = &mut isl.stages[0];
    w.cycle.phase = "night".into();
    crate::enemies::waves::start_wave(w, None);
    run_island(&mut isl, &data, 20.0);
    let w = &isl.stages[0];
    assert!(
        w.spawn_queue.is_empty() && w.enemies.is_empty(),
        "Welle in sicherer Stufe: {} + {}",
        w.spawn_queue.len(),
        w.enemies.len()
    );
}

#[test]
fn sichere_stufe_bleibt_nach_dem_laden() {
    use crate::island::testutil::{load_island, save_json};
    let (mut isl, data) = portal_island();
    island_attacker(&mut isl, &data, 0);
    isl.stages[0].portal_hp[0] = 1.0;
    run_island(&mut isl, &data, 1.0);
    let saved_at = "2026-11-15T10:00:00Z";
    let raw = save_json(&isl, saved_at);
    let mut loaded = load_island(&data, &raw);
    let lw = &loaded.stages[0];
    assert!(
        lw.portals.len() == 1 && lw.portals_down.len() == 1 && lw.portals[0] != lw.portals_down[0],
        "geladen: {:?} / {:?}",
        lw.portals,
        lw.portals_down
    );
    assert_eq!(save_json(&loaded, saved_at), raw, "Stand ändert sich beim Neuladen");
    loaded.stages[0].troops.clear();
    island_attacker(&mut loaded, &data, 0);
    loaded.stages[0].portal_hp[0] = 1.0;
    run_island(&mut loaded, &data, 1.0);
    let mut again = load_island(&data, &save_json(&loaded, saved_at));
    let aw = &mut again.stages[0];
    aw.cycle.phase = "night".into();
    crate::enemies::waves::start_wave(aw, None);
    run_island(&mut again, &data, 20.0);
    let aw = &again.stages[0];
    assert!(
        safe(aw) && aw.spawn_queue.is_empty() && aw.enemies.is_empty(),
        "sicher {}, Welle {} + {}",
        safe(aw),
        aw.spawn_queue.len(),
        aw.enemies.len()
    );
}

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
fn letztes_portal_ohne_doppel_punkt_bei_miniboss() {
    let (mut isl, data) = portal_island();
    let v = crate::world::vals(&isl.stages[0]);
    let Some(b) = crate::boss::data::mini_boss(&v.bosses, isl.stages[0].biome.depth).cloned() else {
        return; // Go: t.Skip("kein Miniboss in dieser Stufe")
    };
    let _ = &data;
    let mini = v.monarch.skill_point_sources.miniboss;
    let pool = isl.skill_pool;
    let secure = |isl: &mut crate::island::Island| {
        let mut r = isl.stage_ref(0, &data.difficulty);
        crate::portal::secure_stage(&mut isl.stages[0], &v, Some(&mut r));
        isl.put_stage_ref(r);
    };
    secure(&mut isl); // Miniboss lebt noch oder ist nie erschienen: der Punkt kommt einmal
    assert!(
        isl.skill_pool == pool + mini && isl.defeated_bosses.contains(&b.id),
        "Pool {pool} → {}, besiegt {:?}",
        isl.skill_pool,
        isl.defeated_bosses
    );
    secure(&mut isl);
    let hub_x = isl.stages[0].hub_x;
    let mut r = isl.stage_ref(0, &data.difficulty);
    let e = crate::boss::spawn_boss(&mut isl.stages[0], &v, &b, hub_x + 100.0, Some(&r));
    isl.stages[0].enemies[e].hp = 0.0;
    crate::enemies::waves::remove_dead_enemies(&mut isl.stages[0], Some(&mut r));
    isl.put_stage_ref(r);
    assert_eq!(
        isl.skill_pool,
        pool + mini,
        "Doppelter Punkt: Pool {pool} → {}",
        isl.skill_pool
    );
}
