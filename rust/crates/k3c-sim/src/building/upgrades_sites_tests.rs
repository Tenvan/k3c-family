//! Tests zu `building_levels_data_test.go`, `site_level_test.go` und `sites_test.go` (Go, gehören zu `upgrades.rs`: Ausbau,
//! Stufen und Kosten der Plätze): B-569 AC-01, W1.2 (B-112/AC-02, Sprint W1 AC-02), W3 (B-116/AC-02, AC-03). Werte aus
//! data/buildings.json › levels. Die Tests über `day_world`, `step` und die Ausbau-Funktionen warten auf RU18 (`new_hub_site`,
//! `site_cost`, `site_payable`, `next_level`, `level_of`, `upgrade_payable`).

use k3c_data::buildings::Cost;
use k3c_data::hub::Stock;

use super::testutil::*;
use crate::economy::site_payable;
use crate::hub::level_of;
use crate::pending::next_level;
use crate::state::World;
use crate::state::common::destroy_site;
use crate::unit::spawn_vagrant;
use crate::world::pending::{site_cost, upgrade_payable};
use crate::world::{hub_of, vals};

/// Go: `materialOf`: der Vorrat, der genau den Preis (ohne Gold) deckt.
fn material_of(c: &Cost) -> Stock {
    Stock {
        wood: c.wood,
        stone: c.stone,
        copper: c.copper,
        iron: c.iron,
        crystal: c.crystal,
    }
}

/// B-569 AC-01: Hub-Gebäude ohne Produkt-Zahlziel haben eigene `levels` (Stufe n ab Hub-Stufe n); Stufe 1 trägt die
/// Grundwerte, die HP wachsen, und die Zahl der Stufen entspricht der Hub-Stufenzahl.
#[test]
fn gebaeude_levels_in_den_daten() {
    let w = bare_world(); // Go: `fixtureIsland`; die hub.json ist dieselbe (Standard-Welt)
    let v = vals(&w);
    for kind in ["storage", "nursery", "barracks", "tavern", "healer"] {
        let b = &v.buildings[kind];
        assert_eq!(
            b.levels.len(),
            hub_of(&w).levels.len(),
            "{kind}: levels, erwartet Hub-Stufen"
        );
        let first = &b.levels[0];
        assert!(
            first.hp == b.hp && first.build_seconds == b.build_seconds && first.cost == b.cost,
            "{kind}: Stufe 1 {first:?} weicht von den Grundwerten ab"
        );
        for n in 1..b.levels.len() {
            assert!(
                b.levels[n].hp > b.levels[n - 1].hp,
                "{kind}: HP der Stufe {} nicht größer als Stufe {n}",
                n + 1
            );
        }
    }
}

/// Stufe 1 der levels sind die bisherigen Werte (der Client liest hp, cost, buildSeconds).
#[test]
fn stufe1_gleich_grundwerte() {
    let w = bare_world();
    let v = vals(&w);
    for kind in ["wall", "tower"] {
        let b = &v.buildings[kind];
        assert!(
            b.levels.len() == 5
                && b.levels[0].hp == b.hp
                && b.levels[0].cost == b.cost
                && b.levels[0].build_seconds == b.build_seconds,
            "{kind}: {} Stufen, Stufe 1 {:?}",
            b.levels.len(),
            b.levels[0]
        );
    }
    assert_eq!(v.hub.levels.len(), 5, "Hub-Stufen");
}

/// Go: `upgradeAt` bezahlt mit den Spielern den Ausbau von `s` auf die nächste Stufe (Gold je Spieler gleich verteilt,
/// Material genau im Vorrat) und lässt einen Bauern ohne Weg bauen. Rückgabe: Bauzeit in Sekunden, -1 bei Zeitablauf.
fn upgrade_at(w: &mut World, players: &[usize], s: usize) -> f64 {
    let lv = next_level(w, None, &w.sites[s]).unwrap_or_else(|| panic!("{} nicht ausbaubar", w.sites[s].kind));
    w.stock = material_of(&lv.cost);
    let n = players.len() as i64;
    let share = lv.cost.gold / n;
    for (i, &p) in players.iter().enumerate() {
        w.players[p].gold = share + if i == 0 { lv.cost.gold - share * n } else { 0 };
    }
    let interval = vals(w).economy.pay_interval_seconds;
    let sx = w.sites[s].x;
    let waiting = |w: &World| w.sites[s].upgrade == "waitingWorker";
    assert!(
        pay_at(w, players, sx, 1.2 * lv.cost.gold as f64 * interval + 5.0, waiting),
        "Ausbau nicht bezahlt"
    );
    for &p in players {
        assert_eq!(w.players[p].gold, 0, "Spieler {p} hat Gold übrig");
    }
    assert_eq!(
        w.stock,
        Stock::default(),
        "Vorrat, erwartet genau {:?} abgebucht",
        lv.cost
    );
    w.troops[0].x = sx;
    let target = level_of(w, &w.sites[s]) + 1;
    let done = |w: &World| {
        assert_eq!(
            w.sites[s].state, "built",
            "alte Stufe während des Ausbaus nicht mehr gebaut"
        );
        level_of(w, &w.sites[s]) == target
    };
    run_until(w, 2.0 * lv.build_seconds, &done)
}

