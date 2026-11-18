//! Tests zu `guard.rs` (Go: `enemies_guard_test.go`, WZ27.1, B-517): Gegnerlager, mit zwei Spielern über `CreateIsland`.

use super::guard::*;
use super::testutil::*;
use crate::state::Enemy;
use crate::state::save::HubSave;
use k3c_level::types::Guard;

/// Eine Welt mit den Wachen der Planung (Go: `guardIsland` ohne Insel und Schritt).
fn guard_world() -> (crate::state::World, std::sync::Arc<crate::state::WorldValues>) {
    let mut w = bare_world();
    let start = w.hub_x + 50.0 + 3.0 * 50.0;
    w.level.guards = vec![
        Guard {
            kind: "goblin".into(),
            x: start + 20.0,
        },
        Guard {
            kind: "greed".into(),
            x: start + 40.0,
        },
    ];
    let v = w.vals.clone().expect("Wertesatz");
    (w, v)
}

#[test]
fn lager_stellt_wachen_auf() {
    let (mut w, v) = guard_world();
    spawn_guards(&mut w, &v, None);
    let gs = guards_of(&w);
    assert_eq!(gs.len(), 2);
    let start = w.hub_x + 50.0 + 3.0 * 50.0;
    let (a, b) = (&w.enemies[gs[0]], &w.enemies[gs[1]]);
    assert!(a.kind == "goblin" && a.x == start + 20.0 && a.home_x == start + 20.0);
    assert!(b.kind == "greed" && b.x == start + 40.0);
    for g in [a, b] {
        assert!(g.hp > 0.0 && g.hp == g.max_hp, "{}: HP {}/{}", g.kind, g.hp, g.max_hp);
    }
    // Nur beim ersten Tick.
    spawn_guards(&mut w, &v, None);
    assert_eq!(w.enemies.len(), 2);
}

#[test]
fn ohne_lager_keine_wachen() {
    let mut w = bare_world();
    let v = w.vals.clone().expect("Wertesatz");
    spawn_guards(&mut w, &v, None);
    assert!(w.enemies.is_empty() && w.level.guards.is_empty());
}

#[test]
fn geraeumte_lager_kommen_nicht_wieder() {
    let (mut w, v) = guard_world();
    spawn_guards(&mut w, &v, None);
    let first = w.enemies[0].clone();
    mark_cleared(&mut w, &first);
    assert_eq!(guards_cleared_keys(&w).len(), 1);
    // Ein neuer Stand mit demselben Lager: die geräumte Wache fehlt.
    let mut w2 = guard_world().0;
    w2.guards_cleared = w.guards_cleared.clone();
    spawn_guards(&mut w2, &v, None);
    assert_eq!(w2.enemies.len(), 1);
    assert_eq!(w2.enemies[0].kind, "greed");
}

/// Wachen stehen still, solange niemand im Radius ist, und gehören zu keiner Welle.
#[test]
fn wachen_stehen_und_zaehlen_nicht_als_welle() {
    let (mut w, v) = guard_world();
    spawn_guards(&mut w, &v, None);
    let before: Vec<f64> = w.enemies.iter().map(|e| e.x).collect();
    assert!(w.enemies.iter().all(|e| !is_wave_enemy(e)));
    w.portals = vec![0.0];
    super::send_enemies_home(&mut w);
    for ei in 0..w.enemies.len() {
        super::guard::step_guard(&mut w, &v, ei, None, DT);
    }
    let after: Vec<f64> = w.enemies.iter().map(|e| e.x).collect();
    assert_eq!(before, after);
    assert!(w.enemies.iter().all(|e| !e.fleeing));
}

#[test]
fn wache_kehrt_zum_lager_zurueck() {
    let (mut w, v) = guard_world();
    spawn_guards(&mut w, &v, None);
    let home = w.enemies[0].home_x;
    w.enemies[0].x = home + 3.0;
    for _ in 0..300 {
        super::guard::step_guard(&mut w, &v, 0, None, DT);
    }
    assert!(
        (w.enemies[0].x - home).abs() < 1e-9,
        "Wache bei {}, Lager bei {home}",
        w.enemies[0].x
    );
    assert!(!w.enemies[0].fleeing);
}

#[test]
fn keep_guards_laesst_nur_wachen_stehen() {
    let mut es = vec![
        Enemy {
            guard: true,
            ..Enemy::default()
        },
        Enemy::default(),
        Enemy {
            guard: true,
            ..Enemy::default()
        },
    ];
    es[2].id = 7;
    keep_guards(&mut es);
    assert_eq!(es.len(), 2);
    assert_eq!(es[1].id, 7);
}

#[test]
fn apply_guards_uebernimmt_geraeumte_und_entfernt_ihre_wachen() {
    let (mut w, v) = guard_world();
    spawn_guards(&mut w, &v, None);
    let key0 = guard_key(&w.enemies[0]);
    let h = HubSave {
        guards_cleared: vec![key0],
        ..HubSave::default()
    };
    apply_guards(&mut w, &h);
    assert_eq!(w.enemies.len(), 1);
    assert_eq!(w.enemies[0].kind, "greed");
    assert_eq!(guards_cleared_keys(&w).len(), 1);
}

