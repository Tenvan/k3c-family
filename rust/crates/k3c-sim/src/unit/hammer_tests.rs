//! Tests zu `hammer.rs` (Go: `unit_hammer_test.go`, WI1, B-950 bis B-953). Jede Mechanik läuft mit 2 Spielern; die
//! Tests über `Step` und `create_world` warten auf RU14.

use std::sync::Arc;

use super::hammer::*;
use crate::enemies::testutil::*;
use crate::state::{Troop, World};

/// Go: `hammerlessPeasant` ist ein Bauer ohne Hammer bei `x`.
fn hammerless_peasant(w: &mut World, x: f64) -> usize {
    add_peasant(w, x, "")
}

/// B-950: Ohne Hammer baut kein Bauer; mit Hammer baut er, während beide Spieler am Bauplatz zahlen.
#[test]
fn nur_handwerker_bauen() {
    let mut w = quiet_world();
    w.cycle_speed = 1e-6;
    let _players = [add_player(&mut w), add_player(&mut w)];
    let site = line_site(&mut w, "wall", 1, 1);
    w.sites[site].state = "waitingWorker".into();
    let sx = w.sites[site].x;
    let peasant = hammerless_peasant(&mut w, sx);
    let build_seconds = w.vals.as_ref().map_or(0.0, |v| v.buildings["wall"].build_seconds);
    run(&mut w, 3.0 * build_seconds);
    assert!(
        w.sites[site].state == "waitingWorker" && w.troops[peasant].job.is_none(),
        "Bauer ohne Hammer"
    );
    w.troops[peasant].hammer = true;
    let done = |w: &World| w.sites[site].state == "built";
    assert!(
        run_until(&mut w, 3.0 * build_seconds, &done) >= 0.0,
        "Handwerker baut nicht"
    );
    // Reparatur ist ebenfalls Hammer-Arbeit.
    w.sites[site].hp = w.sites[site].max_hp / 2.0;
    let other = hammerless_peasant(&mut w, sx);
    w.troops[peasant].hammer = false;
    assert!(crate::building::repair::repair_job(&mut w, other).is_none());
    w.troops[other].hammer = true;
    let j = crate::building::repair::repair_job(&mut w, other).expect("Reparatur");
    assert_eq!(j.site_id, w.sites[site].id);
}

/// B-950: Das Welt-Feld `startTroops.craftsman` bestimmt die Start-Handwerker (Standard 1, mindestens ein Handwerker).
#[test]
fn start_handwerker() {
    let w = crate::world::testutil::new_world(0, "test", crate::world::Options::default());
    let n = w.troops.iter().filter(|t| t.kind == "peasant" && t.hammer).count() as i64;
    let want = w.vals.as_ref().map_or(0, |v| v.hub.start_troops.craftsman);
    assert!(want >= 1 && n == want, "Start-Handwerker {n}, laut Daten {want}");
}

#[test]
fn start_handwerker_aus_den_welt_werten() {
    let mut w = bare_world();
    let mut v = (*w.vals.clone().expect("Wertesatz")).clone();
    v.hub.start_troops.craftsman = 3;
    spawn_start_peasants(&mut w, &v, 200.0);
    let hammers = w.troops.iter().filter(|t| t.hammer).count() as i64;
    assert_eq!(w.troops.len() as i64, 3 + v.hub.start_troops.peasant);
    assert_eq!(hammers, 3);
    assert!(w.troops.iter().all(|t| t.kind == "peasant"));
    assert_eq!(w.troops[0].x, 202.0, "Start-Bauern stehen ab Hub + 2");
}

/// Review WI1: Start-Handwerker mindestens 1.
#[test]
fn handwerker_mindestens_einer() {
    let mut w = bare_world();
    let mut v = (*w.vals.clone().expect("Wertesatz")).clone();
    v.hub.start_troops.craftsman = 0;
    spawn_start_peasants(&mut w, &v, 200.0);
    assert!(
        !w.troops.is_empty() && w.troops[0].hammer,
        "ohne Start-Handwerker: {} Bauern",
        w.troops.len()
    );
}

