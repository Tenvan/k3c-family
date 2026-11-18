//! Tests zu `hub_level_test.go` (W1.1, B-112/AC-01, AC-03, Sprint W1 AC-01; GEB17.4, B-592/AC-02): Hub-Ausbau an der Burg
//! und Sperre nach Hub-Stufe. Werte aus data/. Die Tests, die `step` rechnen, warten auf RU18.4 und RU18.5
//! (`merchant_dawn`, `step_passives`, `speed_of`). Dazu eigene Tests der Funktionen ohne Schritt.

use k3c_data::hub::Stock;

use super::level::{
    add_cost, finish_upgrade, hub_gate, hub_step_of, hub_steps, level_of, new_hub_site, next_level, pay_upgrade,
    site_hub_level, step_upgrade, upgrade_payable,
};
use super::testutil::{hub, make_island};
use crate::economy::site_payable;
use crate::economy::view::material_of;
use crate::enemies::testutil::run;
use crate::island::CreateOptions;
use crate::island::testutil::{on_stage, test_data};
use crate::state::World;
use crate::world::testutil::{day_world, pay_at, quiet_world, run_until, site_line};
use crate::world::{add_player, hub_of, vals};

/// Go: `buildHubStep`: zahlt den Ausbauschritt `st` genau (Gold von `pi`, Material im Vorrat) und lässt ihn bauen; die
/// Bauzeit muss der aus den Daten entsprechen.
fn build_hub_step(w: &mut World, pi: usize, st: &k3c_data::buildings::LevelData) {
    const SPARE: i64 = 7;
    w.stock = material_of(&st.cost);
    w.stock.wood += SPARE;
    w.players[pi].gold = st.cost.gold;
    let interval = vals(w).economy.pay_interval_seconds;
    let seconds = 1.2 * st.cost.gold as f64 * interval + 5.0; // Takt auf ganze Ticks gerundet
    let x = w.castle.x;
    let waiting = |w: &World| w.hub_site.as_ref().is_some_and(|h| h.upgrade == "waitingWorker");
    assert!(
        pay_at(w, &[pi], x, seconds, waiting),
        "Schritt nicht bezahlt: {:?}, Vorrat {:?}",
        w.hub_site,
        w.stock
    );
    let want = Stock {
        wood: SPARE,
        ..Stock::default()
    };
    assert!(
        w.players[pi].gold == 0 && w.stock == want,
        "Gold übrig {}, Vorrat {:?}; erwartet genau {:?} abgebucht",
        w.players[pi].gold,
        w.stock,
        st.cost
    );
    let (level, step) = (w.hub_level, w.hub_step);
    w.troops[0].x = w.castle.x; // ohne Weg: gemessen wird nur die Bauzeit
    let took = run_until(w, 2.0 * st.build_seconds + 1.0, |w| {
        w.hub_level != level || w.hub_step != step
    });
    let dt = crate::world::testutil::DT;
    assert!(
        took >= st.build_seconds - dt && took <= st.build_seconds + 0.5,
        "Schritt dauerte {took:.2} s, laut Daten {} s",
        st.build_seconds
    );
}

/// GEB17.4 (B-592/AC-02): Jede Hub-Stufe wird in ihren Ausbauschritten bezahlt und gebaut, Kosten samt Material-Tor und
/// Bauzeit aus data/hub.json; erst der letzte Schritt hebt die Hub-Stufe.
#[test]
#[ignore = "wartet auf RU19 (pure)"]
fn hub_ausbau_stufe2_bis5() {
    let mut w = day_world();
    let p = add_player(&mut w);
    let h = hub();
    for n in 2..=h.levels.len() as i64 {
        let steps = h.levels[(n - 1) as usize].steps.len() as i64;
        for step in 1..=steps {
            assert!(
                w.hub_level == n - 1 && w.hub_step == step,
                "vor Stufe {n} Schritt {step}: Hub-Stufe {} Schritt {}",
                w.hub_level,
                w.hub_step
            );
            build_hub_step(&mut w, p, &hub_step_of(&h.levels[(n - 1) as usize], step));
        }
        assert!(
            w.hub_level == n && w.hub_step == 1,
            "nach Stufe {n}: Hub-Stufe {} Schritt {}",
            w.hub_level,
            w.hub_step
        );
    }
    let hs = w.hub_site.clone().expect("Hub-Platz");
    assert!(
        hs.state == "built" && !upgrade_payable(&w, &hs),
        "Hub-Stufe {}: weiter ausbaubar {hs:?}",
        w.hub_level
    );
}

