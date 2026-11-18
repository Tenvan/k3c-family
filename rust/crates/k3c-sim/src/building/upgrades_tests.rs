//! Tests zu `upgrades.rs` (Go: `building_upgrades_test.go`, W4.3b, B-122, B-014; Sprint W4 AC-04, AC-06): Elite-Upgrade in der
//! Schmiede und Rüstung in der Rüstkammer, Kosten und Werte aus data/troops.json und data/buildings.json. Die Tests über
//! `step` und `create_world` warten auf RU18 (Zahlplatz `new_hub_site`, Schritte der Wirtschaft); die Tests von
//! `levels_data`, `site_level` und `sites` stehen in `upgrades_sites_tests.rs`.
#![allow(clippy::unimplemented)] // Rumpf des Tests, der Hilfen anderer Tests (RU11) braucht

use k3c_data::hub::Stock;

use super::testutil::*;
use super::upgrades::{armor_bonus, armor_stage, armored_hp, place_payable, step_crafts};
use crate::state::{PlayerCommand, World};
use crate::world::vals;

/// Go: `eliteSpot` ist der Ort des Elite-Krieger-Angebots an der Schmiede `s` und sein Index in buildings.smithy.offers.
pub(super) fn elite_spot(w: &World, s: usize) -> (f64, usize) {
    let site = &w.sites[s];
    let v = vals(w);
    for (i, o) in v.buildings[&site.kind].offers.iter().enumerate() {
        if o.kind == "eliteWarrior" {
            return (site.x + o.dx, i);
        }
    }
    panic!("Schmiede ohne Elite-Krieger-Angebot");
}

/// Go: `wantElite`: Ein Spieler zahlt am Zahlziel `at` der Schmiede das Gold für `kind`, erst danach kommt das Material in
/// den Vorrat. Der nächste Kämpfer der Ausgangsfigur zur Schmiede wird nach `craft_seconds` Elite, der fernere nicht; Kosten
/// und HP laut Daten.
fn want_elite(kind: &str, at: impl Fn(&World, usize) -> f64) -> (World, usize) {
    let mut w = quiet_world();
    let p = add_player(&mut w);
    let smithy = build_site(&mut w, "smithy");
    let v = vals(&w);
    let (from, cost) = (v.troops[kind].upgrade_from.clone(), v.troops[kind].cost.clone());
    let sx = w.sites[smithy].x;
    let hub = w.hub_x;
    let near = fighter_at(&mut w, &from, sx + 30.0);
    let far = fighter_at(&mut w, &from, hub - 60.0);
    let gold = w.players[p].gold;
    let spot = at(&w, smithy);
    pay_at(
        &mut w,
        &[p],
        spot,
        1.5 * cost.gold as f64 * v.economy.pay_interval_seconds,
        |_| false,
    );
    assert_eq!(
        gold - w.players[p].gold,
        cost.gold,
        "{kind}: Spieler zahlte {} Gold",
        gold - w.players[p].gold
    );
    w.stock = Stock {
        stone: 1000,
        copper: 1000,
        ..Stock::default()
    };
    let done = |w: &World| w.troops[near].kind == kind;
    let took = run_until(&mut w, 60.0, &done);
    let want = v.buildings["smithy"].craft_seconds.as_ref().map_or(0.0, |m| m[kind]);
    assert!(
        (took - want).abs() <= 2.0 * DT,
        "{kind}: Upgrade nach {took:.2} s, laut Daten {want} s"
    );
    assert!(
        w.stock.stone == 1000 - cost.stone && w.stock.copper == 1000 - cost.copper,
        "{kind}: Vorrat {:?}, laut Daten {cost:?} abgebucht",
        w.stock
    );
    let hp = v.troops[kind].hp;
    assert!(
        w.troops[far].kind == from && w.troops[near].hp == hp && w.troops[near].max_hp == hp,
        "{kind}: fernerer ist {}, Elite-HP {}/{}, laut Daten {hp}",
        w.troops[far].kind,
        w.troops[near].hp,
        w.troops[near].max_hp
    );
    let keep = w.troops.remove(near);
    w.troops = vec![keep];
    (w, 0)
}

