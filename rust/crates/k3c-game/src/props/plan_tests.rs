//! Tests zu `plan.go`. Die sieben Go-Tests bauen ihre Welt mit `sim.CreateIsland` (Rust: RU10); sie stehen hier mit
//! `#[ignore = "wartet auf RU10"]` und `island()` als Platzhalter (`plan` samt `decor` läuft seit RU16.5). `sites`,
//! `combat`, `plantation`, `pickups` und `coins` liefert RU16.4; deren Aussagen laufen als Literal-Welten in
//! `plan_sites_tests.rs`, `plan_combat_tests.rs` und `felling_tests.rs`. Dazu laufen Tests auf Literal-Welten über
//! `castle`, `entities` und `nodes`.
#![allow(clippy::unimplemented)] // Platzhalter: wartet auf den genannten Sprint (RU10, RU15, RU18, RU23)

use k3c_level::types::Entity;
use k3c_sim::state::{ResourceNode, Site, World};

use super::buildings::DAMAGE_SHADE;
use super::data::site_name;
use super::listen::{HubState, StageInfo};
use super::plan::{Item, Planner, Shape, px};
use super::plan_castle::bits_arr;
use super::testutil::{FixedSimView, camera, fake_sheets};

/// Go: `island(t)`: die erste Stufe einer neuen Insel der Standard-Welt (`create_island`, RU10).
fn island() -> World {
    let mut data = k3c_data::load_all(&k3c_data::Source::Embedded).expect("eingebettete Daten");
    // früheres Layout der Insel 1 (der Fünf-Inseln-Standard gibt ihr 16 Chunks): eine Insel, Hafen auf Platz -7
    let one = r#"{"id":"island1","name":"Insel 1","depths":[0,1,2,3,4],"depthScaling":{"hp":1.5,"damage":1.3,"speed":1.1},"plannedChunks":{"0":[{"slot":-7,"kind":"harbor"}]}}"#;
    data.standard_world.islands = Some(vec![serde_json::from_str(one).expect("Insel")]);
    // Die Geschichte der Standard-Welt kennt mehr Etappen als diese eine Insel: nur Etappe 1 bleibt (sonst „Etappe 2 hat keine Insel“).
    if let Some(etappen) = data
        .standard_world
        .story
        .get_mut("etappen")
        .and_then(|v| v.as_object_mut())
    {
        etappen.retain(|k, _| k == "1");
    }
    let hub = k3c_data::Source::Embedded.read("hub.json").expect("hub.json");
    let rules = k3c_level::rules::Rules::from_data(&data, &hub).expect("Regeln");
    let world = k3c_sim::island::worlds::standard_world(&data, &rules).expect("Standard-Welt");
    let isl =
        k3c_sim::island::create::create_island(&data, &world, "plan", &[0], 1.0, Default::default()).expect("Insel");
    isl.stages[0].clone()
}

fn assets_of(items: &[Item]) -> Vec<String> {
    items
        .iter()
        .filter(|it| matches!(it.shape, Shape::Sprite | Shape::Tiles))
        .map(|it| format!("{}:{}", it.look.asset, it.look.anim))
        .collect()
}

fn has_text(items: &[Item], part: &str) -> bool {
    items.iter().any(|it| it.shape == Shape::Text && it.text.contains(part))
}

fn labels_of(items: &[Item]) -> Vec<String> {
    items
        .iter()
        .filter(|it| it.shape == Shape::Text)
        .map(|it| it.text.clone())
        .collect()
}

#[test]
fn plan_burg_und_plaetze_aus_der_simulation() {
    let Some(lib) = fake_sheets() else {
        return;
    };
    let w = island();
    let v = FixedSimView::default();
    let mut p = Planner::new(Some(lib));
    let mut info = StageInfo::default();
    info.update(&w, 0.0, &v);
    p.plan(Some(&w), Some(&info), &camera(w.castle.x), 0.0, &v);
    let got = assets_of(&p.body).join(" ");
    assert!(
        got.contains("gebaeude-paper/lagerfeuer:stand"),
        "Burg der Hub-Stufe 1 fehlt: {got}"
    );
}

