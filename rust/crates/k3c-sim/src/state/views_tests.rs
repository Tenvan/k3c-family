//! Tests der kleinen Zustands-Typen: node_mark, purse, site_view, torches, `key` (Port der Go-Tests; Tests, die eine
//! Kampagne oder Insel brauchen, sind `ignore` mit Sprint, sie bleiben mit Namen und Beschreibung erhalten).

use std::collections::BTreeMap;

use k3c_det::golden;
use serde::Deserialize;

use super::common::destroy_site;
use super::node_mark::{apply_marked_by, mark_node, marked_by_of};
use super::save::key;
use super::torches::{torch_xs, torch_xs_of};
use super::{Player, ResourceNode, Site, World};

fn node(kind: &str, x: f64) -> ResourceNode {
    ResourceNode {
        kind: kind.into(),
        x,
        ..Default::default()
    }
}

/// Go: `TestFackelPlaetze` (torches_test.go): derselbe Fall prüft den Client (`testdata/torches.json`).
#[test]
fn fackel_plaetze() {
    #[derive(Deserialize)]
    struct Case {
        #[serde(rename = "hubX")]
        hub_x: f64,
        offsets: BTreeMap<String, Vec<f64>>,
        sites: Vec<Site>,
        torches: Vec<f64>,
    }
    let raw = golden::read(golden::testdata().join("torches.json"));
    let c: Case = serde_json::from_value(raw).unwrap();
    assert_eq!(torch_xs(&c.sites, c.hub_x, &c.offsets), c.torches);
}

/// Die Versätze der Datei `data/light.json`: Mauer und Turm tragen Fackeln.
#[test]
fn fackel_versaetze_aus_light_json() {
    let d = k3c_data::load_all(&k3c_data::Source::Embedded).unwrap();
    assert!(
        !d.light.torches["wall"].is_empty() && !d.light.torches["tower"].is_empty(),
        "{:?}",
        d.light.torches
    );
}

/// Go: `TestFackelnDerWelt` (torches_test.go).
#[test]
fn fackeln_der_welt() {
    let mut w = crate::world::testutil::quiet_world();
    let offsets = k3c_data::load_all(&k3c_data::Source::Embedded).unwrap().light.torches;
    assert!(
        !offsets["wall"].is_empty() && !offsets["tower"].is_empty(),
        "data/light.json › torches ohne Mauer oder Turm: {offsets:?}"
    );
    assert!(torch_xs_of(&w, &offsets).is_empty(), "ohne gebaute Mauer schon Fackeln");
    // Go: `leftWall`: die linke Mauer am nächsten zur Hub-Mitte (Plätze liegen nach X sortiert), gebaut mit voller HP
    let hub = w.hub_x;
    let wi = w
        .sites
        .iter()
        .rposition(|s| s.kind == "wall" && s.x < hub)
        .expect("keine linke Mauer");
    (w.sites[wi].state, w.sites[wi].hp) = ("built".into(), w.sites[wi].max_hp);
    let got = torch_xs_of(&w, &offsets).len();
    assert_eq!(
        got,
        offsets["wall"].len(),
        "gebaute Mauer: {got} Fackeln, erwartet {}",
        offsets["wall"].len()
    );
    let id = w.sites[wi].id;
    destroy_site(&mut w, id);
    assert!(
        torch_xs_of(&w, &offsets).is_empty(),
        "zerstörte Mauer trägt noch Fackeln"
    );
}

/// Go: `TestKnotenMarkiererInSnapshotUndSpielstand` (node_mark_test.go): Wer einen Baum markiert, steht im Snapshot und im
/// Spielstand; nach dem Laden ist der Markierer derselbe.
#[test]
fn knoten_markierer_in_snapshot_und_spielstand() {
    use crate::campaign::Campaign;
    use crate::world::testutil::pay_at;
    let mut c = Campaign::create("markedby", "t", 1e-6).expect("Kampagne");
    c.join_player();
    let p1 = c.join_player().expect("Spieler 2");
    let w = c.current_world_mut().expect("Welt");
    // der äußerste Baum liegt abseits anderer Zahlziele
    let (tx, ti) = w.nodes.iter().enumerate().filter(|(_, n)| n.kind == "tree").fold(
        (f64::MIN, usize::MAX),
        |(bx, bi), (i, n)| if n.x > bx { (n.x, i) } else { (bx, bi) },
    );
    assert!(
        w.nodes[ti].marked_by.is_none(),
        "unmarkierter Knoten hat einen Markierer"
    );
    let index = w.players[p1].index;
    pay_at(w, &[p1], tx, 10.0, |w| {
        w.nodes.iter().any(|n| n.kind == "tree" && n.x == tx && n.marked)
    });
    let tree = w.nodes.iter().find(|n| n.kind == "tree" && n.x == tx).expect("Baum");
    assert!(
        tree.marked && tree.marked_by == Some(index),
        "markiert {}, markedBy {:?}, erwartet Spieler {index}",
        tree.marked,
        tree.marked_by
    );
    let s = c.to_save("2026-10-23T00:00:00Z").expect("Spielstand");
    let loaded = Campaign::from_save(&s, 1e-6).expect("laden");
    let got = loaded
        .current_world()
        .and_then(|w| w.nodes.iter().find(|n| n.kind == "tree" && n.x == tx));
    assert!(
        got.is_some_and(|n| n.marked && n.marked_by == Some(index)),
        "nach dem Laden: {got:?}"
    );
}