#[test]
#[ignore = "wartet auf RU19 (pure)"]
fn hub_ausbau_wartet_auf_material() {
    let mut w = day_world();
    let p = add_player(&mut w);
    let lv = hub_step_of(&hub().levels[1], 1);
    w.players[p].gold = lv.cost.gold;
    let x = w.castle.x;
    pay_at(&mut w, &[p], x, 30.0, |w| {
        w.hub_site.as_ref().is_some_and(|h| !h.upgrade.is_empty())
    });
    run(&mut w, 5.0);
    let up = w.hub_site.as_ref().map(|h| h.upgrade.clone()).unwrap_or_default();
    assert!(
        up == "waitingMaterial" && w.hub_step == 1,
        "ohne Material: {up:?}, Schritt {}",
        w.hub_step
    );
    w.stock = material_of(&lv.cost);
    assert!(
        run_until(&mut w, 2.0 * lv.build_seconds + 1.0, |w| w.hub_step == 2) >= 0.0,
        "mit Material nicht ausgebaut"
    );
}

/// Während einer Welle bezahlt: Der Bauer baut erst nach der Gefahr.
#[test]
#[ignore = "wartet auf RU19 (pure)"]
fn hub_ausbau_nach_der_gefahr() {
    let mut w = day_world();
    let p = add_player(&mut w);
    let lv = hub_step_of(&hub().levels[1], 1);
    w.stock = material_of(&lv.cost);
    w.players[p].gold = lv.cost.gold;
    let w_end = w.width_units - 1.0;
    let ei = crate::enemies::testutil::spawn(&mut w, "goblin", w_end);
    w.enemies[ei].stun = f64::INFINITY;
    let x = w.castle.x;
    let paid = pay_at(&mut w, &[p], x, 30.0, |w| {
        w.hub_site.as_ref().is_some_and(|h| h.upgrade == "waitingWorker")
    });
    assert!(paid, "bei Gefahr nicht bezahlbar: {:?}", w.hub_site);
    run(&mut w, 2.0 * lv.build_seconds);
    let progress = w.hub_site.as_ref().map_or(0.0, |h| h.build_progress);
    assert!(
        w.hub_step == 1 && progress == 0.0,
        "bei Gefahr gebaut: Schritt {}, Fortschritt {progress}",
        w.hub_step
    );
    w.enemies.clear();
    assert!(
        run_until(&mut w, 2.0 * lv.build_seconds + 1.0, |w| w.hub_step == 2) >= 0.0,
        "nach der Gefahr nicht ausgebaut"
    );
}

#[test]
#[ignore = "wartet auf RU19 (pure)"]
fn hub_ausbau_zwei_spieler_zahlen_gemeinsam() {
    let mut w = day_world();
    let (p0, p1) = (add_player(&mut w), add_player(&mut w));
    let lv = hub_step_of(&hub().levels[1], 1);
    let half = lv.cost.gold / 2;
    w.players[p0].gold = half;
    w.players[p1].gold = lv.cost.gold - half;
    let x = w.castle.x;
    let started = |w: &World| w.hub_site.as_ref().is_some_and(|h| !h.upgrade.is_empty());
    assert!(
        pay_at(&mut w, &[p0, p1], x, 30.0, started),
        "gemeinsam nicht bezahlt: {:?}",
        w.hub_site
    );
    let paid = w.hub_site.as_ref().map_or(0, |h| h.upgrade_paid);
    assert!(
        w.players[p0].gold == 0 && w.players[p1].gold == 0 && paid == lv.cost.gold,
        "Gold übrig {}/{}, bezahlt {paid} von {}",
        w.players[p0].gold,
        w.players[p1].gold,
        lv.cost.gold
    );
}

/// Ein Hub-Platz ist erst ab seiner Hub-Stufe aus data/hub.json bezahlbar (Treppe, Kaserne ab 2, Schmiede ab 3 …).
#[test]
fn hub_platz_erst_ab_hub_stufe() {
    let data = test_data();
    let mut isl = make_island("sperre", &[0, 1], CreateOptions::default()).unwrap_or_else(|e| panic!("{e}"));
    let levels = hub().levels.len() as i64;
    for i in 0..isl.stages.len() {
        on_stage(&mut isl, &data, i, |w, r| {
            let v = vals(w);
            for level in 1..=levels {
                w.hub_level = level;
                for s in &w.sites {
                    if site_line(w, s).is_some() {
                        continue;
                    }
                    let want = level >= site_hub_level(w, s);
                    assert_eq!(
                        site_payable(w, &v, Some(r), s),
                        want,
                        "Tiefe {}, Hub-Stufe {level}: {} bezahlbar = {}",
                        w.biome.depth,
                        s.kind,
                        !want
                    );
                }
            }
        });
    }
    let w = &isl.stages[0];
    let want = [
        ("stairsDown", 2),
        ("storage", 2),
        ("barracks", 2),
        ("tavern", 2),
        ("smithy", 3),
        ("armory", 4),
        ("workshop", 1),
        ("bowyer", 1),
    ];
    for (kind, level) in want {
        for s in w.sites.iter().filter(|s| s.kind == kind) {
            let got = site_hub_level(w, s);
            assert_eq!(got, level, "{kind}: Hub-Stufe {got}, erwartet {level}");
        }
    }
}