#[test]
fn plan_gebaute_stufen_schaden_und_ruine() {
    let Some(lib) = fake_sheets() else {
        return;
    };
    let mut w = island();
    let v = FixedSimView::default();
    let mut p = Planner::new(Some(lib));
    let wall = w
        .sites
        .iter_mut()
        .find(|s| s.kind == "wall")
        .expect("keine Mauer im Hub");
    *wall = Site {
        id: wall.id,
        kind: "wall".to_string(),
        x: wall.x,
        state: "built".to_string(),
        level: 3,
        hp: 100.0,
        max_hp: 600.0,
        ruin: true,
        ..Default::default()
    };
    let wx = wall.x;
    p.plan(Some(&w), None, &camera(wx), 0.0, &v);
    let sprite = p
        .body
        .iter()
        .rfind(|it| it.shape == Shape::Sprite && it.look.asset == "gebaeude-paper/mauer")
        .expect("Mauer Stufe 3, schwer beschädigt");
    assert_eq!(sprite.look.anim, "stand.s3");
    assert_eq!(sprite.shade, DAMAGE_SHADE[3]);
    let lines = p.top.iter().filter(|it| it.shape == Shape::Line).count();
    let rects = p.top.iter().filter(|it| it.shape == Shape::Rect).count();
    // `rubbleStones` (plan_sites.go, RU16.4) hat 6 Einträge.
    assert!(lines > 0 && rects >= 6, "Risse {lines}, Trümmer und Balken {rects}");
}

#[test]
#[ignore = "wartet auf RU10"]
fn plan_ohne_grafik_platzhalter() {
    let w = island();
    let v = FixedSimView::default();
    let mut p = Planner::new(None);
    p.plan(Some(&w), None, &camera(w.castle.x), 0.0, &v);
    assert_eq!(assets_of(&p.body).len(), 0, "ohne Grafik Bilder");
    assert!(!p.body.is_empty(), "keine Platzhalter-Formen");
    p.plan(None, None, &camera(0.0), 0.0, &v);
    assert_eq!(p.body.len() + p.top.len(), 0, "ohne Welt nichts");
}

#[test]
#[ignore = "wartet auf RU10"]
fn plan_nur_sichtbares() {
    let w = island();
    let v = FixedSimView::default();
    let mut p = Planner::new(fake_sheets());
    p.plan(Some(&w), None, &camera(-10000.0), 0.0, &v);
    assert_eq!(p.body.len() + p.top.len(), 0, "weit außerhalb der Welt");
}

#[test]
fn plan_baustelle_mit_preis_und_geruest() {
    let Some(lib) = fake_sheets() else {
        return;
    };
    let mut w = island();
    let v = FixedSimView {
        payable: true,
        gold_left: (10, true),
        ..Default::default()
    };
    let mut p = Planner::new(Some(lib));
    let mut info = StageInfo::default();
    info.update(&w, 0.0, &v);
    let i = info
        .flags
        .iter()
        .position(|f| f.payable)
        .expect("kein bezahlbarer Platz");
    let (id, x, kind) = (w.sites[i].id, w.sites[i].x, w.sites[i].kind.clone());
    info.tags = vec![id];
    p.plan(Some(&w), Some(&info), &camera(x), 0.0, &v);
    assert!(has_text(&p.top, &site_name(&kind)), "Preis-Anzeige von {kind} fehlt");
    w.sites[i].state = "waitingWorker".to_string();
    w.sites[i].build_progress = 0.5;
    info.update(&w, 0.0, &v);
    p.plan(Some(&w), Some(&info), &camera(x), 0.0, &v);
    assert!(assets_of(&p.body).join(" ").contains("geruest"), "Gerüst im Bau fehlt");
}

/// Go prüft null Allokationen je Bild (`testing.AllocsPerRun`); Rust hat keinen Allokationszähler: Nach dem zweiten
/// Planen bleibt die Kapazität der Puffer unverändert.
#[test]
#[ignore = "wartet auf RU10"]
fn plan_ohne_allokation_nach_dem_ersten_bild() {
    let w = island();
    let v = FixedSimView::default();
    let mut p = Planner::new(fake_sheets());
    let mut info = StageInfo::default();
    info.update(&w, 0.0, &v);
    let k = camera(w.castle.x);
    p.plan(Some(&w), Some(&info), &k, 0.0, &v);
    p.plan(Some(&w), Some(&info), &k, 1.0, &v);
    let caps = (
        p.body.capacity(),
        p.top.capacity(),
        p.walls.capacity(),
        p.backs.capacity(),
        p.roots.capacity(),
    );
    p.plan(Some(&w), Some(&info), &k, 1.0, &v);
    let after = (
        p.body.capacity(),
        p.top.capacity(),
        p.walls.capacity(),
        p.backs.capacity(),
        p.roots.capacity(),
    );
    assert_eq!(caps, after, "Puffer wachsen je Bild");
}