/// Go: `TestKnotenMarkiererFehltImAltenStand` (node_mark_test.go): Ein Stand ohne `nodesMarkedBy` lädt, der Knoten bleibt
/// markiert, der Markierer unbekannt. Hier ohne Kampagne: derselbe Weg über `marked_by_of` und `apply_marked_by`.
#[test]
fn knoten_markierer_fehlt_im_alten_stand() {
    let p = Player {
        index: 1,
        ..Default::default()
    };
    let mut tree = node("tree", 12.5);
    mark_node(&mut tree, &p);
    assert!(tree.marked && tree.marked_by == Some(1));
    let w = World {
        nodes: vec![tree],
        ..Default::default()
    };
    let saved = marked_by_of(&w);
    assert_eq!(saved.get("tree@12.50"), Some(&1));

    let mut loaded = node("tree", 12.5);
    loaded.marked = true;
    apply_marked_by(&mut loaded, &saved);
    assert_eq!(loaded.marked_by, Some(1));

    let mut old = node("tree", 12.5);
    old.marked = true;
    apply_marked_by(&mut old, &BTreeMap::new()); // älterer Stand: Markierer unbekannt
    assert!(old.marked && old.marked_by.is_none());

    let mut unmarked = node("tree", 12.5);
    apply_marked_by(&mut unmarked, &saved); // nicht markiert: bleibt ohne Markierer
    assert!(unmarked.marked_by.is_none());
}

#[test]
fn marked_by_of_ohne_markierer_ist_leer() {
    let mut n = node("rock", 3.0);
    n.marked = true; // markiert, Markierer unbekannt
    let w = World {
        nodes: vec![n, node("tree", 4.0)],
        ..Default::default()
    };
    assert!(marked_by_of(&w).is_empty());
}

/// Gleichstände (x = n/8) runden weg von 0 wie `toFixed(2)` in JS.
#[test]
fn key_rundet_wie_to_fixed() {
    let cases = [
        ("tree", 1.0, "tree@1.00"),
        ("tree", 0.125, "tree@0.13"),
        ("tree", 0.375, "tree@0.38"),
        ("tree", -0.125, "tree@-0.13"),
        ("rock", 12.5, "rock@12.50"),
        ("rock", 99.987_654, "rock@99.99"),
    ];
    for (kind, x, want) in cases {
        assert_eq!(key(kind, x), want, "key({kind}, {x})");
    }
}

/// Go: `TestPurseSizes` (purse_test.go, B-922/AC-01): Geldsack in drei Größen als Skill-Stufen: ohne Skill Größe 1 (Grundwert),
/// purseMedium Größe 2, purseLarge Größe 3 und erst nach purseMedium; jede Größe ist die Münz-Obergrenze des Spielers.
#[test]
fn purse_sizes() {
    use super::purse::{purse_max_of, purse_size};
    use crate::economy::give_gold;
    use crate::island::add_island_player;
    use crate::island::testutil::{add_island_pool, must_island, on_stage, test_data};
    use crate::monarch::learn_skill;
    let data = test_data();
    let mut isl = must_island(&data, "geldsack", &[0]);
    let (p, q) = (add_island_player(&mut isl, 0), add_island_player(&mut isl, 0));
    add_island_pool(&mut isl, 0, 5);
    let base = data.economy.purse.max_gold;
    on_stage(&mut isl, &data, 0, |w, r| {
        let v = crate::world::vals(w);
        let (size, max) = (purse_size(w, &w.players[p]), purse_max_of(w, &w.players[p], Some(r)));
        assert!(size == 1 && max == base, "ohne Skill: Größe {size}, Grenze {max}");
        assert!(
            learn_skill(w, &v, p, "purseLarge").is_err(),
            "purseLarge ohne purseMedium gelernt"
        );
        for (i, id) in ["purseMedium", "purseLarge"].into_iter().enumerate() {
            learn_skill(w, &v, p, id).expect("lernen");
            let want = v
                .skills
                .list
                .iter()
                .find(|s| s.id == id)
                .map_or(0, |s| s.effect.max_gold);
            let (size, max) = (purse_size(w, &w.players[p]), purse_max_of(w, &w.players[p], Some(r)));
            assert!(
                size == i as i64 + 2 && max == want && want > base,
                "{id}: Größe {size}, Grenze {max}, erwartet {want}"
            );
        }
        let (size, max) = (purse_size(w, &w.players[q]), purse_max_of(w, &w.players[q], Some(r)));
        assert!(
            size == 1 && max == base,
            "Mitspieler ohne Skill: Größe {size}, Grenze {max}"
        );
        (w.players[p].gold, w.players[q].gold) = (base, base);
        let (px, qx) = (w.players[p].x, w.players[q].x);
        give_gold(w, Some(r), p, 1000, px);
        give_gold(w, Some(r), q, 1000, qx);
        let want_p = purse_max_of(w, &w.players[p], Some(r));
        assert!(
            w.players[p].gold == want_p && w.players[q].gold == base,
            "Gold p {}, q {}",
            w.players[p].gold,
            w.players[q].gold
        );
    });
}

