//! Tests des Schiffs und des Hafens (Go: `ship_test.go`, `harbor_test.go`; BS5.1 bis BS5.3, B-637). Die Tests, die eine
//! Welt oder Insel bauen und rechnen, brauchen Plätze, Ruinen und Zahlen (RU18), `dev_build_site` und die
//! Geschichte der Standard-Welt (RU19) und tragen `ignore`; `own_*` prüft Teile ohne diese Voraussetzungen.

use k3c_data::buildings::{Cost, LevelData};
use k3c_data::hub::Stock;
use k3c_data::ship::{ShipFile, ShipFileStages};
use k3c_level::level::HARBOR_KIND;
use k3c_level::types::Entity;

use super::pending::{dev_build_site, level_of, upgrade_payable};
use super::ship::{SHIP_KIND, add_ship_site, harbor, ship_complete, with_ship};
use super::testutil::*;
use super::vals;
use crate::enemies::is_building;
use crate::island::create::{CreateOptions, create_island};
use crate::island::testutil::{count, load_island, save_json, standard_world, test_data};
use crate::island::{Island, switch_testutil::*};
use crate::state::{Site, World};

/// Go: `shipWorld`: eine ruhige Welt mit Hafen-Entity und Schiffs-Platz 60 Units links der Burg, ohne Bäume und Felsen.
fn ship_world() -> (World, usize) {
    let mut w = day_world();
    clear_nodes(&mut w);
    let x = w.hub_x - 60.0;
    w.level.entities.push(Entity {
        kind: HARBOR_KIND.into(),
        x,
        ..Entity::default()
    });
    add_ship_site(&mut w);
    let si = w.sites.len() - 1;
    let s = &w.sites[si];
    assert!(s.kind == SHIP_KIND && s.state == "unpaid", "Schiffs-Platz fehlt: {s:?}");
    (w, si)
}

/// Go: `sitePayable` (economy.go), RU18.
fn site_payable(_w: &World, _s: &Site) -> bool {
    panic!("RU18: site_payable")
}

/// Go: `materialOf` (economy.go): der Material-Anteil eines Preises als Vorrat.
fn material_of(c: &Cost) -> Stock {
    Stock {
        wood: c.wood,
        stone: c.stone,
        copper: c.copper,
        iron: c.iron,
        crystal: c.crystal,
    }
}

/// Go: `shipAt`: Stufe und Schiffs-Platz des Hafens der Insel.
fn ship_at(isl: &Island) -> (usize, usize) {
    harbor(isl).expect("kein Hafen auf der Insel")
}

#[test]
#[ignore = "wartet auf RU19"]
fn schiff_in_abschnitten_bauen() {
    // BS5.2/AC-02 (B-637/AC-03): Das Schiff wird in drei Abschnitten bezahlt und gebaut (Kosten aus data/ship.json,
    // unabhängig von der Hub-Stufe); erst der letzte Abschnitt macht es fertig.
    let (mut w, si) = ship_world();
    let pi = super::add_player(&mut w);
    let v = vals(&w);
    let stages = &v.buildings[SHIP_KIND].levels;
    assert!(
        stages.len() == 3 && w.hub_level == 1,
        "{} Abschnitte, Hub-Stufe {}",
        stages.len(),
        w.hub_level
    );
    for (i, st) in stages.iter().enumerate() {
        assert!(site_payable(&w, &w.sites[si]), "Abschnitt {} nicht bezahlbar", i + 1);
        assert!(
            !ship_complete(&w, &w.sites[si]),
            "Schiff vor Abschnitt {} fertig",
            i + 1
        );
        w.stock = material_of(&st.cost);
        w.players[pi].gold = st.cost.gold;
        let x = w.sites[si].x;
        w.troops[0].x = x;
        let want = i as i64 + 1;
        let done = move |w: &World| {
            let s = &w.sites[si];
            s.state == "built" && s.upgrade.is_empty() && level_of(w, s) == want
        };
        let pay_seconds = 2.0 * st.cost.gold as f64 * v.economy.pay_interval_seconds + 5.0;
        assert!(
            pay_at(&mut w, &[pi], x, pay_seconds, |w| w.players[pi].gold == 0),
            "Abschnitt {want} nicht bezahlt"
        );
        assert!(
            run_until(&mut w, 3.0 * st.build_seconds + 5.0, done) >= 0.0,
            "Abschnitt {want} nicht gebaut"
        );
        assert!(
            w.players[pi].gold == 0 && w.stock == Stock::default(),
            "Abschnitt {want}: Gold oder Vorrat übrig"
        );
    }
    let s = &w.sites[si];
    assert!(
        ship_complete(&w, s) && !upgrade_payable(&w, s) && !site_payable(&w, s),
        "Schiff nicht fertig: {s:?}"
    );
}

