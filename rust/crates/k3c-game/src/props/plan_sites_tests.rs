//! Tests zu `plan_sites.go`, `plan_objects.go` (Pickups, Münzen) und den Platz-Zeichnern. Go hat dafür keinen eigenen
//! Test (geprüft über `plan_test.go`, dessen Tests die Insel aus RU10 und `decor` aus RU16.5 brauchen); hier laufen
//! dieselben Aussagen auf Literal-Welten über `sites`, `back_walls`, `pickups` und `coins`.

use k3c_sim::state::{Coin, Pickup, Site, World};

use super::buildings::DAMAGE_SHADE;
use super::data::site_name;
use super::listen::{HubState, Opening, SiteFlags, StageInfo};
use super::plan::{Item, Planner, Shape, px};
use super::testutil::{StubSheets, camera, wall_at};

fn has_text(items: &[Item], part: &str) -> bool {
    items.iter().any(|it| it.shape == Shape::Text && it.text.contains(part))
}

fn assets_of(items: &[Item]) -> String {
    items
        .iter()
        .filter(|it| matches!(it.shape, Shape::Sprite | Shape::Tiles))
        .map(|it| format!("{}:{}", it.look.asset, it.look.anim))
        .collect::<Vec<_>>()
        .join(" ")
}

fn info_for(flags: Vec<SiteFlags>, tags: Vec<i64>) -> StageInfo {
    StageInfo {
        flags,
        tags,
        stock: [50, 50, 50, 50, 50],
        hub: HubState::default(),
        ..Default::default()
    }
}

fn open_site(state: &str, kind: &str) -> Site {
    Site {
        id: 7,
        kind: kind.to_string(),
        x: 10.0,
        state: state.to_string(),
        ..Default::default()
    }
}

#[test]
fn baustelle_mit_preis_und_geruest() {
    let flags = SiteFlags {
        visible: true,
        payable: true,
        gold_left: 4,
        ..Default::default()
    };
    let mut w = World {
        sites: vec![open_site("unpaid", "tower")],
        ..Default::default()
    };
    let info = info_for(vec![flags], vec![7]);
    let mut p = Planner::new(Some(StubSheets::rc()));
    p.sites(&w, Some(&info), &camera(10.0));
    assert!(has_text(&p.top, &site_name("tower")), "Preis-Anzeige fehlt");
    assert_eq!(
        p.top.iter().filter(|it| it.shape == Shape::Ring).count(),
        4,
        "4 Münz-Plätze"
    );
    assert!(p.body.iter().any(|it| it.shape == Shape::Frame), "Bauschild fehlt");
    w.sites[0].state = "waitingWorker".to_string();
    w.sites[0].build_progress = 0.5;
    p.body.clear();
    p.top.clear();
    p.sites(&w, Some(&info), &camera(10.0));
    assert!(assets_of(&p.body).contains("geruest"), "Gerüst im Bau fehlt");
    assert!(
        p.top
            .iter()
            .any(|it| it.shape == Shape::Rect && (it.w - px(80.0) * 0.5).abs() < 1e-9),
        "Fortschrittsbalken fehlt"
    );
}

#[test]
fn baustelle_ohne_grafik_platzhalter_und_wartegrund() {
    let w = World {
        sites: vec![open_site("waitingMaterial", "tower")],
        ..Default::default()
    };
    let mut p = Planner::new(None);
    p.sites(&w, None, &camera(10.0));
    assert!(
        p.body.iter().any(|it| it.shape == Shape::Rect),
        "Gerüst-Platzhalter fehlt"
    );
    assert!(p.top.iter().any(|it| it.shape == Shape::Rect), "Material-Symbol fehlt");
    assert!(has_text(&p.top, &site_name("tower")), "Wartegrund-Text fehlt");
}

#[test]
fn gebaute_stufe_schaden_und_ruine() {
    let mut wall = wall_at(1, 10.0, 3, "built");
    wall.hp = 100.0;
    wall.max_hp = 600.0;
    wall.ruin = true;
    let w = World {
        sites: vec![wall],
        ..Default::default()
    };
    let mut p = Planner::new(Some(StubSheets::rc()));
    p.sites(&w, None, &camera(10.0));
    let sprite = p
        .body
        .iter()
        .rfind(|it| it.shape == Shape::Sprite && it.look.asset == "gebaeude-paper/mauer")
        .expect("Mauer Stufe 3, schwer beschädigt");
    assert_eq!(sprite.look.anim, "stand.s3");
    assert_eq!(sprite.shade, DAMAGE_SHADE[3]);
    let lines = p.top.iter().filter(|it| it.shape == Shape::Line).count();
    let rects = p.top.iter().filter(|it| it.shape == Shape::Rect).count();
    // 5 Risse (3 + 2 + 2 + 3 + 1 Linien) und 6 Trümmersteine plus Lebensbalken (Hintergrund und Füllung)
    assert_eq!(lines, 11, "Risse");
    assert_eq!(rects, 6 + 2, "Trümmer und Balken");
}

#[test]
fn gebaute_stufe_ohne_grafik_dunkelt_die_form() {
    let mut wall = wall_at(1, 10.0, 1, "built");
    wall.hp = 300.0;
    wall.max_hp = 600.0;
    let w = World {
        sites: vec![wall],
        ..Default::default()
    };
    let mut p = Planner::new(None);
    p.sites(&w, None, &camera(10.0));
    // Stufe 2: 255 · 0,25 = 63
    assert!(
        p.body
            .iter()
            .any(|it| it.shape == Shape::Rect && it.clr == [0, 0, 0, 63]),
        "Abdunklung der Platzhalter-Form fehlt"
    );
}

