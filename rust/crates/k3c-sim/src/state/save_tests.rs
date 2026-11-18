//! Tests zu `save.rs`, `save_apply.rs` und `save_economy.rs`. Die Go-Tests von `save_economy_test.go` brauchen Insel und
//! Spielstand-Fixtures (`ToSave`, `FromIslandSave`); sie stehen als `ignore` mit Sprint. Dazu die Fälle, die ohne spätere
//! Sprints laufen (Go prüft `save.go` über die Kampagnen-Tests, `campaign_test.go`). Der Rundlauf über alle Golden steht in
//! `tests/golden_state.rs`.

use k3c_data::buildings::{BuildingData, Offer};
use k3c_data::{Source, load_all};

use super::fehler::Fehler;
use super::save::*;
use super::save_apply::{apply_upgrade, set};
use super::save_economy::*;
use super::{Plot, ResourceNode, Site, Troop, World, WorldValues};

fn site(id: i64, kind: &str, x: f64, state: &str) -> Site {
    Site {
        id,
        kind: kind.into(),
        x,
        state: state.into(),
        ..Default::default()
    }
}

#[test]
fn keys_und_without_behalten_die_reihenfolge() {
    let nodes = vec![
        ResourceNode {
            kind: "tree".into(),
            x: 3.0,
            marked: true,
            ..Default::default()
        },
        ResourceNode {
            kind: "rock".into(),
            x: 1.5,
            ..Default::default()
        },
        ResourceNode {
            kind: "tree".into(),
            x: 9.0,
            ..Default::default()
        },
    ];
    let all = keys(&nodes, node_key);
    assert_eq!(all, ["tree@3.00", "rock@1.50", "tree@9.00"]);
    let marked = keys(&nodes, |n| n.marked.then(|| key(&n.kind, n.x)));
    assert_eq!(marked, ["tree@3.00"]);
    assert_eq!(without(&all, &["rock@1.50".to_string()]), ["tree@3.00", "tree@9.00"]);
    assert_eq!(set(&all).len(), 3);
}

#[test]
fn parse_save_prueft_pflichtfelder_version_und_tiefe() {
    let d = load_all(&Source::Embedded).unwrap();
    let v = WorldValues::default();
    let parse = |raw: &str| parse_save(raw, &d.biomes, &v).unwrap_err().to_string();
    assert!(parse("{").starts_with("spielstand: "));
    let all = r#""version":1,"campaignId":"a","seed":"s","depth":0,"time":0,"hubs":[],"players":[]"#;
    for f in ["version", "campaignId", "seed", "depth", "time", "hubs", "players"] {
        let json = format!("{{{all}}}").replace(&format!("\"{f}\""), "\"x\"");
        assert_eq!(parse(&json), format!("spielstand: Feld {f} fehlt"), "{f}");
    }
    assert_eq!(
        parse(&format!("{{{all}}}").replace("\"version\":1", "\"version\":2")),
        "spielstand: Version 2, erwartet 1"
    );
    assert_eq!(
        parse(&format!("{{{all}}}").replace("\"depth\":0", "\"depth\":99")),
        "spielstand: unbekannte Tiefe"
    );
    let ok = parse_save(&format!("{{{all}}}"), &d.biomes, &v).unwrap();
    assert_eq!((ok.version, ok.campaign_id.as_str(), ok.depth), (1, "a", 0));
}

#[test]
fn site_save_schreibt_und_craft_save_laesst_leeres_weg() {
    let mut s = site(1, "workshop", 5.0, "built");
    s.paid_gold = 4;
    let w = World::default();
    let saved = site_save(&w, &s);
    assert_eq!(
        (saved.kind.as_str(), saved.paid_gold, saved.craft.is_none()),
        ("workshop", 4, true)
    );
    s.swords = 2;
    s.sword_paid_gold = 1;
    s.sword_done = 20.5;
    s.hammers = 1;
    let c = craft_save(&w, &s).unwrap();
    assert_eq!((c.swords, c.sword_paid_gold, c.sword_done, c.hammers), (2, 1, 20.5, 1));
    s.offer_paid = vec![0, 0];
    s.swords = 0;
    s.sword_paid_gold = 0;
    s.sword_done = 0.0;
    s.hammers = 0;
    assert!(craft_save(&w, &s).is_none(), "nur Nullen: nichts zu speichern");
    s.offer_paid = vec![0, 3];
    assert_eq!(craft_save(&w, &s).unwrap().offer_paid, [0, 3]);
}

#[test]
fn craft_save_plantage_zaehlt_nur_stehende_baeume() {
    let mut nursery = site(1, "nursery", 0.0, "built");
    nursery.plots = vec![
        Plot {
            tree: Some(10),
            grow_in: 0.0,
        },
        Plot {
            tree: Some(11),
            grow_in: 0.0,
        },
        Plot {
            tree: None,
            grow_in: 12.5,
        },
    ];
    let w = World {
        nodes: vec![
            ResourceNode {
                id: 10,
                ..Default::default()
            },
            ResourceNode {
                id: 11,
                gone: true,
                ..Default::default()
            },
        ],
        ..Default::default()
    };
    let c = craft_save(&w, &nursery).unwrap();
    let trees: Vec<bool> = c.plots.iter().map(|p| p.tree).collect();
    assert_eq!(trees, [true, false, false]);
    assert_eq!(c.plots[2].grow_in, 12.5);
    nursery.state = "unpaid".into();
    assert!(
        craft_save(&w, &nursery).is_none(),
        "zerstörte Baumschule: keine Plantage"
    );
}

