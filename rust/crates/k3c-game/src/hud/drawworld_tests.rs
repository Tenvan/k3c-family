//! Tests zu `drawworld.rs`: `TestAnleitungMitBild` (`guide_test.go`: Bild der Anleitung, Lage von Bild und Text) und, zu
//! AC-01 (zwei Spieler), das Radar in beiden Zellen.

use k3c_platform::NullCanvas;

use super::drawworld::guide_layout;
use super::hints::{GuideIcon, GuideId, guide_image};
use super::layout::Rect;
use super::radar::{RadarKind, RadarMarker};
use super::state::SeatState;
use super::testutil::hud;
use crate::view::cell::{Cell, new_camera, rect};

/// Münze und Mond als Bild (B-811 AC-03): Münz-Hinweis mit Münze, Nacht mit Mond, Landstreicher ohne; fehlen die
/// Vorlagen zum Zeichnen, steht der Text allein. Lage: Bild links neben dem Text, beide zusammen mittig.
#[test]
fn anleitung_mit_bild() {
    let cases = [
        (GuideId::Coin, true, GuideIcon::Coin),
        (GuideId::Dusk, true, GuideIcon::Moon),
        (GuideId::Recruit, true, GuideIcon::None),
        (GuideId::Coin, false, GuideIcon::None),
        (GuideId::Dusk, false, GuideIcon::None),
    ];
    for (id, ready, want) in cases {
        assert_eq!(guide_image(id, ready), want, "Hinweis {id:?}, Vorlagen {ready}");
    }
    let (img, tx, ty) = guide_layout(500.0, 300.0, 200.0, 30.0, true, 1.0);
    assert!(
        img.w == 30.0 && img.h == 30.0 && img.y == ty && img.x + img.w <= tx && ty == 270.0,
        "Bild links neben dem Text: {img:?}, Text bei {tx}/{ty}"
    );
    assert_eq!((img.x + tx + 200.0) / 2.0, 500.0, "Bild und Text mittig um x");
    let (img, tx, _) = guide_layout(500.0, 300.0, 200.0, 30.0, false, 1.0);
    assert!(
        img.w == 0.0 && tx == 400.0,
        "ohne Bild nur der Text mittig: {img:?} {tx}"
    );
}

/// AC-01: Das Radar zeichnet je Zelle in die eigene Hälfte; ohne Marker nur Leiste, Rahmen und Ausschnitt, mit Markern
/// kommen Formen dazu (Scheiben, Dreieck als Sprites).
#[test]
fn radar_je_zelle() {
    let mut h = hud();
    let mut st = SeatState {
        width_units: 100.0,
        ..SeatState::default()
    };
    let cells = [rect(0, 0, 1920, 540), rect(0, 540, 1920, 1080)];
    let mut got = Vec::new();
    for (i, r) in cells.into_iter().enumerate() {
        let c = Cell {
            index: i as i64,
            player: i as i64,
            rect: r,
            camera: new_camera(r, 50.0, 32.0),
        };
        let area = Rect::new(100.0, f64::from(i as i32) * 540.0 + 500.0, 640.0, 20.0);
        let mut cv = NullCanvas::default();
        h.paint.stencils(&mut cv);
        h.draw_radar(&mut cv, &st, &c, area, 1.0, None);
        got.push((cv.rects, cv.sprites));
        st.radar = [
            RadarKind::Castle,
            RadarKind::Enemy,
            RadarKind::Player,
            RadarKind::Portal,
        ]
        .into_iter()
        .enumerate()
        .map(|(n, kind)| RadarMarker {
            kind,
            pos: 0.2 * n as f64,
            own: i == 0,
            ..RadarMarker::default()
        })
        .collect();
    }
    assert_eq!(got[0], (9, 0), "ohne Marker: Leiste, Rahmen (4), Ausschnitt-Rahmen (4)");
    assert_eq!(
        got[1],
        (10, 4),
        "mit Markern: Burg als Fläche, Gegner, Monarch (Ring, Scheibe), Portal als Sprites"
    );
}