#[test]
#[ignore = "wartet auf RU10"]
fn plan_ausbau_schild_der_burg() {
    let w = island();
    let v = FixedSimView::default();
    let mut p = Planner::new(fake_sheets());
    let mut info = StageInfo {
        hub_level: 2,
        hub_step: 2,
        stock: [0, 10, 0, 0, 0],
        hub: HubState {
            has: true,
            gold: 50,
            paid: 10,
            steps: 3,
            material: [0, 100, 0, 0, 0],
            ..Default::default()
        },
        ..Default::default()
    };
    info.tags = vec![w.castle.id];
    p.plan(Some(&w), Some(&info), &camera(w.castle.x), 0.0, &v);
    assert!(
        has_text(&p.top, "Ausbau 2/3 · 100 Stein"),
        "Angebot fehlt: {:?}",
        labels_of(&p.top)
    );
    let rings = p.top.iter().filter(|it| it.shape == Shape::Ring).count();
    assert_eq!(rings, 50, "50 Münz-Plätze erwartet");
    info.hub.state = "waitingNextIsland".to_string();
    info.hub.paid = 50;
    p.plan(Some(&w), Some(&info), &camera(w.castle.x), 0.0, &v);
    assert!(
        has_text(&p.top, "Ausbau 2/3: Tor-Material fehlt (Stein)"),
        "Wartegrund fehlt: {:?}",
        labels_of(&p.top)
    );
}

// ---- Literal-Welten: Burg, Objekte und Natur laufen ohne Insel und ohne die Planer-Methoden aus RU16.4/RU16.5 ----

fn node(kind: &str, x: f64) -> ResourceNode {
    ResourceNode {
        kind: kind.to_string(),
        x,
        ..Default::default()
    }
}

fn entity(kind: &str, x: f64) -> Entity {
    Entity {
        kind: kind.to_string(),
        x,
        ..Default::default()
    }
}

#[test]
fn castle_entities_nodes_nur_sichtbares() {
    let mut w = World::default();
    w.level.entities = vec![entity("exit", 10.0), entity("portal", 14.0)];
    w.nodes = vec![node("tree", 5.0), node("rock", 8.0)];
    let mut p = Planner::new(None);
    let far = camera(-10000.0);
    p.castle(&w, None, &far);
    p.entities(&w, &far, "forest");
    p.nodes(&w, &far, "forest");
    assert_eq!(p.body.len() + p.top.len(), 0, "weit außerhalb der Welt");
    let near = camera(8.0);
    p.castle(&w, None, &near);
    p.entities(&w, &near, "forest");
    p.nodes(&w, &near, "forest");
    assert!(!p.body.is_empty(), "in der Kamera stehen die Platzhalter");
}

#[test]
fn entities_und_nodes_stehen_an_den_positionen() {
    let mut w = World::default();
    w.level.entities = vec![entity("exit", 20.0), entity("portal", 30.0), entity("unbekannt", 40.0)];
    w.portals_down = vec![30.0];
    w.nodes = vec![node("tree", 5.0), node("rock", 8.0)];
    w.nodes[0].marked = true;
    w.nodes[0].progress = 0.5;
    let mut p = Planner::new(None);
    let k = camera(20.0);
    p.entities(&w, &k, "forest");
    assert_eq!(
        p.body.len(),
        1,
        "Portal ist zerstört, unbekannte Art fehlt: {:?}",
        p.body
    );
    assert_eq!((p.body[0].shape, p.body[0].x), (Shape::Rect, 20.0));
    assert!(has_text(&p.top, "alle hierher") || has_text(&p.top, "verschüttet"));
    p.body.clear();
    p.top.clear();
    p.nodes(&w, &camera(6.0), "forest");
    let at = |x: f64, shape: Shape| p.body.iter().any(|it| it.shape == shape && (it.x - x).abs() < 1e-9);
    assert!(
        at(5.0, Shape::Rect) && at(5.0, Shape::Tri),
        "Baum aus Stamm und Krone bei x=5"
    );
    assert!(at(5.0 - px(24.0), Shape::Rect), "Abbau-Schild am markierten Baum");
    assert!(
        p.body.iter().any(|it| it.shape == Shape::Circle && it.x == 8.0),
        "Fels bei x=8"
    );
    assert_eq!(p.top.len(), 2, "Fortschrittsbalken: Hintergrund und Anteil");
}

#[test]
fn burg_balken_und_platzhalter() {
    let mut w = World::default();
    w.castle.x = 3.0;
    w.castle.hp = 50.0;
    w.castle.max_hp = 100.0;
    let mut p = Planner::new(None);
    p.castle(&w, None, &camera(3.0));
    assert!(
        p.body.iter().any(|it| it.shape == Shape::Frame),
        "Platzhalter-Burg mit Rahmen"
    );
    assert_eq!(p.top.len(), 2, "Lebensbalken der verletzten Burg");
    assert_eq!(bits_arr([0, 100, 0, 0, 1]), 0b10010);
}