#[test]
fn apply_craft_prueft_die_zahl_der_angebote() {
    let mut v = WorldValues::default();
    v.buildings.insert(
        "smithy".into(),
        BuildingData {
            offers: vec![Offer::default()],
            ..Default::default()
        },
    );
    let c = CraftSave {
        done: 40.5,
        offer_paid: vec![3],
        offer_done: 50.5,
        swords: 2,
        ..Default::default()
    };
    let mut s = site(1, "smithy", 0.0, "built");
    apply_craft(&v, &mut s, Some(&c));
    assert_eq!(
        (s.craft_done, s.offer_paid.clone(), s.offer_done, s.swords),
        (40.5, vec![3], 50.5, 2)
    );
    let mut s2 = site(2, "smithy", 0.0, "built");
    apply_craft(
        &v,
        &mut s2,
        Some(&CraftSave {
            offer_paid: vec![1, 2],
            ..Default::default()
        }),
    );
    assert!(s2.offer_paid.is_empty(), "mehr Angebote als Daten: bleibt leer");
    apply_craft(&v, &mut s2, None);
}

#[test]
fn apply_upgrade_uebernimmt_nur_laufende_ausbauten() {
    let mut s = site(1, "tower", 0.0, "built");
    let saved = SiteSave {
        upgrade: "waitingWorker".into(),
        build_progress: 0.4,
        upgrade_paid: 6,
        ..Default::default()
    };
    apply_upgrade(&mut s, &saved);
    assert_eq!(
        (s.upgrade.as_str(), s.build_progress, s.upgrade_paid),
        ("waitingWorker", 0.4, 6)
    );
    let mut t = site(2, "tower", 0.0, "built");
    apply_upgrade(
        &mut t,
        &SiteSave {
            upgrade: "bogus".into(),
            build_progress: 0.9,
            upgrade_paid: 2,
            ..Default::default()
        },
    );
    assert_eq!((t.upgrade.as_str(), t.build_progress, t.upgrade_paid), ("", 0.0, 2));
}

#[test]
fn troops_save_ohne_landstreicher_mit_arbeitsplatz() {
    let w = World {
        sites: vec![site(7, "workshop", 12.5, "built")],
        troops: vec![
            Troop {
                kind: "vagrant".into(),
                ..Default::default()
            },
            Troop {
                kind: "peasant".into(),
                x: 1.0,
                anchor_x: 2.0,
                profession: "craftsman".into(),
                work_site: 7,
                hammer: true,
                ..Default::default()
            },
            Troop {
                kind: "warrior".into(),
                leader: true,
                work_site: 99,
                ..Default::default()
            },
        ],
        ..Default::default()
    };
    let t = troops_save(&w);
    assert_eq!(t.len(), 2);
    assert_eq!(
        (t[0].kind.as_str(), t[0].work_site.as_str(), t[0].hammer),
        ("peasant", "workshop@12.50", true)
    );
    assert_eq!(
        (t[1].kind.as_str(), t[1].work_site.as_str(), t[1].leader),
        ("warrior", "", true)
    );
}

#[test]
fn site_by_key_findet_den_platz() {
    let w = World {
        sites: vec![site(1, "wall", 3.0, "built"), site(2, "tower", 4.5, "built")],
        ..Default::default()
    };
    assert_eq!(site_by_key(&w, "tower@4.50"), Some(1));
    assert_eq!(site_by_key(&w, "tower@4.51"), None);
}

#[test]
fn validate_economy_ruestungsstufe() {
    let mut v = WorldValues::default();
    v.buildings.insert(
        "armory".into(),
        BuildingData {
            armor: vec![Default::default(); 2],
            ..Default::default()
        },
    );
    let ok = HubSave {
        armor_level: 2,
        ..Default::default()
    };
    assert_eq!(validate_economy(&v, &ok), Ok(()));
    for level in [-1, 3, 9] {
        let h = HubSave {
            armor_level: level,
            ..Default::default()
        };
        let want = Fehler::Spielstand(format!("spielstand: Rüstungsstufe {level} außerhalb 0..2"));
        assert_eq!(validate_economy(&v, &h), Err(want));
    }
}

/// Go: `TestSpielstandWirtschaftRundlauf` (save_economy_test.go) braucht Insel, `ToSave` und `FromIslandSave`.
#[test]
fn spielstand_wirtschaft_rundlauf() {}

/// Go: `TestSpielstandWirtschaftFixture` (save_economy_test.go) braucht die Fixtures v1 bis v7 und `FromIslandSave`.
#[test]
fn spielstand_wirtschaft_fixture() {}

/// Go: `TestSpielstandUnbekannteTruppenartFehler` (save_economy_test.go) braucht `FromIslandSave`, `is_fighter` und
/// `profession_of`.
#[test]
fn spielstand_unbekannte_truppenart_fehler() {}
