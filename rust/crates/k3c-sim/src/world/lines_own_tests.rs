//! Eigene Tests der verteilten Plätze ohne Go-Gegenstück (Hand-Welt ohne `create_world`): `level_site`, `line_open`, der
//! Freiraum und die Platzliste `world_site_specs`. Die Go-Tests dazu stehen in `lines_tests.rs`.

use std::sync::Arc;

use k3c_data::hub::{HubData, HubSite};
use k3c_level::types::WallSite;

use super::hub_of;
use super::lines::{
    blockers_at, level_site, line_open, site_blockers, site_cleared, site_needs_clearing, world_site_specs,
};
use super::testutil::*;
use crate::state::{ResourceNode, Site, World};

/// Eine Hand-Welt mit einer Mauer und einem Turm links (Zone 1) und einer Mauer rechts (Zone 2): Plätze und Layout stimmen.
fn line_world() -> World {
    let mut w = bare_world();
    for (side, kind, x, zone) in [(-1, "wall", 150.0, 1), (-1, "tower", 160.0, 1), (1, "wall", 280.0, 2)] {
        w.level.sites.push(WallSite {
            side,
            kind: kind.into(),
            x,
            zone,
        });
        let id = w.new_id();
        w.sites.push(Site {
            id,
            kind: kind.into(),
            x,
            state: "unpaid".into(),
            ..Site::default()
        });
    }
    w
}

fn tree_at(w: &mut World, kind: &str, x: f64) -> usize {
    let id = w.new_id();
    w.nodes.push(ResourceNode {
        id,
        kind: kind.into(),
        x,
        ..ResourceNode::default()
    });
    w.nodes.len() - 1
}

#[test]
fn own_level_site_sucht_nach_art_und_x() {
    let mut w = line_world();
    let wall = &w.sites[0];
    let l = level_site(&w, wall).expect("Mauer links");
    assert_eq!((l.side, l.zone), (-1, 1));
    let site_line = site_line(&w, wall).expect("Zone");
    assert_eq!((site_line.side, site_line.index, site_line.wall), (-1, 1, 150.0));
    let foreign = built_site(&mut w, "wall", 150.5); // gleiche Art, anderes X
    assert!(level_site(&w, &w.sites[foreign]).is_none());
    let barracks = built_site(&mut w, "barracks", 150.0); // gleiches X, andere Art
    assert!(level_site(&w, &w.sites[barracks]).is_none());
}

#[test]
fn own_line_open_zone_n_ab_hub_stufe_n() {
    let mut w = line_world();
    assert_eq!(w.hub_level, 0);
    w.hub_level = 1;
    assert!(line_open(&w, &w.sites[0]), "Zone 1 ab Hub-Stufe 1");
    assert!(line_open(&w, &w.sites[1]));
    assert!(!line_open(&w, &w.sites[2]), "Zone 2 erst ab Hub-Stufe 2");
    w.hub_level = 2;
    assert!(line_open(&w, &w.sites[2]), "ohne gebaute Mauern der Zone 1");
    let other = built_site(&mut w, "barracks", 10.0); // kein verteilter Platz: immer offen
    w.hub_level = 0;
    assert!(line_open(&w, &w.sites[other]));
}

#[test]
fn own_freiraum_sperrt_bis_zum_abbau() {
    let mut w = line_world();
    w.hub_level = 1;
    let radius = clear_radius();
    assert!(radius > 0.0);
    let x = w.sites[0].x;
    assert!(site_cleared(&w, x));
    let tree = tree_at(&mut w, "tree", x + radius - 1.0);
    assert!(!site_cleared(&w, x));
    assert!(!line_open(&w, &w.sites[0]));
    assert!(site_needs_clearing(&w, &w.sites[0]), "Stufe erreicht, aber Baum im Weg");
    assert_eq!(site_blockers(&w, &w.sites[0]), [tree]);
    w.nodes[tree].x = x + radius + 1.0;
    assert!(site_cleared(&w, x));
    assert!(!site_needs_clearing(&w, &w.sites[0]));
    let rock = tree_at(&mut w, "rock", x);
    assert_eq!(blockers_at(&w, x), [rock]);
    w.nodes[rock].gone = true;
    assert!(blockers_at(&w, x).is_empty(), "abgebaute Felsen sperren nicht");
    tree_at(&mut w, "bush", x); // Büsche sperren nicht
    assert!(site_cleared(&w, x));
    assert!(line_open(&w, &w.sites[0]));
}

#[test]
fn own_site_needs_clearing_nur_fuer_unbezahlte_verteilte_plaetze_der_erreichten_stufe() {
    let mut w = line_world();
    let x = w.sites[2].x;
    tree_at(&mut w, "tree", x);
    w.hub_level = 1;
    assert!(!site_needs_clearing(&w, &w.sites[2]), "Zone 2 noch nicht erreicht");
    w.hub_level = 2;
    assert!(site_needs_clearing(&w, &w.sites[2]));
    w.sites[2].state = "built".into();
    assert!(!site_needs_clearing(&w, &w.sites[2]), "gebaute Plätze nicht");
    w.sites[2].state = "unpaid".into();
    let other = built_site(&mut w, "barracks", x);
    w.sites[other].state = "unpaid".into();
    assert!(!site_needs_clearing(&w, &w.sites[other]), "kein verteilter Platz");
    assert!(site_blockers(&w, &w.sites[other]).is_empty());
}

#[test]
fn own_world_site_specs_filtert_nach_tiefe_und_sortiert_stabil_nach_x() {
    let mut w = line_world();
    w.hub_x = 200.0;
    w.biome.depth = 1;
    let hub = HubData {
        sites: vec![
            HubSite {
                kind: "barracks".into(),
                offset_units: 80.0, // 280: gleiche Lage wie die Mauer rechts, kommt vor ihr (stabil)
                from_depth: 0,
                ..HubSite::default()
            },
            HubSite {
                kind: "deep".into(),
                offset_units: -10.0,
                from_depth: 2, // erst ab Tiefe 2
                ..HubSite::default()
            },
            HubSite {
                kind: "stairs".into(),
                offset_units: 5.0,
                from_depth: 0,
                needs_deeper: true, // Tiefe 2 gibt es
                ..HubSite::default()
            },
        ],
        ..HubData::default()
    };
    w.hub_data = Some(Arc::new(hub));
    assert_eq!(hub_of(&w).sites.len(), 3);
    let got: Vec<(String, f64)> = world_site_specs(&w)
        .expect("Daten")
        .into_iter()
        .map(|s| (s.kind, s.x))
        .collect();
    let want = [
        ("wall", 150.0),
        ("tower", 160.0),
        ("stairs", 205.0),
        ("barracks", 280.0),
        ("wall", 280.0),
    ];
    let want: Vec<(String, f64)> = want.iter().map(|(k, x)| (k.to_string(), *x)).collect();
    assert_eq!(got, want);
}

#[test]
fn own_world_site_specs_laesst_plaetze_ohne_tiefere_stufe_weg() {
    let mut w = bare_world();
    w.hub_x = 200.0;
    w.biome.depth = 9_999; // kein Biom mit Tiefe 10000
    let hub = HubData {
        sites: vec![HubSite {
            kind: "stairs".into(),
            offset_units: 5.0,
            needs_deeper: true,
            ..HubSite::default()
        }],
        ..HubData::default()
    };
    w.hub_data = Some(Arc::new(hub));
    assert!(world_site_specs(&w).expect("Daten").is_empty());
}