#[test]
#[ignore = "wartet auf RU19 (pure)"]
fn upgrade_elite_bogenschuetze() {
    let (mut w, _) = want_elite("eliteArcher", |w, s| w.sites[s].x);
    // Start in Reichweite, vor der Schmiede: Sonst greift das Skelett sie an (gegner.md § 4, B-325).
    let hub = w.hub_x;
    spawn(&mut w, "skeleton", hub + 20.0);
    let mut tm = 0.0;
    while tm < 20.0 && w.projectiles.is_empty() {
        crate::enemies::testutil::step(&mut w, None, DT);
        tm += DT;
    }
    let want = vals(&w).troops["eliteArcher"].damage;
    assert!(
        w.projectiles.first().is_some_and(|p| p.damage == want),
        "Elite-Bogenschütze schießt nicht mit Schaden {want}"
    );
}

#[test]
#[ignore = "wartet auf RU19 (pure)"]
fn upgrade_elite_krieger() {
    let (_w, _) = want_elite("eliteWarrior", |w, s| elite_spot(w, s).0);
    unimplemented!("RU18: Testhilfen warriorPostOf und firstHit (unit_warrior_test.go)")
}

/// (d) Zwei Spieler zahlen gleichzeitig am Schmiede-Platz (Elite-Bogenschütze) und am Elite-Krieger-Angebot; wer vor dem
/// vollen Preis loslässt, bekommt die Münzen zurück.
#[test]
#[ignore = "wartet auf RU19 (pure)"]
fn upgrade_zwei_spieler_an_beiden_schmiede_zielen() {
    let mut w = quiet_world();
    let (p1, p2) = (add_player(&mut w), add_player(&mut w));
    let s = build_site(&mut w, "smithy");
    let (spot, idx) = elite_spot(&w, s);
    let (g1, g2) = (w.players[p1].gold, w.players[p2].gold);
    let sx = w.sites[s].x;
    let cmds = [
        PlayerCommand {
            pay: true,
            ..Default::default()
        },
        PlayerCommand {
            pay: true,
            ..Default::default()
        },
    ];
    let mut tm = 0.0;
    while tm < 20.0 {
        w.players[p1].x = sx;
        w.players[p2].x = spot;
        crate::world::step(&mut w, &cmds, None, DT);
        tm += DT;
    }
    let v = vals(&w);
    let (archer, warrior) = (v.troops["eliteArcher"].cost.gold, v.troops["eliteWarrior"].cost.gold);
    assert!(
        w.sites[s].bow_paid_gold == archer
            && w.sites[s].offer_paid[idx] == warrior
            && g1 - w.players[p1].gold == archer
            && g2 - w.players[p2].gold == warrior,
        "Elite-Bogenschütze {}/{archer}, Elite-Krieger {}/{warrior}",
        w.sites[s].bow_paid_gold,
        w.sites[s].offer_paid[idx]
    );
    w.sites[s].offer_paid[idx] = 0;
    let g2 = w.players[p2].gold;
    let mut paid = 0;
    let mut only2 = vec![PlayerCommand::default(); w.players.len()];
    only2[w.players[p2].index as usize].pay = true;
    let mut tm = 0.0;
    while tm < 3.0 * v.economy.pay_interval_seconds {
        w.players[p2].x = spot;
        crate::world::step(&mut w, &only2, None, DT);
        paid = paid.max(w.sites[s].offer_paid[idx]);
        tm += DT;
    }
    crate::enemies::testutil::step(&mut w, None, DT);
    assert!(
        paid != 0 && w.sites[s].offer_paid[idx] == 0 && w.players[p2].gold == g2,
        "gezahlt {paid}, nach dem Loslassen {} Gold im Angebot, Spieler hat {} statt {g2}",
        w.sites[s].offer_paid[idx],
        w.players[p2].gold
    );
}