// Eigene Tests der Funktionen ohne `step` (Go prüft sie nur über die Schritte oben).

#[test]
fn new_hub_site_ist_die_gebaute_burg() {
    let w = quiet_world();
    let s = new_hub_site(&w);
    assert_eq!(
        (s.id, s.kind.as_str(), s.x, s.state.as_str(), s.build_progress),
        (w.castle.id, "castle", w.castle.x, "built", 1.0)
    );
    assert_eq!(w.hub_site.as_ref(), Some(&s), "create_world setzt den Zahlplatz");
}

#[test]
fn level_of_hub_ist_die_hub_stufe_sonst_mindestens_eins() {
    let mut w = quiet_world();
    w.hub_level = 3;
    let hs = w.hub_site.clone().expect("Hub-Platz");
    assert_eq!(level_of(&w, &hs), 3);
    let s = w
        .sites
        .iter()
        .find(|s| s.kind == "workshop")
        .cloned()
        .expect("workshop");
    assert_eq!(level_of(&w, &s), 1, "Stufe 0 zählt als 1");
}

#[test]
fn hub_steps_und_gate_folgen_der_stufe() {
    let mut w = quiet_world();
    let h = hub_of(&w).clone();
    assert_eq!(hub_steps(&w), 3, "Stufe 1 auf 2: drei Schritte");
    w.hub_level = h.levels.len() as i64;
    assert_eq!(hub_steps(&w), 0, "letzte Stufe");
    w.hub_level = 1;
    w.hub_step = 3;
    assert_eq!(hub_gate(&w, None), h.levels[1].steps[2].gate);
    w.hub_step = 1;
    assert_eq!(
        hub_gate(&w, None),
        Default::default(),
        "nur der letzte Schritt hat ein Tor"
    );
    w.hub_level = h.levels.len() as i64;
    assert_eq!(hub_gate(&w, None), Default::default(), "letzte Stufe ohne Tor");
}

#[test]
fn hub_step_of_begrenzt_den_schritt_und_bucht_das_tor_ein() {
    let h = hub();
    let lv = &h.levels[1];
    let st = hub_step_of(lv, 3);
    assert_eq!(st.cost, add_cost(&lv.steps[2].cost, &lv.steps[2].gate));
    assert_eq!(hub_step_of(lv, 99).cost, st.cost, "zu großer Schritt: der letzte");
    assert_eq!(
        hub_step_of(lv, 0).cost,
        add_cost(&lv.steps[0].cost, &lv.steps[0].gate),
        "Schritt 0 zählt als 1"
    );
    assert_eq!(
        &hub_step_of(&h.levels[0], 5),
        &h.levels[0],
        "ohne Schritte die Stufe selbst"
    );
}

#[test]
fn ausbau_zahlen_buchen_und_abschliessen_am_hub() {
    let mut w = quiet_world();
    let id = w.castle.id;
    let hs = w.hub_site.clone().expect("Hub-Platz");
    let step1 = next_level(&w, None, &hs).expect("Schritt 1");
    for _ in 0..step1.cost.gold {
        pay_upgrade(&mut w, None, id);
    }
    assert_eq!(w.hub_site.as_ref().map(|h| h.upgrade.as_str()), Some("waitingMaterial"));
    step_upgrade(&mut w, None, id);
    assert_eq!(
        w.hub_site.as_ref().map(|h| h.upgrade.as_str()),
        Some("waitingMaterial"),
        "ohne Material"
    );
    w.stock = material_of(&step1.cost);
    step_upgrade(&mut w, None, id);
    let h = w.hub_site.as_ref().expect("Hub-Platz");
    assert_eq!((h.upgrade.as_str(), h.build_progress), ("waitingWorker", 0.0));
    assert_eq!(w.stock, Stock::default(), "Material abgebucht");
    finish_upgrade(&mut w, id);
    assert_eq!((w.hub_level, w.hub_step), (1, 2), "Schritt 1 hebt nur den Schritt");
    assert!(w.events.is_empty(), "ohne Ereignis");
    for _ in 0..2 {
        finish_upgrade(&mut w, id);
    }
    assert_eq!((w.hub_level, w.hub_step), (2, 1), "der letzte Schritt hebt die Stufe");
    let ev = w.events.last().expect("Ereignis upgraded");
    assert_eq!(ev.get("type").and_then(|v| v.as_str()), Some("upgraded"));
    assert_eq!(ev.get("level").and_then(|v| v.as_i64()), Some(2));
}
