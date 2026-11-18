//! Tests der kleinen Zustands-Typen: node_mark, purse, site_view, torches, `key` (Port der Go-Tests; Tests, die eine
//! Kampagne oder Insel brauchen, sind `ignore` mit Sprint, sie bleiben mit Namen und Beschreibung erhalten).

use std::collections::BTreeMap;

use k3c_det::golden;
use serde::Deserialize;

use super::node_mark::{apply_marked_by, mark_node, marked_by_of};
use super::save::key;
use super::torches::torch_xs;
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

/// Go: `TestFackelnDerWelt` (torches_test.go) braucht `quietWorld` und `destroySite` (ruft `is_ruin`, RU18).
#[test]
#[ignore = "wartet auf RU18"]
fn fackeln_der_welt() {}

/// Go: `TestKnotenMarkiererInSnapshotUndSpielstand` (node_mark_test.go) braucht Kampagne, `gather_of` (RU18) und Spielstand.
#[test]
#[ignore = "wartet auf RU18"]
fn knoten_markierer_in_snapshot_und_spielstand() {}

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

/// Go: `TestPurseSizes` (purse_test.go) braucht Insel, Skills und `giveGold`.
#[test]
#[ignore = "wartet auf RU18"]
fn purse_sizes() {}

/// Go: `TestPurseRespecDropsNewGold` (purse_test.go) braucht Insel, Skills und Respec.
#[test]
#[ignore = "wartet auf RU18"]
fn purse_respec_drops_new_gold() {}

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