/// Go: `wantLevel` prüft Stufe, HP, Position und Bauzeit nach einem Ausbau laut Daten.
fn want_level(w: &World, s: usize, level: i64, x: f64, took: f64) {
    let site = &w.sites[s];
    let lv = &vals(w).buildings[&site.kind].levels[(level - 1) as usize];
    assert!(
        site.level == level && site.hp == lv.hp && site.max_hp == lv.hp && site.x == x,
        "{}: Stufe {}, HP {}/{}, X {}; erwartet Stufe {level}, HP {}, X {x}",
        site.kind,
        site.level,
        site.hp,
        site.max_hp,
        site.x,
        lv.hp
    );
    assert!(
        took >= lv.build_seconds - DT && took <= lv.build_seconds + 0.5,
        "{} Stufe {level}: Bau dauerte {took:.2} s, laut Daten {} s",
        site.kind,
        lv.build_seconds
    );
}

/// Mauer der Linie 1 von Stufe 1 bis 5 am selben Platz; Linie 2 muss dafür nicht gebaut sein.
#[test]
#[ignore = "wartet auf RU18.4 (preset_for)"]
fn mauer_ausbau_am_selben_platz() {
    let mut w = day_world();
    let p = add_player(&mut w);
    let wall = line_site(&mut w, "wall", -1, 1);
    built(&mut w.sites[wall]);
    let x = w.sites[wall].x;
    for level in 2..=vals(&w).buildings["wall"].levels.len() as i64 {
        w.hub_level = level;
        let took = upgrade_at(&mut w, &[p], wall);
        want_level(&w, wall, level, x, took);
    }
    let outer = line_site(&mut w, "wall", -1, 2);
    assert_eq!(w.sites[outer].state, "unpaid", "Linie 2");
    assert!(!upgrade_payable(&w, &w.sites[wall]), "Mauer Stufe 5 weiter ausbaubar");
}

#[test]
fn mauer_stufe3_braucht_hub_stufe3() {
    let mut w = day_world();
    let wall = line_site(&mut w, "wall", 1, 1);
    built(&mut w.sites[wall]);
    w.sites[wall].level = 2;
    w.hub_level = 2;
    let v = vals(&w);
    assert!(
        !site_payable(&w, &v, None, &w.sites[wall]),
        "Mauer Stufe 3 bei Hub-Stufe 2 bezahlbar"
    );
    w.hub_level = 3;
    assert!(
        site_payable(&w, &v, None, &w.sites[wall]),
        "Mauer Stufe 3 bei Hub-Stufe 3 nicht bezahlbar"
    );
}

/// Turm-Ausbau am selben Platz (Q45); der Bogenschütze bleibt oben, bis der Zaubertum ihn absteigen lässt (Q31,
/// building_spell_tower_test.go).
#[test]
#[ignore = "wartet auf RU18.4 (preset_for)"]
fn turm_ausbau_schuetze_bleibt() {
    let mut w = day_world();
    let p = add_player(&mut w);
    let tower = line_site(&mut w, "tower", 1, 1);
    built(&mut w.sites[tower]);
    let tx = w.sites[tower].x;
    let archer = spawn_vagrant(&mut w, tx, tx);
    crate::unit::jobs::make_archer(&mut w, archer);
    let tid = w.sites[tower].id;
    w.troops[archer].anchor_x = tx;
    w.troops[archer].tower_id = Some(tid);
    let v = vals(&w);
    for level in 2..=v.buildings["tower"].levels.len() as i64 {
        w.hub_level = level;
        let took = upgrade_at(&mut w, &[p], tower);
        want_level(&w, tower, level, tx, took);
        let spell = level >= v.buildings["tower"].spell.from_level;
        assert_eq!(
            spell,
            w.troops[archer].tower_id.is_none(),
            "Stufe {level}: Schütze auf dem Turm"
        );
    }
}

#[test]
#[ignore = "wartet auf RU18.4 (preset_for)"]
fn mauer_ausbau_zwei_spieler() {
    let mut w = day_world();
    let (p0, p1) = (add_player(&mut w), add_player(&mut w));
    let wall = line_site(&mut w, "wall", -1, 1);
    built(&mut w.sites[wall]);
    w.hub_level = 2;
    let x = w.sites[wall].x;
    let took = upgrade_at(&mut w, &[p0, p1], wall);
    want_level(&w, wall, 2, x, took);
}