/// armorWorld: gebaute Rüstkammer bei Hub-Stufe `level`, ein Spieler mit Gold, Eisen im Vorrat.
fn armor_world(level: i64) -> (World, usize, usize) {
    let mut w = quiet_world();
    w.hub_level = level;
    w.stock = Stock {
        iron: 1000,
        ..Stock::default()
    };
    let p = add_player(&mut w);
    let s = build_site(&mut w, "armory");
    (w, p, s)
}

/// Go: `buyArmor` zahlt die nächste Rüstungsstufe und wartet die Herstellung ab; `false`, wenn sie nicht zustande kommt.
fn buy_armor(w: &mut World, p: usize, s: usize) -> bool {
    let level = w.armor_level;
    let v = vals(w);
    w.players[p].gold = v.economy.purse.max_gold;
    let sx = w.sites[s].x;
    pay_at(w, &[p], sx, 60.0, |w| w.sites[s].craft_done > 0.0);
    let secs = v.buildings["armory"].craft_seconds.as_ref().map_or(0.0, |m| m["armor"]);
    let done = |w: &World| w.armor_level > level;
    run_until(w, 2.0 * secs, &done) > 0.0
}

/// (b) Stufe 1 ab Hub-Stufe 4, Stufe 2 ab 5; Zuschlag auf MaxHP und HP ohne Vollheilung, nur Kämpfer, auch neue.
#[test]
#[ignore = "wartet auf RU19 (pure)"]
fn armor_zwei_stufen_nur_kaempfer() {
    let v = vals(&bare_world());
    let stages = v.buildings["armory"].armor.clone();
    assert_eq!(
        stages.len(),
        2,
        "{} Rüstungsstufen in den Daten, erwartet 2",
        stages.len()
    );
    let (mut w, p, s) = armor_world(stages[0].hub_level - 1);
    let payable = |w: &World, s: usize| crate::economy::site_payable(w, &v, None, &w.sites[s]);
    assert!(!payable(&w, s), "Rüstung bei Hub-Stufe {} bezahlbar", w.hub_level);
    w.hub_level = stages[0].hub_level;
    let hub = w.hub_x;
    let warrior = fighter_at(&mut w, "warrior", hub);
    w.troops[warrior].hp = 50.0;
    let peasant = crate::unit::spawn_vagrant(&mut w, hub, hub);
    let php = v.troops["peasant"].hp;
    w.troops[peasant].kind = "peasant".into();
    w.troops[peasant].hp = php;
    w.troops[peasant].max_hp = php;
    assert!(payable(&w, s), "Rüstung bei Hub-Stufe {} nicht bezahlbar", w.hub_level);
    let iron = w.stock.iron;
    assert!(buy_armor(&mut w, p, s), "Stufe 1 nicht hergestellt: {:?}", w.sites[s]);
    let spent = v.economy.purse.max_gold - w.players[p].gold;
    assert!(
        spent == stages[0].cost.gold && iron - w.stock.iron == stages[0].cost.iron,
        "Stufe 1 kostete {spent} Gold"
    );
    let base = v.troops["warrior"].hp;
    let gain = base * stages[0].hp_bonus;
    assert!(
        w.troops[warrior].max_hp == base + gain && w.troops[warrior].hp == 50.0 + gain,
        "Krieger {}/{}, erwartet {}/{} (ohne Vollheilung)",
        w.troops[warrior].hp,
        w.troops[warrior].max_hp,
        50.0 + gain,
        base + gain
    );
    assert_eq!(w.troops[peasant].max_hp, php, "Bauer mit Rüstung");
    let fresh = fighter_at(&mut w, "archer", hub);
    assert_eq!(
        w.troops[fresh].max_hp,
        v.troops["archer"].hp * (1.0 + stages[0].hp_bonus),
        "neuer Bogenschütze"
    );
    assert!(!payable(&w, s), "Stufe 2 bei Hub-Stufe {} bezahlbar", w.hub_level);
    w.hub_level = stages[1].hub_level;
    let ok = buy_armor(&mut w, p, s);
    assert!(
        ok && w.troops[warrior].max_hp == base * (1.0 + stages[1].hp_bonus) && !payable(&w, s),
        "Stufe 2: Stufe {}, Krieger MaxHP {}",
        w.armor_level,
        w.troops[warrior].max_hp
    );
}