#[test]
#[ignore = "wartet auf RU19 (decode_world_story)"]
fn inselwechsel_braucht_fertiges_schiff() {
    // BS5.2/AC-02: Ohne fertiges Schiff öffnet der besiegte Endboss den Inselwechsel nicht; mit letztem Abschnitt öffnet er.
    let data = test_data();
    let (world, _) = test_islands(&data);
    let mut isl = gate_island(&data, &world, 2);
    let (wi, si) = ship_at(&isl);
    isl.stages[wi].sites[si].level = 2; // das Segel fehlt
    assert!(
        !ship_complete(&isl.stages[wi], &isl.stages[wi].sites[si]),
        "zwei von drei Abschnitten gelten als fertig"
    );
    let seconds = data.hub.travel.seconds + 1.0;
    let ev = step_seconds(&mut isl, &data, seconds);
    assert!(
        count(&ev, "islandGateOpen") == 0 && !isl.gate_open && !isl.switch_ready,
        "Wechsel ohne fertiges Schiff"
    );
    isl.stages[wi].sites[si].level = 3;
    let ev = step_seconds(&mut isl, &data, seconds);
    let n = count(&ev, "islandGateOpen");
    assert!(
        n == 1 && isl.switch_ready,
        "mit fertigem Schiff: {n}× islandGateOpen, bereit {}",
        isl.switch_ready
    );
}

#[test]
#[ignore = "wartet auf RU19 (decode_world_story)"]
fn inselwechsel_am_steg_nicht_an_der_burg() {
    // Der Wechselpunkt liegt am Steg, nicht an der Burg: wer an der Burg des Hafens (Stufe 0) steht, lässt den Wechsel warten.
    let data = test_data();
    let (world, _) = test_islands(&data);
    let mut isl = gate_island(&data, &world, 2);
    let (wi, _) = ship_at(&isl);
    let hub_x = isl.stages[wi].hub_x;
    isl.stages[0].players[1].x = hub_x;
    step_seconds(&mut isl, &data, data.hub.travel.seconds + 1.0);
    assert!(
        !isl.switch_ready,
        "Wechsel, obwohl ein Spieler an der Burg statt am Steg steht"
    );
}

#[test]
#[ignore = "wartet auf RU18"]
fn schiff_kein_gegner_ziel() {
    // Gegner zerstören das Schiff nicht (B-637, Default): es ist kein Gebäude-Ziel.
    let (mut w, si) = ship_world();
    w.sites[si].state = "built".into();
    assert!(
        !is_building(&w.sites[si]),
        "das Schiff gilt als Gebäude-Ziel der Gegner"
    );
}

#[test]
#[ignore = "wartet auf RU19 (decode_world_story)"]
fn schiff_spielstand_rundlauf() {
    // BS5.3/AC-03 (B-637/AC-04): Der Schiff-Fortschritt überlebt Speichern und Laden: gebaute Abschnitte, ein laufender
    // Ausbau (der Endboss-Sieg steht in den besiegten Bossen, `save_boss_tests.rs`).
    let data = test_data();
    let mut isl = create_island(
        &data,
        &standard_world(&data),
        "schiff-stand",
        &[0, 1],
        1.0,
        CreateOptions::default(),
    )
    .expect("Insel");
    let (wi, si) = ship_at(&isl);
    dev_build_site(&mut isl.stages[wi], si, 2); // Kiel und Rumpf gebaut
    let s = &mut isl.stages[wi].sites[si];
    s.upgrade = "waitingMaterial".into();
    s.upgrade_paid = 30;
    let saved_at = "2026-10-28T12:00:00Z";
    let first = save_json(&isl, saved_at);
    let loaded = load_island(&data, &first);
    assert_eq!(
        first,
        save_json(&loaded, saved_at),
        "Speichern, Laden, Speichern ergibt etwas anderes"
    );
    let (lwi, lsi) = ship_at(&loaded);
    let (lw, ls) = (&loaded.stages[lwi], &loaded.stages[lwi].sites[lsi]);
    let ok = ls.state == "built" && ls.level == 2 && ls.upgrade == "waitingMaterial" && ls.upgrade_paid == 30;
    assert!(ok && !ship_complete(lw, ls), "Schiff nach dem Laden: {ls:?}");
}

