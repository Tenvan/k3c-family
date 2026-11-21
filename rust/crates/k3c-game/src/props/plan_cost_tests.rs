use std::rc::Rc;

use k3c_data::buildings::Cost;

use super::plan::{Item, Planner, Shape, Sheets};
use super::plan_cost::SMALL;

/// Alle Sheets sind da (1 × 1 Unit).
struct AllSheets;

impl Sheets for AllSheets {
    fn size_of(&self, _: &str, _: &str) -> Option<(f64, f64)> {
        Some((1.0, 1.0))
    }
}

fn cost() -> Cost {
    Cost {
        wood: 20,
        stone: 5,
        ..Cost::default()
    }
}

fn texts(items: &[Item]) -> Vec<&str> {
    items
        .iter()
        .filter(|i| i.shape == Shape::Text)
        .map(|i| i.text.as_str())
        .collect()
}

/// HB1.1: Kosten stehen als Icon mit Zahl, der Name klein darüber.
#[test]
fn kosten_als_icons_mit_zahl_und_kleinem_namen() {
    let mut p = Planner::new(Some(Rc::new(AllSheets)));
    p.price_cost(0.0, 1.0, (3, 1), "Mauer", &cost(), true);
    let icons: Vec<&Item> = p.top.iter().filter(|i| i.shape == Shape::Sprite).collect();
    assert_eq!(icons.len(), 2, "Holz und Stein");
    assert_eq!(icons[0].look.asset, "icons/icon-holz");
    assert_eq!(icons[1].look.asset, "icons/icon-stein");
    assert!(icons.iter().all(|i| i.w > 0.0 && i.w < 1.0), "verkleinert");
    assert_eq!(texts(&p.top), ["20", "5", "Mauer"]);
    assert!(p.top.iter().filter(|i| i.shape == Shape::Text).all(|i| i.size == SMALL));
}

/// Ohne Icon-Sheets steht der Kurztext, ohne Wort „Holz“ zu verlieren.
#[test]
fn ohne_icons_kurztext() {
    let mut p = Planner::new(None);
    p.price_cost(0.0, 1.0, (0, 0), "Mauer", &cost(), false);
    assert!(p.top.iter().all(|i| i.shape != Shape::Sprite));
    assert_eq!(texts(&p.top), ["20 Holz, 5 Stein", "Mauer"]);
}