#[test]
fn nur_sichtbare_plaetze() {
    let w = World {
        sites: vec![wall_at(1, 10.0, 1, "built")],
        ..Default::default()
    };
    let mut p = Planner::new(Some(StubSheets::rc()));
    p.sites(&w, None, &camera(-10000.0));
    assert!(p.body.is_empty() && p.top.is_empty(), "weit außerhalb der Welt");
    let hidden = info_for(vec![SiteFlags::default()], vec![]);
    p.sites(&w, Some(&hidden), &camera(10.0));
    assert!(p.body.is_empty() && p.top.is_empty(), "Platz nicht sichtbar gemeldet");
}

#[test]
fn hintergrundmauer_zwischen_zwei_gebauten_mauern() {
    let w = World {
        hub_x: 0.0,
        sites: vec![wall_at(1, 10.0, 1, "built"), wall_at(2, 20.0, 2, "built")],
        ..Default::default()
    };
    let mut p = Planner::new(Some(StubSheets::rc()));
    p.back_walls(&w, &camera(15.0));
    let tiles: Vec<&Item> = p.body.iter().filter(|it| it.shape == Shape::Tiles).collect();
    assert_eq!(tiles.len(), 1);
    assert_eq!((tiles[0].x, tiles[0].x2), (10.0, 20.0));
    let mut ohne = Planner::new(None);
    ohne.back_walls(&w, &camera(15.0));
    assert!(ohne.body.is_empty(), "ohne Grafik bleibt der Abschnitt leer");
}

#[test]
fn preis_reihen_und_farbe() {
    let mut p = Planner::new(None);
    p.price(0.0, 1.0, 12, 5, "Turm", false);
    let rings = p.top.iter().filter(|it| it.shape == Shape::Ring).count();
    let fills = p.top.iter().filter(|it| it.shape == Shape::Circle).count();
    assert_eq!((rings, fills), (12, 5));
    let label = p.top.last().expect("Text");
    assert_eq!(label.text, "Turm");
    assert_eq!(label.clr, super::plan::POOR_RED, "Material fehlt: rot");
    assert!((label.y - (1.0 + 2.0 * px(22.0))).abs() < 1e-9, "zwei Reihen");
}

#[test]
fn pickups_und_muenzen_an_den_positionen() {
    let w = World {
        pickups: vec![Pickup {
            id: 1,
            kind: "chest".to_string(),
            x: 5.0,
        }],
        coins: vec![Coin {
            x: 6.0,
            ..Default::default()
        }],
        ..Default::default()
    };
    let info = StageInfo {
        openings: vec![Opening { x: 7.0, start: 0.0 }],
        ..Default::default()
    };
    let mut p = Planner::new(Some(StubSheets::rc()));
    p.pickups(&w, Some(&info), &camera(5.0), 0.1);
    p.coins(&w, &camera(5.0));
    let xs: Vec<f64> = p.body.iter().map(|it| it.x).collect();
    assert_eq!(xs, vec![5.0, 7.0, 6.0], "Truhe, offene Truhe, Münze");
    assert_eq!(p.body[1].alpha, 1.0, "frisch geöffnet: voll sichtbar");
    let mut ohne = Planner::new(None);
    ohne.pickups(&w, None, &camera(5.0), 0.0);
    ohne.coins(&w, &camera(5.0));
    assert_eq!(
        ohne.body.iter().map(|it| it.shape).collect::<Vec<_>>(),
        vec![Shape::Rect, Shape::Circle]
    );
}

/// EQ1.2: Das Regal der vier Hersteller zeigt je Stück ein Icon des Gegenstands; ein leeres Regal zeigt keins.
#[test]
fn regal_zeigt_je_stueck_ein_icon() {
    let cases = [
        ("workshop", "icons/icon-guertel", 3),
        ("bowyer", "icons/icon-bogen", 2),
        ("smithy", "icons/icon-schwert", 1),
        ("armory", "icons/icon-ruestung", 3),
    ];
    for (kind, icon, n) in cases {
        let mut site = Site {
            id: 7,
            kind: kind.to_string(),
            x: 10.0,
            state: "built".to_string(),
            ..Default::default()
        };
        match kind {
            "workshop" => site.hammers = n,
            "bowyer" => site.bows = n,
            "smithy" => site.swords = n,
            _ => site.armors = n,
        }
        let w = World {
            sites: vec![site.clone()],
            ..Default::default()
        };
        let mut p = Planner::new(Some(StubSheets::rc()));
        p.sites(&w, None, &camera(10.0));
        let got = p
            .top
            .iter()
            .filter(|it| it.shape == Shape::Sprite && it.look.asset == icon)
            .count();
        assert_eq!(got as i64, n, "{kind}: Icons im Regal");
        site.hammers = 0;
        site.bows = 0;
        site.swords = 0;
        site.armors = 0;
        let w = World {
            sites: vec![site],
            ..Default::default()
        };
        let mut p = Planner::new(Some(StubSheets::rc()));
        p.sites(&w, None, &camera(10.0));
        assert!(
            !p.top.iter().any(|it| it.look.asset == icon),
            "{kind}: leeres Regal ohne Icon"
        );
    }
}