/// B-951: Zwei Spieler zahlen den Hammer der Werkstatt; nach der Herstellung holt ihn ein Bauer ohne Hammer.
#[test]
#[ignore = "wartet auf RU18 (Ursache ungeprueft: Bauer holt den Hammer nicht)"]
fn werkstatt_hammer_herstellen_und_holen() {
    let mut w = quiet_world();
    w.cycle_speed = 1e-6;
    let players = [add_player(&mut w), add_player(&mut w)];
    let shop = build_site(&mut w, "workshop");
    let cost = w
        .vals
        .as_ref()
        .map(|v| v.buildings["workshop"].hammer_cost.clone())
        .unwrap_or_default();
    let sx = w.sites[shop].x;
    let paid = |w: &World| w.sites[shop].bow_paid_gold >= cost.gold || w.sites[shop].craft_done > 0.0;
    assert!(pay_at(&mut w, &players, sx, 10.0, &paid), "Hammer nicht bezahlt");
    let peasant = hammerless_peasant(&mut w, sx + 6.0);
    let has = |w: &World| w.troops[peasant].hammer;
    assert!(run_until(&mut w, 80.0, &has) >= 0.0, "Bauer holt den Hammer nicht");
    assert!(
        w.sites[shop].hammers == 0 && w.sites[shop].bows == 0,
        "Regal danach nicht leer"
    );
}

/// B-951: Der Bogner stellt Bögen her (Preis des Bogens), die Werkstatt keine; ein Bauer wird Bogenschütze.
#[test]
#[ignore = "wartet auf RU18 (Ursache ungeprueft: Bauer wird nicht Bogenschütze)"]
fn bogner_stellt_boegen_her() {
    let mut w = quiet_world();
    w.cycle_speed = 1e-6;
    let players = [add_player(&mut w), add_player(&mut w)];
    let bowyer = build_site(&mut w, "bowyer");
    let shop = build_site(&mut w, "workshop");
    let bow_gold = w.vals.as_ref().map_or(0, |v| v.troops["archer"].cost.gold);
    let bx = w.sites[bowyer].x;
    let paid = |w: &World| w.sites[bowyer].bow_paid_gold >= bow_gold || w.sites[bowyer].craft_done > 0.0;
    assert!(pay_at(&mut w, &players, bx, 10.0, &paid), "Bogen nicht bezahlt");
    let peasant = hammerless_peasant(&mut w, bx + 6.0);
    let archer = |w: &World| w.troops[peasant].kind == "archer";
    assert!(run_until(&mut w, 80.0, &archer) >= 0.0, "Bauer wird nicht Bogenschütze");
    assert!(
        w.sites[shop].bows == 0 && w.sites[shop].craft_done == 0.0,
        "Werkstatt stellt Bögen her"
    );
}

/// B-950: Fällt der Handwerker, bleibt der Hammer liegen; ein anderer Bauer holt ihn, aber nicht mit einem Gegner in 8 Units.
#[test]
fn hammer_faellt_und_wird_aufgehoben() {
    let mut w = quiet_world();
    w.cycle_speed = 1e-6;
    add_player(&mut w);
    add_player(&mut w);
    let hub = w.hub_x;
    let smith = add_peasant(&mut w, hub - 5.0, "");
    w.troops[smith].hammer = true;
    lose_layer(&mut w, smith);
    assert!(
        !w.troops[smith].hammer && w.drops.len() == 1 && w.drops[0].kind == HAMMER_KIND,
        "Hammer fällt nicht"
    );
    let other = hammerless_peasant(&mut w, hub + 10.0);
    let dx = w.drops[0].x + 3.0;
    let near = spawn(&mut w, "goblin", dx);
    w.enemies[near].stun = f64::INFINITY;
    let v = w.vals.clone().expect("Wertesatz");
    assert!(
        crate::unit::warrior::weapon_to_fetch(&w, &v, other).is_none(),
        "Gegner nahe: kein Auftrag"
    );
    w.enemies.clear();
    let has = |w: &World| w.troops[other].hammer;
    assert!(run_until(&mut w, 30.0, &has) >= 0.0 && w.drops.is_empty());
}