#[test]
#[ignore = "wartet auf RU10 (ruin_island)"]
fn ohne_lager_keine_wachen_in_der_insel() {
    let mut isl = ruin_island(vec![k3c_level::types::PlannedChunk {
        slot: 4,
        kind: "forest".into(),
        ..Default::default()
    }]);
    guard_steps(&mut isl.stages[0], 2.0);
    assert!(isl.stages[0].enemies.is_empty() && isl.stages[0].level.guards.is_empty());
}

#[test]
#[ignore = "wartet auf RU10 (ruin_island)"]
fn lager_stellt_wachen_auf_ueber_die_insel() {
    let mut isl = ruin_island(guard_plan());
    step(&mut isl.stages[0], None, DT); // erster Tick: Wachen stehen
    let w = &isl.stages[0];
    assert!(guards_of(w).len() == 2 && w.level.guards.len() == 2);
}

/// Wachen stehen still und zählen nicht als Welle (über `Step`).
#[test]
#[ignore = "wartet auf RU10 (ruin_island)"]
fn wachen_stehen_und_zaehlen_nicht_als_welle_ueber_step() {
    let mut isl = ruin_island(guard_plan());
    let w = &mut isl.stages[0];
    step(w, None, DT);
    let before = [w.enemies[0].x, w.enemies[1].x];
    assert!(
        !crate::state::common::is_dangerous(w) || w.cycle.phase == "night",
        "Wachen machen den Tag gefährlich"
    );
    super::send_enemies_home(w);
    guard_steps(w, 5.0);
    assert!(
        guards_of(w).len() == 2 && w.enemies[0].x == before[0] && w.enemies[1].x == before[1] && !w.enemies[0].fleeing
    );
}

/// Ziel im Radius: die Wache greift an (Spieler verliert Gold), das Ziel außerhalb des Radius bleibt unbehelligt.
#[test]
#[ignore = "wartet auf RU10 (ruin_island)"]
fn wache_greift_im_radius_an_und_kehrt_zurueck() {
    let mut isl = ruin_island(guard_plan());
    let w = &mut isl.stages[0];
    step(w, None, DT);
    let g = guards_of(w)[0];
    let home = w.enemies[g].home_x;
    let (far, near) = (0, 1);
    w.players[far].x = home - 30.0; // außerhalb des Radius (10)
    w.players[near].x = home - 6.0; // darin
    probe(w, &[far, near]);
    let gold = w.players[near].gold;
    guard_steps(w, 6.0);
    assert!(!was_hit(w, far), "Spieler außerhalb des Radius getroffen");
    assert!(
        was_hit(w, near) || w.players[near].gold != gold,
        "Spieler im Radius nicht angegriffen"
    );
    assert!(w.enemies[g].x != home, "die Wache hat sich nicht auf das Ziel zubewegt");
    w.players[near].x = home - 30.0; // Ziel weg: die Wache kehrt zum Lager zurück
    guard_steps(w, 6.0);
    assert!((w.enemies[g].x - home).abs() <= 1e-9 && !w.enemies[g].fleeing);
}

/// Der Dieb-Trait gilt nicht für Wachen: kein Goldklau, keine Flucht.
#[test]
#[ignore = "wartet auf RU10 (ruin_island)"]
fn wache_klaut_kein_gold() {
    let mut isl = ruin_island(guard_plan());
    let w = &mut isl.stages[0];
    step(w, None, DT);
    let thief = guards_of(w)[1];
    w.players[0].x = w.enemies[thief].home_x - 2.0;
    w.players[0].gold = 50;
    guard_steps(w, 5.0);
    assert!(!w.enemies[thief].fleeing && w.enemies[thief].carried_gold == 0);
}

#[test]
#[ignore = "wartet auf RU10 (ruin_island)"]
fn besiegte_wache_ist_geraeumt_ohne_aggression() {
    let mut isl = ruin_island(guard_plan());
    let w = &mut isl.stages[0];
    step(w, None, DT);
    let g = guards_of(w);
    w.aggression = Some(0.0);
    w.enemies[g[0]].hp = 0.0;
    remove_dead_enemies(w);
    assert!(w.aggression == Some(0.0) && w.guards_cleared.len() == 1);
    guard_steps(w, 3.0);
    assert!(guards_of(w).len() <= 1, "geräumte Wache kommt wieder");
}

#[test]
#[ignore = "wartet auf RU10 (ruin_island)"]
fn geraeumtes_lager_bleibt_im_spielstand() {
    let mut isl = ruin_island(guard_plan());
    let w = &mut isl.stages[0];
    step(w, None, DT);
    let g = guards_of(w)[0];
    w.enemies[g].hp = 0.0;
    step(w, None, DT);
    assert_eq!(guards_of(w).len(), 1);
    // Speichern, Laden, Speichern ist gleich: `ToSave`/`FromIslandSave` liefert RU10.
    assert_eq!(guards_cleared_keys(w).len(), 1);
}

/// Burgfall: die Gegner der Welle verschwinden, die Wachen bleiben.
#[test]
#[ignore = "wartet auf RU10 (ruin_island)"]
fn burgfall_laesst_wachen_stehen() {
    let mut isl = ruin_island(guard_plan());
    let w = &mut isl.stages[0];
    step(w, None, DT);
    let at = w.hub_x + 30.0;
    spawn(w, "goblin", at);
    w.castle.hp = 0.0;
    step(w, None, DT);
    assert!(w.enemies.len() == 2 && guards_of(w).len() == 2);
}
