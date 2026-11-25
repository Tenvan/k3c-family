//! Tests zu `bottle.rs` (GS4.7, B-980/AC-02 bis AC-05): Radar-Marker, Pfeilrichtung je Zelle, Geschichtszeile nach dem
//! Aufheben in allen Hälften, Ende der Markierung, Laden ohne Zeilen.

use std::rc::Rc;

use k3c_sim::bottle::BottleTarget;
use k3c_sim::state::{Site, World};
use serde_json::json;

use super::{ArrowAim, arrow_aim, arrow_dir, bottle_markers};
use crate::hud::radar::RadarKind;
use crate::hud::testutil::{hud, two_players};
use crate::view::bottle::goal_in;
use crate::view::cell::Camera;
use crate::view::frame::{Event, Frame, Stage};

/// Kamera mit sichtbarem Bereich 30 bis 70 Units.
fn kamera() -> Camera {
    Camera {
        x: 50.0,
        scale: 1.0,
        center: 0.0,
        half: 20.0,
    }
}

fn aim(x: f64, dy: i8) -> Option<ArrowAim> {
    Some(ArrowAim { x, dy })
}

/// Frame mit zwei Stufen (0: Tiefe 0 mit Treppe nach unten bei 90, 1: Tiefe 1); Ziel auf Stufe `ts` bei `tx`.
fn zwei_stufen(target: Option<(usize, f64)>) -> Frame {
    let mut home = World::default();
    home.biome.depth = 0;
    home.width_units = 200.0;
    home.sites.push(Site {
        kind: "stairsDown".to_string(),
        x: 90.0,
        ..Default::default()
    });
    home.bottle.target = target.map(|(stage, x)| BottleTarget { stage, x });
    let mut deep = World::default();
    deep.biome.depth = 1;
    deep.sites.push(Site {
        kind: "stairsUp".to_string(),
        x: 10.0,
        ..Default::default()
    });
    Frame {
        stages: vec![
            Stage {
                index: 0,
                depth: 0,
                world: Some(Rc::new(home)),
                ..Default::default()
            },
            Stage {
                index: 1,
                depth: 1,
                world: Some(Rc::new(deep)),
                ..Default::default()
            },
        ],
        ..Default::default()
    }
}

#[test]
fn radar_marker_flasche_und_ziel() {
    let mut w = World::default();
    w.biome.depth = 0;
    w.width_units = 100.0;
    w.bottle.ashore = Some(25.0);
    let mut out = Vec::new();
    bottle_markers(&mut out, &w, Some(75.0));
    assert_eq!(out.len(), 2);
    assert_eq!((out[0].kind, out[0].pos), (RadarKind::Bottle, 0.25));
    assert_eq!((out[1].kind, out[1].pos), (RadarKind::Goal, 0.75));
    out.clear();
    w.bottle.ashore = None;
    bottle_markers(&mut out, &w, None);
    assert!(out.is_empty(), "ohne Zustand keine Marker");
}

#[test]
fn pfeil_richtung_links_rechts_im_bild() {
    let k = kamera();
    assert_eq!(arrow_dir(aim(10.0, 0), &k), Some((-1, 0)), "Ziel links");
    assert_eq!(arrow_dir(aim(120.0, 0), &k), Some((1, 0)), "Ziel rechts");
    assert_eq!(arrow_dir(aim(50.0, 0), &k), None, "Ziel im Bild: kein Pfeil");
    assert_eq!(arrow_dir(None, &k), None, "keine Markierung");
}

#[test]
fn pfeil_richtung_tiefer_und_hoeher() {
    let k = kamera();
    assert_eq!(
        arrow_dir(aim(50.0, 1), &k),
        Some((0, 1)),
        "Übergang im Bild: nach unten"
    );
    assert_eq!(
        arrow_dir(aim(90.0, 1), &k),
        Some((1, 1)),
        "Übergang rechts: schräg unten"
    );
    assert_eq!(
        arrow_dir(aim(10.0, -1), &k),
        Some((-1, -1)),
        "Übergang links: schräg oben"
    );
    assert_eq!(arrow_dir(aim(50.0, -1), &k), Some((0, -1)), "höher: nach oben");
}

#[test]
fn zwei_spieler_in_verschiedenen_stufen_je_eigene_richtung() {
    let f = zwei_stufen(Some((1, 150.0)));
    let w0 = f.stages[0].world.as_deref().expect("Welt");
    let w1 = f.stages[1].world.as_deref().expect("Welt");
    // Spieler in der oberen Stufe: Ziel tiefer, nächster Übergang nach unten ist die Treppe bei 90
    assert_eq!(arrow_aim(&f, 0, w0, 60.0), aim(90.0, 1));
    // Spieler in der Stufe des Ziels: Pfeil zum Ziel selbst
    assert_eq!(arrow_aim(&f, 1, w1, 60.0), aim(150.0, 0));
    assert_eq!(goal_in(&f, 1), Some(150.0));
    assert_eq!(goal_in(&f, 0), None);
    // Ziel auf der oberen Stufe, Spieler unten: höher über die Treppe bei 10
    let g = zwei_stufen(Some((0, 20.0)));
    assert_eq!(
        arrow_aim(&g, 1, g.stages[1].world.as_deref().expect("Welt"), 60.0),
        aim(10.0, -1)
    );
}

#[test]
fn ziel_erreicht_loescht_pfeil() {
    let f = zwei_stufen(None);
    let w0 = f.stages[0].world.as_deref().expect("Welt");
    assert_eq!(arrow_aim(&f, 0, w0, 60.0), None, "ohne Markierung kein Pfeil");
    assert_eq!(goal_in(&f, 0), None);
}

fn found(line: i64) -> Event {
    Event {
        data: json!({"type": "bottleFound", "player": 0, "n": 1, "line": line})
            .as_object()
            .expect("Objekt")
            .clone(),
        ..Default::default()
    }
}

#[test]
fn aufheben_blendet_zeilen_in_allen_haelften_ein() {
    let (_, mut f) = two_players(0);
    f.campaign.bottles = Rc::new(vec![vec!["Eins".to_string(), "Zwei".to_string()]]);
    let mut h = hud();
    h.listen(&f);
    assert!(h.seats[0].story.showing.a.is_empty());
    f.events = vec![found(0)];
    h.listen(&f);
    for p in 0..2 {
        assert_eq!(h.seats[p].story.showing.a, "Eins", "Hälfte {p}");
        assert_eq!(h.seats[p].story.showing.b, "Zwei", "Hälfte {p}");
    }
}

#[test]
fn ohne_zeilen_erscheint_nichts() {
    let (_, mut f) = two_players(0);
    let mut h = hud();
    h.listen(&f);
    f.events = vec![found(-1)];
    h.listen(&f);
    assert!(h.seats[0].story.showing.a.is_empty(), "line -1");
    f.campaign.bottles = Rc::new(vec![vec!["x".to_string()]]);
    f.events = vec![found(5)];
    h.listen(&f);
    assert!(h.seats[1].story.showing.a.is_empty(), "Eintrag fehlt");
}

#[test]
fn geladener_stand_zeigt_markierung_ohne_geschichtszeile() {
    let mut f = zwei_stufen(Some((0, 20.0)));
    f.campaign.bottles = Rc::new(vec![vec!["Eins".to_string()]]);
    let mut h = hud();
    h.listen(&f);
    assert!(goal_in(&f, 0).is_some(), "Markierung steht sofort im Zustand");
    assert!(h.seats[0].story.showing.a.is_empty(), "ohne Ereignis keine Zeile");
}
