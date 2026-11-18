//! Traits und Angriffsrate der Gegner (Go: `enemies_traits_test.go`, K1.1, gegner.md § 2). Die Tests rufen
//! `step_enemies` direkt, damit Truppen und Spieler stehen bleiben, wo der Test sie hinstellt.

use k3c_data::enemies::KNOWN_TRAITS;
use k3c_rng::Rng;

use super::pools_tests::biome_for_depth;
use super::testutil::*;
use super::traits::{attack_rate, invulnerable_in, kite};
use super::waves::plan_wave;
use super::{BODY, step_enemies};
use crate::state::Troop;
use std::sync::Arc;

/// Go: `warriorAt` (mit Wertesatz der Welt, ohne `Step`).
fn warrior(w: &mut crate::state::World, x: f64) -> usize {
    let id = w.new_id();
    w.troops.push(Troop {
        id,
        kind: "warrior".into(),
        x,
        hp: 1000.0,
        max_hp: 1000.0,
        ..Troop::default()
    });
    w.troops.len() - 1
}

/// Go: `hitsOn`: die `hit`-Ereignisse auf die ID.
fn hits_on(w: &crate::state::World, id: i64) -> usize {
    super::events_of_type(w, "hit")
        .iter()
        .filter(|e| e.get("id").and_then(|v| v.as_i64()) == Some(id))
        .count()
}

/// Go: `stepFor` mit `kite` allein (die übrigen Schritte brauchen Ereignisse, RU19): so viele Takte wie Sekunden.
fn kite_for(w: &mut crate::state::World, ei: usize, seconds: f64) {
    let v = w.vals.clone().expect("Wertesatz");
    for _ in 0..(seconds / DT).round() as i64 {
        kite(w, &v, ei, DT);
        w.time += DT;
    }
}

/// (a) Flächenschlag: zwei Truppen im Radius 3 um das Ziel nehmen Schaden, eine außerhalb nicht; der zweite Schlag kommt
/// erst nach 4 s, dazwischen greift der Troll normal an.
#[test]
#[ignore = "wartet auf RU19 (unit_x)"]
fn gegner_flaechenschlag() {
    let mut w = quiet_world();
    let x = w.hub_x - 30.0;
    let e = spawn(&mut w, "caveTroll", x);
    let (target, near, out) = (
        warrior(&mut w, x + 2.0),
        warrior(&mut w, x + 4.5),
        warrior(&mut w, x + 6.0),
    );
    let mut aoe_at = Vec::new();
    for i in 0..(4.5 / DT) as i64 {
        let before = w.troops[near].hp;
        step_enemies(&mut w, None, DT);
        if w.troops[near].hp < before {
            aoe_at.push(i);
        }
    }
    let gap = (aoe_at[aoe_at.len() - 1] - aoe_at[0]) as f64 * DT;
    // Der Schlag fällt auf den nächsten Angriffs-Takt ab 4 s (Abklingzeit in ganzen Ticks).
    let tid = w.troops[target].id;
    assert!(
        aoe_at.len() == 2 && (4.0 - DT..=4.25).contains(&gap),
        "Flächenschläge {aoe_at:?}"
    );
    assert!(
        w.troops[out].hp == w.troops[out].max_hp && hits_on(&w, tid) == 5,
        "Troll bei {}",
        w.enemies[e].x
    );
}

/// (g) Zwei Spieler im Radius: beide nehmen Schaden.
#[test]
#[ignore = "wartet auf RU18.4 (preset_for)"]
fn gegner_flaechenschlag_zwei_spieler() {
    let mut w = quiet_world();
    let (p1, p2) = (add_player(&mut w), add_player(&mut w));
    let x = w.hub_x - 30.0;
    spawn(&mut w, "caveTroll", x);
    w.players[p1].x = x + 1.5;
    w.players[p2].x = x + 3.5;
    probe(&mut w, &[p1, p2]);
    step_enemies(&mut w, None, DT);
    assert!(was_hit(&w, p1) && was_hit(&w, p2));
}