#[test]
#[ignore = "wartet auf RU19 (decode_world_story)"]
fn hafen_chunk_auf_insel_eins() {
    // BS5.1/AC-01 (B-637/AC-02): Insel 1, Tiefe 0 hat genau einen Hafen am geplanten Platz (−7), andere Tiefen keinen.
    let data = test_data();
    let isl = create_island(
        &data,
        &standard_world(&data),
        "hafen",
        &[0, 1],
        1.0,
        CreateOptions::default(),
    )
    .expect("Insel");
    for (depth, w) in isl.stages.iter().enumerate() {
        let harbors: Vec<&Entity> = w.level.entities.iter().filter(|e| e.kind == HARBOR_KIND).collect();
        if depth == 1 {
            assert!(harbors.is_empty(), "Tiefe 1: {} Häfen", harbors.len());
            continue;
        }
        assert_eq!(harbors.len(), 1, "Tiefe 0: {} Häfen, erwartet 1", harbors.len());
        let cw = w.level.width_units / w.level.chunks.len() as f64;
        let start = w.level.hub_center_units - 25.0 - 7.0 * cw; // Anfang von −7 (geplante-chunks.md § 1)
        let x = harbors[0].x;
        assert!(
            x >= start && x <= start + cw,
            "Hafen bei {x}, Platz −7 liegt in {start}…{}",
            start + cw
        );
    }
}

fn ship_file(n: usize) -> ShipFile {
    let stages = (1..=n as i64)
        .map(|i| ShipFileStages {
            cost: Cost {
                gold: 10 * i,
                wood: i,
                ..Cost::default()
            },
            hp: 100.0 * i as f64,
            build_seconds: 5.0 * i as f64,
        })
        .collect();
    ShipFile { stages }
}

#[test]
fn own_with_ship_traegt_abschnitte_als_stufen() {
    let mut b = std::collections::BTreeMap::new();
    with_ship(&mut b, &ship_file(3)).expect("ship.json");
    let ship = &b[SHIP_KIND];
    assert_eq!(ship.levels.len(), 3);
    assert!(
        ship.hp == 100.0 && ship.build_seconds == 5.0 && ship.cost.gold == 10,
        "erster Abschnitt am Platz: {ship:?}"
    );
    let last: &LevelData = &ship.levels[2];
    assert!(
        last.hp == 300.0 && last.cost.gold == 30 && last.build_seconds == 15.0,
        "Stufe 3: {last:?}"
    );
}

#[test]
fn own_with_ship_ohne_abschnitte_ist_ein_fehler() {
    let mut b = std::collections::BTreeMap::new();
    assert_eq!(
        with_ship(&mut b, &ship_file(0)),
        Err("ship.json: keine Abschnitte".to_string())
    );
    assert!(b.is_empty(), "nichts eingetragen");
}

#[test]
fn own_with_ship_aus_den_eingebetteten_daten() {
    let data = test_data();
    let mut b = data.buildings.clone();
    with_ship(&mut b, &data.ship).expect("ship.json");
    assert_eq!(b[SHIP_KIND].levels.len(), 3, "drei Abschnitte: Kiel, Rumpf, Segel");
}

#[test]
fn own_harbor_findet_die_stufe_mit_dem_schiff() {
    let ship_site = Site {
        kind: SHIP_KIND.into(),
        ..Site::default()
    };
    let wall = Site {
        kind: "wall".into(),
        ..Site::default()
    };
    let stage = |sites: Vec<Site>| World {
        sites,
        ..World::default()
    };
    let isl = Island {
        stages: vec![
            stage(vec![wall.clone()]),
            stage(vec![wall.clone(), ship_site.clone(), wall.clone()]),
        ],
        ..Island::default()
    };
    assert_eq!(harbor(&isl), Some((1, 1)));
    let isl = Island {
        stages: vec![stage(vec![wall])],
        ..Island::default()
    };
    assert_eq!(harbor(&isl), None, "ohne Hafen");
}