/// Eigener Test (ohne Go-Gegenstück, läuft ohne `create_world`): Die Rüstungsstufe der Rüstkammer wächst mit der
/// Herstellung, nur Kämpfer bekommen den Zuschlag auf `max_hp` und `hp` ohne Vollheilung.
#[test]
fn rustkammer_stufe_wirkt_auf_kaempfer() {
    let mut w = bare_world();
    let v = vals(&w);
    let stages = v.buildings["armory"].armor.clone();
    let hub = w.hub_x;
    let si = built_site(&mut w, "armory", hub + 80.0);
    w.hub_level = stages[0].hub_level - 1;
    assert!(
        !place_payable(&w, &v, None, &w.sites[si]),
        "Rüstung unter der Hub-Stufe bezahlbar"
    );
    w.hub_level = stages[0].hub_level;
    assert!(
        place_payable(&w, &v, None, &w.sites[si]),
        "Rüstung auf der Hub-Stufe nicht bezahlbar"
    );
    let warrior = fighter_at(&mut w, "warrior", hub);
    w.troops[warrior].hp = 50.0;
    let peasant = crate::unit::spawn_vagrant(&mut w, hub, hub);
    w.troops[peasant].kind = "peasant".into();
    let php = v.troops["peasant"].hp;
    w.troops[peasant].max_hp = php;
    w.sites[si].bow_paid_gold = stages[0].cost.gold;
    w.stock = Stock {
        wood: 1000,
        stone: 1000,
        copper: 1000,
        iron: 1000,
        crystal: 1000,
    };
    let stock = w.stock.clone();
    step_crafts(&mut w, &v, None);
    assert!(
        w.sites[si].craft_done > 0.0 && w.sites[si].bow_paid_gold == 0,
        "Herstellung nicht begonnen"
    );
    assert_eq!(
        stock.iron - w.stock.iron,
        stages[0].cost.iron,
        "Material nicht abgebucht"
    );
    assert!(
        !place_payable(&w, &v, None, &w.sites[si]),
        "Rüstung bezahlbar, solange eine entsteht"
    );
    assert_eq!(w.armor_level, 0, "Stufe vor Ablauf der Zeit");
    w.time = w.sites[si].craft_done;
    step_crafts(&mut w, &v, None);
    assert_eq!(w.armor_level, 1);
    assert_eq!(w.sites[si].craft_done, 0.0);
    let base = v.troops["warrior"].hp;
    let gain = base * stages[0].hp_bonus;
    assert_eq!(w.troops[warrior].max_hp, base + gain);
    assert_eq!(w.troops[warrior].hp, 50.0 + gain, "ohne Vollheilung");
    assert_eq!(w.troops[peasant].max_hp, php, "Bauer bekommt keine Rüstung");
    assert_eq!(armor_bonus(&w, &v), stages[0].hp_bonus);
    assert_eq!(
        armored_hp(&w, "archer"),
        v.troops["archer"].hp * (1.0 + stages[0].hp_bonus),
        "neuer Kämpfer mit Rüstung"
    );
}

/// Eigener Test: `armor_stage` liefert die nächste Stufe der Daten, nach der letzten nichts; `armor_bonus` folgt der
/// erreichten Stufe.
#[test]
fn rustkammer_stufen_folge() {
    let mut w = bare_world();
    let v = vals(&w);
    let stages = v.buildings["armory"].armor.clone();
    assert_eq!(armor_bonus(&w, &v), 0.0, "ohne Rüstung");
    for (level, stage) in stages.iter().enumerate() {
        w.armor_level = level as i64;
        assert_eq!(armor_stage(&w, &v, None).map(|st| st.hub_level), Some(stage.hub_level));
    }
    w.armor_level = stages.len() as i64;
    assert!(armor_stage(&w, &v, None).is_none(), "alle Stufen erreicht");
    assert_eq!(armor_bonus(&w, &v), stages[stages.len() - 1].hp_bonus);
}