/// (b) Ein Schwarm-Platz erzeugt `swarmSize` Gegner der Art am selben Portal.
#[test]
fn gegner_schwarm_platz() {
    let v = test_values();
    let mut b = biome_for_depth(2);
    b.enemies.portal = vec!["ratSwarm".to_string()];
    b.enemies.night.clear();
    let size = v.enemies["ratSwarm"].swarm_size as usize;
    let mut r = Rng::new("schwarm");
    let portals = [10.0, 90.0];
    let plan = plan_wave(&v, &b, 1, &mut r, &portals, false, 1.0);
    assert!(
        size == 4 && plan.len() >= 5 * size && plan.len().is_multiple_of(size),
        "Schwarmgröße {size}, {} Gegner",
        plan.len()
    );
    for (i, s) in plan.iter().enumerate() {
        let want = portals[(i / size) % 2];
        assert!(
            s.kind == "ratSwarm" && s.x == want,
            "Gegner {i}: {} am Portal {}, erwartet {want}",
            s.kind,
            s.x
        );
    }
}

/// (c) Phasen: alle 8 s ab Spawn für 2 s unverwundbar (Treffer ändern dann die HP nicht und melden kein `hit`; der
/// Treffer selbst läuft über `apply_damage_by`, RU19).
#[test]
fn gegner_phasen() {
    let mut w = bare_world();
    w.time = 5.0;
    let at = w.hub_x - 30.0;
    let e = spawn(&mut w, "mineGhost", at);
    let v = w.vals.clone().expect("Wertesatz");
    for (age, hurt) in [
        (1.0, true),
        (7.9, true),
        (8.1, false),
        (9.9, false),
        (10.1, true),
        (15.9, true),
        (16.5, false),
        (18.1, true),
    ] {
        let invulnerable = invulnerable_in(&v, 5.0 + age, &w.enemies[e]);
        assert_eq!(!invulnerable, hurt, "nach {age} s");
    }
}

/// (d) Kiting: Der Goblin Archer weicht einem Krieger unter 8 Units aus (nie über das Portal hinaus).
#[test]
fn gegner_kiting() {
    let mut w = bare_world();
    let x = w.hub_x - 30.0;
    let e = spawn(&mut w, "goblinArcher", x - 10.0);
    w.enemies[e].x = x;
    let t = warrior(&mut w, x + 5.0);
    kite_for(&mut w, e, 3.0);
    let dist = w.troops[t].x - w.enemies[e].x;
    let min = w.vals.as_ref().map_or(0.0, |v| v.enemies["goblinArcher"].kite_distance);
    assert!(
        dist >= min && dist <= 8.2 && w.enemies[e].x >= w.enemies[e].home_x,
        "Abstand {dist}"
    );
}

/// (d) Kiting am Rand: Am Portal bleibt der Archer stehen.
#[test]
fn gegner_kiting_am_portal() {
    let mut w = bare_world();
    let x = w.hub_x - 30.0;
    let e = spawn(&mut w, "goblinArcher", x);
    warrior(&mut w, x + 3.0);
    let v = w.vals.clone().expect("Wertesatz");
    assert!(kite(&mut w, &v, e, DT), "Ziel zu nah: der Archer rückt nicht vor");
    kite_for(&mut w, e, 2.0);
    assert_eq!(w.enemies[e].x, x);
}

/// (d) Kiting an der Mauer: Der Archer weicht nicht durch die Mauer aus, er bleibt davor stehen.
#[test]
fn gegner_kiting_an_der_mauer() {
    let mut w = bare_world();
    let wall_x = w.hub_x - 60.0;
    built_site(&mut w, "wall", wall_x);
    let e = spawn(&mut w, "goblinArcher", wall_x - 20.0);
    w.enemies[e].x = wall_x + 1.0;
    warrior(&mut w, wall_x + 4.0);
    kite_for(&mut w, e, 2.0);
    assert!(
        (w.enemies[e].x - (wall_x + BODY)).abs() <= 1e-9,
        "Archer bei {}, Mauer bei {wall_x}",
        w.enemies[e].x
    );
}