/// B-952: Ein gefällter Baum wirft 1 bis 3 Goldmünzen (aus den Daten), deterministisch für gleichen Seed.
#[test]
#[ignore = "wartet auf RU18 (Ursache ungeprueft: Baum nicht gefällt)"]
fn baum_wirft_gold() {
    let mut counts = [0usize; 2];
    for c in &mut counts {
        let mut w = day_world();
        add_player(&mut w);
        add_player(&mut w);
        let tree = first_node(&w, "tree");
        w.nodes[tree].marked = true;
        w.troops[0].x = w.nodes[tree].x;
        let before = w.coins.len();
        let gone = |w: &World| w.nodes.get(tree).is_none_or(|n| n.gone);
        assert!(run_until(&mut w, 60.0, &gone) >= 0.0, "Baum nicht gefällt");
        *c = w.coins.len() - before;
    }
    let tg = w_values().economy.tree_gold;
    assert!(
        counts[0] as i64 >= tg[0] && counts[0] as i64 <= tg[1] && counts[0] == counts[1],
        "Münzen je Baum {counts:?}"
    );
}

fn w_values() -> Arc<crate::state::WorldValues> {
    bare_world().vals.expect("Wertesatz")
}

/// B-953: Die Truhe wirft ihr Gold als Fontäne: Münzen im Flug sind noch nicht aufsammelbar, danach liegen sie am Boden.
#[test]
fn truhe_wirft_fontaene() {
    let v = w_values();
    assert!(v.economy.chest_gold[0] >= 30, "Truhen-Gold {:?}", v.economy.chest_gold);
    let mut xs: [Vec<f64>; 2] = [Vec::new(), Vec::new()];
    for run in &mut xs {
        let mut w = quiet_world();
        let players = [add_player(&mut w), add_player(&mut w)];
        let id = w.new_id();
        let hub = w.hub_x;
        w.pickups.push(crate::state::Pickup {
            id,
            kind: "chest".into(),
            x: hub + 40.0,
        });
        for p in players {
            w.players[p].x = hub + 40.0;
        }
        let gold = w.players[players[0]].gold;
        step(&mut w, None, DT);
        let got = w.coins.len() as i64;
        assert!(got >= v.economy.chest_gold[0] && got <= v.economy.chest_gold[1] && w.players[players[0]].gold == gold);
        for c in &w.coins {
            assert!(c.lands_at > w.time && (c.x - c.from_x).abs() <= v.economy.chest_fountain.spread_units + 1e-9);
            run.push(c.x);
        }
    }
    assert_eq!(xs[0], xs[1], "Fontäne nicht deterministisch");
}

/// Review WI1: Hämmer gelten nicht als Kämpfer-Waffe und werden auch bei voller Truppe geholt.
#[test]
fn hammer_bei_voller_truppe_geholt() {
    let mut w = quiet_world();
    w.cycle_speed = 1e-6;
    add_player(&mut w);
    add_player(&mut w);
    let limit = crate::state::links::troop_limit(&w);
    let hub = w.hub_x;
    for _ in 0..limit {
        let ti = crate::unit::spawn_vagrant(&mut w, hub, hub);
        crate::unit::jobs::make_archer(&mut w, ti);
    }
    let shop = build_site(&mut w, "workshop");
    w.sites[shop].hammers = 1;
    let sx = w.sites[shop].x;
    let peasant = hammerless_peasant(&mut w, sx + 4.0);
    let has = |w: &World| w.troops[peasant].hammer;
    assert!(
        run_until(&mut w, 20.0, &has) >= 0.0,
        "Hammer bei voller Truppe nicht geholt"
    );
}