/// Go: `buildNew` bezahlt einen unbezahlten Platz `s` mit den Spielern (Gold gleich verteilt, Material genau im Vorrat) und
/// lässt einen Bauern bauen. Rückgabe: Bauzeit in Sekunden, -1 bei Zeitablauf.
fn build_new(w: &mut World, players: &[usize], s: usize) -> f64 {
    let v = vals(w);
    let cost = site_cost(w, None, &w.sites[s]);
    w.stock = material_of(&cost);
    let n = players.len() as i64;
    let share = cost.gold / n;
    for (i, &p) in players.iter().enumerate() {
        w.players[p].gold = share + if i == 0 { cost.gold - share * n } else { 0 };
    }
    let sx = w.sites[s].x;
    let waiting = |w: &World| w.sites[s].state == "waitingWorker";
    let seconds = 1.2 * cost.gold as f64 * v.economy.pay_interval_seconds + 5.0;
    assert!(
        pay_at(w, players, sx, seconds, waiting),
        "{}: nicht bezahlt",
        w.sites[s].kind
    );
    for &p in players {
        assert_eq!(w.players[p].gold, 0, "{}: Spieler {p} hat Gold übrig", w.sites[s].kind);
    }
    assert_eq!(
        w.stock,
        Stock::default(),
        "{}: Vorrat, erwartet genau {cost:?} abgebucht",
        w.sites[s].kind
    );
    for t in w.troops.iter_mut().filter(|t| t.kind == "peasant") {
        t.x = sx;
    }
    let bs = v.buildings[&w.sites[s].kind].build_seconds;
    let built_now = |w: &World| w.sites[s].state == "built";
    run_until(w, 2.0 * bs + 1.0, &built_now)
}

/// Go: `wantBuiltFromData` prüft HP und Bauzeit eines frisch gebauten Platzes gegen data/buildings.json.
fn want_built_from_data(w: &World, s: usize, took: f64) {
    let site = &w.sites[s];
    let b = &vals(w).buildings[&site.kind];
    assert!(
        site.hp == b.hp && site.max_hp == b.hp,
        "{}: HP {}/{}, laut Daten {}",
        site.kind,
        site.hp,
        site.max_hp,
        b.hp
    );
    assert!(
        took >= b.build_seconds - DT && took <= b.build_seconds + 0.5,
        "{}: Bau dauerte {took:.2} s, laut Daten {} s",
        site.kind,
        b.build_seconds
    );
}

/// Go: `wantHubBuildFromData`: Platz und Hub-Stufe aus hub.json, darunter nicht bezahlbar, Kosten, HP und Bauzeit aus
/// buildings.json; mit zwei Spielern bezahlt.
fn want_hub_build_from_data(kind: &str) -> World {
    let mut w = day_world();
    let players = [add_player(&mut w), add_player(&mut w)];
    let s = hub_site_of(&w, kind);
    let h = hub_of(&w)
        .sites
        .iter()
        .find(|h| h.kind == kind)
        .map(|h| h.hub_level)
        .unwrap_or(0);
    let v = vals(&w);
    let level = crate::state::site_view::site_hub_level_of(&w, &w.sites[s]);
    assert_eq!(level, h, "{kind}: Hub-Stufe {level}, laut Daten {h}");
    w.hub_level = h - 1;
    assert!(
        !site_payable(&w, &v, None, &w.sites[s]),
        "{kind}: bei Hub-Stufe {} bezahlbar",
        w.hub_level
    );
    w.hub_level = h;
    let took = build_new(&mut w, &players, s);
    want_built_from_data(&w, s, took);
    w
}

#[test]
#[ignore = "wartet auf RU18.4 (preset_for)"]
fn hub_gebaeude_aus_den_daten() {
    for kind in ["barracks", "tavern", "healer", "smithy", "armory"] {
        want_hub_build_from_data(kind);
    }
}

/// W3.2 (B-116/AC-03, Sprint W3 AC-03): Schmiede und Rüstkammer lassen sich bauen (oben, ab ihrer Hub-Stufe) und zerstören;
/// ihre Wirkung prüft `upgrades_tests.rs` (W4.3b, B-313).
#[test]
#[ignore = "wartet auf RU18.4 (preset_for)"]
fn schmiede_und_ruestkammer_zerstoerbar() {
    for kind in ["smithy", "armory"] {
        let mut w = want_hub_build_from_data(kind);
        let s = hub_site_of(&w, kind);
        let id = w.sites[s].id;
        destroy_site(&mut w, id);
        let last = w
            .events
            .last()
            .and_then(|e| e.get("type"))
            .and_then(|t| t.as_str())
            .map(str::to_string);
        assert!(
            w.sites[s].state == "unpaid" && w.sites[s].hp == 0.0 && last.as_deref() == Some("destroyed"),
            "{kind}: nach der Zerstörung {:?}",
            w.sites[s]
        );
    }
}