/// (e) Angriffsrate je Gegner: 2/s greift doppelt so oft an wie 1/s, ohne Eintrag 1/s. Der Zählversuch über 10 s
/// braucht `step_enemies` mit Ereignissen (RU19); hier die Abklingzeiten, die er prüft.
#[test]
fn gegner_angriffsrate() {
    let mut v = test_values();
    let base = v.enemies["skeleton"].clone();
    let (mut fast, mut none) = (base.clone(), base.clone());
    fast.attacks_per_second = 2.0;
    none.attacks_per_second = 0.0;
    v.enemies.insert("testFast".into(), fast);
    v.enemies.insert("testNone".into(), none);
    assert_eq!(base.attacks_per_second, 1.0);
    let cooldown = |k: &str| 1.0 / attack_rate(&v, k);
    assert_eq!(
        (cooldown("skeleton"), cooldown("testFast"), cooldown("testNone")),
        (1.0, 0.5, 1.0)
    );
    assert_eq!(attack_rate(&v, "unbekannt"), 1.0);
    for (kind, d) in &v.enemies {
        if kind != "testFast" && kind != "testNone" {
            assert!(
                d.attacks_per_second > 0.0,
                "{kind} ohne attacksPerSecond in data/enemies.json"
            );
        }
    }
}

/// (e) Angriffe in 10 s: 2/s trifft doppelt so oft wie 1/s.
#[test]
#[ignore = "wartet auf RU19 (unit_x)"]
fn gegner_angriffsrate_zaehlt_treffer() {
    let mut w = quiet_world();
    let mut arc = (*w.vals.clone().expect("Wertesatz")).clone();
    let mut fast = arc.enemies["skeleton"].clone();
    fast.attacks_per_second = 2.0;
    arc.enemies.insert("testFast".into(), fast);
    w.vals = Some(Arc::new(arc));
    let mut ids = Vec::new();
    for (i, kind) in ["skeleton", "testFast"].iter().enumerate() {
        let x = w.hub_x - 60.0 + i as f64 * 15.0;
        spawn(&mut w, kind, x);
        let t = warrior(&mut w, x + 1.0);
        ids.push(w.troops[t].id);
    }
    step_for(&mut w, 10.0);
    let (slow, fast) = (hits_on(&w, ids[0]), hits_on(&w, ids[1]));
    assert!(
        fast + 2 >= 2 * slow && fast <= 2 * slow,
        "Angriffe in 10 s: {slow} und {fast}"
    );
}

/// (f) Daten: Alle Traits sind bekannt, `pack` und `stealsWood` sind gestrichen. Das Laden eines unbekannten Traits
/// prüft `k3c-data` (`enemies::load`).
#[test]
fn gegner_traits_in_daten() {
    let v = test_values();
    for (kind, d) in &v.enemies {
        for tr in &d.traits {
            assert!(
                KNOWN_TRAITS.contains(&tr.as_str()) && tr != "pack" && tr != "stealsWood",
                "{kind}: Trait {tr}"
            );
        }
    }
    let raw = r#"{"wolf": {"tier": "standard", "traits": ["pack"]}}"#;
    assert!(k3c_data::enemies::load(raw).is_err(), "unbekanntes Trait geladen");
}

/// (h) Determinismus: derselbe Seed zweimal ergibt dieselbe Welle (Schwarm, Würfe in fester Reihenfolge). Der Lauf über
/// 40 s mit Flächenschlag, Phasen und Kiting braucht `create_world` und `step` (RU14).
#[test]
fn gegner_wellenplan_deterministisch() {
    let v = test_values();
    let run = || {
        let mut r = Rng::new("k1-traits");
        plan_wave(&v, &biome_for_depth(2), 3, &mut r, &[10.0, 50.0], true, 1.0)
    };
    assert_eq!(run(), run());
}

#[test]
#[ignore = "wartet auf RU18.4 (preset_for)"]
fn gegner_traits_deterministisch() {
    let scenario = || {
        let mut w = quiet_world();
        add_player(&mut w);
        add_player(&mut w);
        for kind in ["caveTroll", "mineGhost", "goblinArcher"] {
            let at = w.hub_x - 12.0;
            spawn(&mut w, kind, at);
        }
        step_for(&mut w, 40.0);
        serde_json::to_string(&(w.enemies, w.players)).expect("JSON")
    };
    assert_eq!(scenario(), scenario(), "zwei Läufe mit demselben Seed weichen ab");
}
