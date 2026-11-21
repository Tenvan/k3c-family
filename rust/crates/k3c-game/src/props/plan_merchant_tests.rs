use std::rc::Rc;

use k3c_sim::state::{Merchant, World};

use super::listen::StageInfo;
use super::plan::{Item, Planner, Shape, Sheets};
use super::plan_sites::monarch_before;
use crate::view::cell::Camera;

struct AllSheets;

impl Sheets for AllSheets {
    fn size_of(&self, _: &str, _: &str) -> Option<(f64, f64)> {
        Some((1.0, 1.0))
    }
}

fn world(with: bool) -> World {
    World {
        merchant: with.then(|| Merchant {
            resource: "wood".into(),
            hp: 5.0,
            max_hp: 10.0,
            ..Merchant::default()
        }),
        ..World::default()
    }
}

/// HB1.2: Mit Händler stehen zwei Stände samt Material-Icon, ohne ihn nichts.
#[test]
fn haendler_zeichnet_zwei_staende() {
    let mut p = Planner::new(Some(Rc::new(AllSheets)));
    p.merchant(&world(true), &Camera::default());
    let icons = p.top.iter().filter(|i: &&Item| i.shape == Shape::Sprite).count();
    assert_eq!(icons, 2, "ein Icon je Stand");
    assert!(p.body.iter().any(|i| i.shape == Shape::Rect), "Tische und Dächer");
    let mut leer = Planner::new(Some(Rc::new(AllSheets)));
    leer.merchant(&world(false), &Camera::default());
    assert!(leer.body.is_empty() && leer.top.is_empty());
}

/// HB1.2: Der Name erscheint nur, wenn ein Monarch vor dem Gebäude steht.
#[test]
fn monarch_vor_dem_gebaeude() {
    let info = StageInfo {
        players: vec![10.0],
        ..StageInfo::default()
    };
    assert!(monarch_before(Some(&info), 11.0, 4.0));
    assert!(!monarch_before(Some(&info), 20.0, 4.0));
    assert!(!monarch_before(None, 10.0, 4.0));
}