/// Go: `TestPurseRespecDropsNewGold` (purse_test.go, Review FB2): nach Respec liegt der Beutel über der Grenze; neues Gold
/// fällt zu Boden, der Beutel bleibt.
#[test]
fn purse_respec_drops_new_gold() {
    use super::purse::purse_max_of;
    use crate::economy::give_gold;
    use crate::island::add_island_player;
    use crate::island::testutil::{add_island_pool, must_island, on_stage, test_data};
    use crate::monarch::{learn_skill, respec};
    let data = test_data();
    let mut isl = must_island(&data, "geldsack", &[0]);
    let p = add_island_player(&mut isl, 0);
    add_island_pool(&mut isl, 0, 5);
    on_stage(&mut isl, &data, 0, |w, r| {
        let v = crate::world::vals(w);
        for id in ["purseMedium", "purseLarge"] {
            learn_skill(w, &v, p, id).expect("lernen");
        }
        w.players[p].gold = purse_max_of(w, &w.players[p], Some(r));
        let (full, coins) = (w.players[p].gold, w.coins.len());
        respec(w, &v, p).expect("Respec");
        let px = w.players[p].x;
        give_gold(w, Some(r), p, 5, px);
        assert!(
            w.players[p].gold == full && w.coins.len() == coins + 5,
            "nach Respec: Gold {} (vorher {full}), Münzen +{}",
            w.players[p].gold,
            w.coins.len() - coins
        );
    });
}

/// Go: `TestSiteHubLevelJeArt` (site_view_test.go): Platz-Stufe je Art; Linien-Plätze nach ihrer Zone.
#[test]
fn site_hub_level_je_art() {
    let w = crate::world::testutil::quiet_world();
    let mut lines = 0;
    for s in &w.sites {
        let mut want = crate::hub::site_hub_level(&w, s);
        if let Some(l) = crate::world::testutil::site_line(&w, s) {
            lines += 1;
            want = l.index;
        }
        let got = super::site_view::site_hub_level_of(&w, s);
        assert_eq!(got, want, "{}@{}: Platz-Stufe {got}, erwartet {want}", s.kind, s.x);
    }
    assert!(lines > 0, "keine Linien-Plätze in der Welt");
    let armory = w.sites.iter().find(|s| s.kind == "armory").expect("Rüstkammer");
    assert_eq!(
        super::site_view::site_hub_level_of(&w, armory),
        4,
        "Rüstkammer: Platz-Stufe laut data/hub.json 4"
    );
}

/// Go: `TestSiteHubLevelPasstZuBezahlbar` (site_view_test.go): Nie bezahlbar unter der Platz-Stufe: Bei Hub-Stufe h ist
/// jeder bezahlbare Platz höchstens Stufe h.
#[test]
fn site_hub_level_passt_zu_bezahlbar() {
    let mut w = crate::world::testutil::quiet_world();
    let v = crate::world::vals(&w);
    for h in 1..=5 {
        w.hub_level = h;
        for s in &w.sites {
            let level = super::site_view::site_hub_level_of(&w, s);
            assert!(
                !(s.state == "unpaid" && crate::economy::site_payable(&w, &v, None, s) && level > h),
                "Hub {h}: {}@{} bezahlbar, Platz-Stufe {level}",
                s.kind,
                s.x
            );
        }
    }
}

/// Go: `TestSiteVisibleBisNaechsteStufe` (site_view_test.go).
#[test]
fn site_visible_bis_naechste_stufe() {
    let mut w = crate::world::testutil::quiet_world();
    w.hub_level = 1;
    let si = w.sites.iter().position(|s| s.kind == "armory").expect("Rüstkammer"); // Platz-Stufe 4
    assert_eq!(w.sites[si].state, "unpaid", "Rüstkammer ist nicht unbezahlt");
    assert!(
        !super::site_view::site_visible(&w, &w.sites[si]),
        "Hub 1: Rüstkammer (Stufe 4) unbezahlt sichtbar"
    );
    w.hub_level = 3;
    assert!(
        super::site_view::site_visible(&w, &w.sites[si]),
        "Hub 3: Rüstkammer (Stufe 4) unsichtbar"
    );
    w.hub_level = 1;
    w.sites[si].paid_gold = 1;
    assert!(
        super::site_view::site_visible(&w, &w.sites[si]),
        "angezahlter Platz unsichtbar"
    );
    w.sites[si].paid_gold = 0;
    w.sites[si].state = "built".into();
    assert!(
        super::site_view::site_visible(&w, &w.sites[si]),
        "gebauter Platz unsichtbar"
    );
}
