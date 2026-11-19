//! B-1107: `LiveSimView` rechnet Wirtschaft, Zahlbarkeit und Lair mit der Insel-Sicht der Sim (`World::island_view`),
//! wie Go mit `w.island`; ohne Insel bleiben die Werte wie bei `w.island == nil`.

use k3c_sim::state::{IslandView, Player, Site, World};
use k3c_sim::world::vals;

use super::pending::LiveSimView;
use super::simview::SimView;

fn view() -> IslandView {
    IslandView {
        stages: 1,
        ..IslandView::default()
    }
}

/// AC-01: Mit Insel hat das Lager ein Maximum (Dev-Bonus zählt dazu), ohne Insel ist es unbegrenzt (0).
#[test]
fn economy_of_zeigt_lager_maximum_der_insel() {
    let mut w = World::default();
    assert_eq!(LiveSimView.economy_of(&w).stock_max, 0, "ohne Insel unbegrenzt");
    w.island_view = Some(view());
    let base = LiveSimView.economy_of(&w).stock_max;
    assert!(base > 0, "Maximum je Hub");
    w.island_view = Some(IslandView {
        stock_bonus: 50,
        ..view()
    });
    let e = LiveSimView.economy_of(&w);
    assert_eq!(e.stock_max, base + 50, "Dev-Bonus");
    assert!(e.purse_max > 0, "Beutel-Maximum mit Bonus");
}

/// AC-02: `cheap_build` der Insel drückt den Rest-Preis eines Bauplatzes auf höchstens 1 Gold; `site_payable` bleibt.
#[test]
fn site_gold_left_rechnet_mit_cheap_build() {
    let mut w = World {
        hub_level: 9,
        ..World::default()
    };
    let v = vals(&w);
    let Some((kind, price)) = v
        .buildings
        .iter()
        .map(|(k, b)| (k.clone(), b.cost.gold))
        .find(|(_, g)| *g > 1)
    else {
        unreachable!("kein Gebäude mit Preis über 1 in data/");
    };
    let s = Site {
        kind,
        state: "unpaid".to_string(),
        ..Site::default()
    };
    assert_eq!(
        LiveSimView.site_gold_left(&w, &s),
        (price, true),
        "ohne Insel voller Preis"
    );
    w.island_view = Some(IslandView {
        cheap_build: true,
        ..view()
    });
    assert_eq!(LiveSimView.site_gold_left(&w, &s), (1, true), "cheap_build");
    assert!(LiveSimView.site_payable(&w, &s));
}

/// AC-03: Der Bau des Endbosses löst aus, solange er wartet; Endboss besiegt oder im Kampf sperrt ihn; ohne Insel nie.
#[test]
fn at_lair_folgt_der_insel_sicht() {
    let mut w = World::default();
    w.level.entities.push(Default::default());
    if let Some(e) = w.level.entities.last_mut() {
        e.kind = "lair".to_string();
        e.x = 100.0;
    }
    let mut boss = k3c_data::bosses::BossData {
        id: "warden".to_string(),
        ..Default::default()
    };
    boss.lair.notice_units = 10.0;
    w.lair_boss = Some(boss);
    let p = Player {
        x: 100.0,
        ..Player::default()
    };
    assert!(!LiveSimView.at_lair(&w, &p), "ohne Insel");
    w.island_view = Some(view());
    assert!(LiveSimView.at_lair(&w, &p), "Bau wartet");
    w.island_view = Some(IslandView {
        lair_closed: true,
        ..view()
    });
    assert!(!LiveSimView.at_lair(&w, &p), "Endboss besiegt oder im Kampf");
}
