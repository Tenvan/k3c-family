//! Tests zur Doppel-Truhe (RU62.2/RU62.4, B-998): wirkendes Kennzeichen zeichnet die Doppel-Truhe, sonst die normale;
//! der wartende Monarch lässt sein Schloss leuchten; aufgehoben zeigt sie das doppelte Offen-Bild.

use k3c_sim::state::coop_view::{CoopChest, CoopState};
use k3c_sim::state::{Pickup, World};

use super::listen::{Opening, StageInfo};
use super::plan::{Item, Planner, Shape};
use super::testutil::{StubSheets, camera};

fn chest_world(coop: Vec<CoopChest>) -> World {
    World {
        pickups: vec![Pickup {
            id: 3,
            kind: "chest".to_string(),
            x: 10.0,
        }],
        coop: CoopState {
            chests: coop,
            ..Default::default()
        },
        ..Default::default()
    }
}

fn double(waiting: Option<i64>) -> CoopChest {
    CoopChest {
        id: 3,
        x: 10.0,
        double: true,
        waiting,
    }
}

fn chest_sprite(items: &[Item]) -> (String, String) {
    let it = items
        .iter()
        .find(|it| it.shape == Shape::Sprite)
        .expect("Truhe gezeichnet");
    (it.look.asset.to_string(), it.look.anim.to_string())
}

fn draw(w: &World) -> Vec<Item> {
    let mut p = Planner::new(Some(StubSheets::rc()));
    p.pickups(w, None, &camera(10.0), 0.0);
    p.body
}

#[test]
fn ohne_wirkendes_kennzeichen_steht_die_normale_truhe() {
    let (asset, _) = chest_sprite(&draw(&chest_world(Vec::new())));
    assert_eq!(asset, "objekte/truhe");
}

#[test]
fn wirkende_doppel_truhe_und_leuchtendes_schloss() {
    for (waiting, anim) in [(None, "zu"), (Some(0), "links"), (Some(1), "rechts")] {
        let (asset, a) = chest_sprite(&draw(&chest_world(vec![double(waiting)])));
        assert_eq!(asset, "objekte/doppel-truhe", "Doppel-Truhe bei {waiting:?}");
        assert_eq!(a, anim, "Schloss bei {waiting:?}");
    }
}

#[test]
fn aufgehobene_doppel_truhe_zeigt_das_doppelte_offen_bild() {
    let mut p = Planner::new(Some(StubSheets::rc()));
    let mut w = chest_world(vec![double(Some(0))]);
    p.pickups(&w, None, &camera(10.0), 0.0);
    w.pickups.clear();
    w.coop.chests.clear();
    p.body.clear();
    let info = StageInfo {
        openings: vec![Opening { x: 10.0, start: 0.0 }],
        ..Default::default()
    };
    p.pickups(&w, Some(&info), &camera(10.0), 0.1);
    let (asset, _) = chest_sprite(&p.body);
    assert_eq!(asset, "objekte/doppel-truhe-offen");
    // eine normale Truhe an anderer Stelle bleibt beim normalen Offen-Bild
    p.body.clear();
    let info = StageInfo {
        openings: vec![Opening { x: 40.0, start: 0.0 }],
        ..Default::default()
    };
    p.pickups(&w, Some(&info), &camera(40.0), 0.1);
    assert_eq!(chest_sprite(&p.body).0, "objekte/truhe-offen");
}