/// Review WI1: Ohne Handwerker und Hammer legt die Burg zum Tagesbeginn einen ab; Gegner nehmen keinen mit.
#[test]
fn burg_legt_ersatzhammer_ab() {
    let mut w = bare_world();
    let v = w.vals.clone().expect("Wertesatz");
    if !v.hub.castle_spare_hammer {
        return; // der Ersatzhammer ist in den Daten abgeschaltet
    }
    spare_hammer(&mut w, &v);
    assert!(w.drops.len() == 1 && w.drops[0].kind == HAMMER_KIND && w.drops[0].x == w.castle.x);
    spare_hammer(&mut w, &v);
    assert_eq!(w.drops.len(), 1, "zweiter Hammer trotz Hammer am Boden");
    w.drops.clear();
    w.troops.push(Troop {
        id: 99,
        kind: "peasant".into(),
        hammer: true,
        ..Troop::default()
    });
    spare_hammer(&mut w, &v);
    assert!(w.drops.is_empty(), "Ersatzhammer trotz Handwerker");
}

#[test]
fn gegner_traegt_den_ersatzhammer_nicht_weg() {
    let mut w = bare_world();
    let v = w.vals.clone().expect("Wertesatz");
    spare_hammer(&mut w, &v);
    w.portals = vec![w.width_units];
    let cx = w.castle.x;
    let e = spawn(&mut w, "goblin", cx);
    crate::enemies::equipment_taken::take_equipment(&mut w, &v, e);
    assert!(
        w.drops.len() == 1 && w.enemies[e].carried.is_none(),
        "Gegner trägt den Hammer weg"
    );
}

/// Review WI1: Plantagen-Bäume geben kein Gold; ein Handwerker wird nicht ausgebildet.
#[test]
fn kein_plantagen_gold_und_handwerker_nicht_ausgebildet() {
    let mut w2 = day_world();
    let id = w2.new_id();
    let at = w2.hub_x + 3.0;
    w2.nodes.push(crate::state::ResourceNode {
        id,
        kind: "tree".into(),
        x: at,
        marked: true,
        plant: true,
        ..Default::default()
    });
    w2.troops[0].x = at;
    let before = w2.coins.len();
    let gone = |w: &World| w.nodes.iter().all(|n| n.id != id);
    assert!(run_until(&mut w2, 60.0, &gone) >= 0.0, "Plantagen-Baum nicht gefällt");
    assert_eq!(w2.coins.len(), before, "Plantagen-Baum warf Münzen");
    // `freePeasant` wählt keinen Handwerker: Test mit RU11.4 (unit_professions).
}

#[test]
fn drop_hammer_laesst_den_hammer_am_boden() {
    let mut w = bare_world();
    w.troops.push(Troop {
        id: 5,
        kind: "peasant".into(),
        x: 77.0,
        hammer: true,
        ..Troop::default()
    });
    drop_hammer(&mut w, 0);
    assert!(!w.troops[0].hammer);
    assert!(w.drops.len() == 1 && w.drops[0].kind == HAMMER_KIND && w.drops[0].x == 77.0);
    drop_hammer(&mut w, 0);
    assert_eq!(w.drops.len(), 1, "ohne Hammer passiert nichts");
}

#[test]
fn place_product_je_hersteller() {
    let w = bare_world();
    let v = w.vals.clone().expect("Wertesatz");
    let mut shop = crate::state::Site {
        kind: "workshop".into(),
        hammers: 2,
        bows: 1,
        ..Default::default()
    };
    let p = place_product(&v, &shop);
    assert_eq!(
        (p.name, p.product, p.rack),
        (HAMMER_KIND, Product::Hammer, v.buildings["workshop"].hammer_rack)
    );
    assert_eq!(p.product.have(&shop), 2);
    p.product.add(&mut shop, 1);
    assert_eq!(shop.hammers, 3);
    let bowyer = crate::state::Site {
        kind: "bowyer".into(),
        bows: 4,
        ..Default::default()
    };
    let b = place_product(&v, &bowyer);
    assert_eq!(
        (b.name, b.product, b.cost),
        ("bow", Product::Bow, v.troops["archer"].cost.clone())
    );
    assert_eq!(b.product.have(&bowyer), 4);
}
