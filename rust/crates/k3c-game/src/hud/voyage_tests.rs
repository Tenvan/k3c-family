//! Port von `native/k3c-game/hud/voyage_test.go`: Schiffsfahrt beim Inselwechsel (B-977): Lage des Schiffs als reine
//! Funktion der Zeit, Spur des Schiffs, Karte der Fahrt, Fahrt nicht zeichenbar. Nicht portiert:
//! `TestVoyagePosAllocatesNothing` und der Allokations-Block von `TestVoyageViewBuildsMap` (`testing.AllocsPerRun` ist in
//! Rust nicht portierbar, das Frame-Budget misst RU39); der Rest von `TestVoyagePosAllocatesNothing` (die vordere Welle
//! bewegt sich) steht in `voyage_shift_moves`. Dazu: Die Fahrt verbraucht keine Eingabe, die vordere Welle wird auf die
//! Tafel gestutzt, und mit geladener Grafik zeichnet sie in der Reihenfolge Papier, Tafel, Welle.

use k3c_platform::{NullCanvas, Rect as PRect};

use super::drawsea_paper::trim_tile;
use super::layout::Rect;
use super::seekarte::SeaKind;
use super::seekarte_paper_tests::sea_lib;
use super::testutil::{HudSession, hud, input, two_players};
use super::texts::tr;
use super::voyage::*;
use crate::view::frame::{Frame, RouteIsland};
use crate::view::input::Action::{Back, Menu};
use crate::view::layer::Drawer;
use crate::view::roles::Overlay;
use crate::view::voyage::{Voyage, VoyageDrawer};

/// Karte der Aufnahme: jede Welt-Miniatur, alle Zustände, drei Reihen in der mittleren Etappe (Go: `shotRoute`).
pub(super) fn shot_route() -> Vec<RouteIsland> {
    let isl = |id: &str, name: &str, etappe, world: &str, stages, chunks| RouteIsland {
        id: id.to_string(),
        name: name.to_string(),
        etappe,
        world: world.to_string(),
        stages,
        chunks,
        ..RouteIsland::default()
    };
    vec![
        isl("gruenhain", "Grünhain", 1, "standard", 2, 24),
        isl("kyoto", "Kyoto", 2, "sengoku", 3, 40),
        isl("theben", "Theben", 2, "nil", 5, 100),
        isl("avalon", "Avalon", 2, "britannien", 1, 16),
        isl("nebelkap", "Nebelkap", 3, "standard", 4, 64),
        isl("edo", "Edo", 3, "sengoku", 2, 0),
    ]
}

/// voyage_of ist die Fahrt von Grünhain nach Theben über die Karte der Aufnahme.
pub(super) fn voyage_of() -> Voyage {
    Voyage {
        from: "gruenhain".to_string(),
        to: "theben".to_string(),
        done: vec!["gruenhain".to_string()],
        open: ["kyoto", "theben", "avalon"].map(String::from).to_vec(),
        duration: 3.0,
        ..Voyage::default()
    }
}

/// B-977/AC-04: Die Lage des Schiffs ist eine reine Funktion der Zeit: gleiche Zeit, gleiche Lage; Start am Liegeplatz
/// der verlassenen Insel, nach der Fahrzeit am Ziel (dort schaukelt es weiter), dazwischen über der geraden Strecke und
/// nie rückwärts.
#[test]
fn voyage_pos_is_pure_function_of_time() {
    const AX: f64 = 100.0;
    const AY: f64 = 400.0;
    const BX: f64 = 900.0;
    const BY: f64 = 490.0;
    const ARC: f64 = 28.0;
    const BOB: f64 = 2.5;
    const DUR: f64 = 3.0;
    let pos = |t: f64| voyage_pos(AX, AY, BX, BY, ARC, BOB, 0.7, t, DUR);
    for at in [0.0, 0.4, 1.5, 2.9, 3.0, 7.25] {
        assert_eq!(pos(at), pos(at), "t={at}");
    }
    assert_eq!(pos(0.0), (AX, AY), "Start");
    for at in [DUR, DUR + 1.3, DUR + 60.0] {
        // wartet am Ziel: x fest, y schaukelt um den Liegeplatz
        let (x, y) = pos(at);
        assert!(x == BX && (y - BY).abs() <= BOB + 1e-9, "t={at} am Ziel: ({x}, {y})");
    }
    assert_ne!(
        pos(DUR + 0.2).1,
        pos(DUR + 0.5).1,
        "am Ziel steht das Schiff still (kein Schaukeln)"
    );
    assert!(
        pos(DUR / 2.0).1 <= (AY + BY) / 2.0 - ARC + BOB,
        "Mitte der Strecke: nicht auf dem Bogen"
    );
    let mut prev = AX;
    for i in 1..=90 {
        let (x, _) = pos(f64::from(i) / 30.0);
        assert!(x >= prev, "Takt {i}: x {x} nach {prev} (rückwärts)");
        prev = x;
    }
}

/// B-977/AC-04: Die vordere Welle bewegt sich mit der Zeit (Rest von `TestVoyagePosAllocatesNothing`).
#[test]
fn voyage_shift_moves() {
    assert_ne!(
        voyage_shift(14.0, 0.22, 0.5),
        voyage_shift(14.0, 0.22, 1.5),
        "vordere Welle bewegt sich nicht"
    );
    assert_eq!(voyage_ease(1.25, 3.0), voyage_ease(1.25, 3.0));
    assert_eq!(
        (voyage_ease(-1.0, 3.0), voyage_ease(3.0, 3.0), voyage_ease(1.0, 0.0)),
        (0.0, 1.0, 1.0)
    );
}

/// GS2.4: Das Schiff bleibt die ganze Fahrt in einer Zeichen-Reihe, der vorderen von Start und Ziel: Es springt
/// unterwegs nicht vor oder hinter eine Insel, gleich in welche Richtung die Fahrt geht.
#[test]
fn voyage_lane_stays_in_front() {
    for (from, to, want) in [(0, 0, 0), (0, 1, 1), (1, 0, 1), (2, 0, 2), (1, 2, 2)] {
        assert_eq!(voyage_lane(from, to), want, "Reihe von {from} nach {to}");
    }
}

/// `var _ view.VoyageDrawer = (*hud)(nil)`: Der Drawer des HUD ist ein `VoyageDrawer`.
#[test]
fn hud_drawer_is_voyage_drawer() {
    let d = super::new_pending(None);
    let _: &dyn VoyageDrawer = &d;
    let _: &dyn Drawer = &d;
}

/// Die Karte der Fahrt: Start und Ziel an ihrem Platz, Start abgeschlossen und ohne Schiff am Liegeplatz, Ziel gewählt,
/// die übrigen Ziele bleiben Ziele, kein Cursor. Ein zweiter Aufruf mit derselben Fahrt baut nicht neu.
#[test]
fn voyage_view_builds_map() {
    let (mut y, route, v) = (VoyageView::default(), shot_route(), voyage_of());
    assert!(y.build("de", &route, &v), "Karte der Fahrt nicht gebaut");
    assert_eq!(y.from, SeaSpot { col: 0, lane: 0, n: 1 }, "Start");
    assert_eq!(y.to, SeaSpot { col: 1, lane: 1, n: 3 }, "Ziel");
    check_voyage_nodes(&y);
    assert!(
        y.sea.hint == tr("de", "sea.voyage")
            && !y.sea.title.is_empty()
            && y.sea.line.n == 0
            && y.sea.line.plain.is_empty(),
        "Texte: {:?}, {:?}",
        y.sea.title,
        y.sea.hint
    );
    y.sea.title.push('!'); // eine Neubau würde den Titel zurücksetzen
    assert!(
        y.build("de", &route, &v) && y.sea.title.ends_with('!'),
        "unveränderte Fahrt baut neu"
    );
    assert!(
        y.build("en", &route, &v) && y.sea.hint == tr("en", "sea.voyage"),
        "Sprachwechsel baut nicht neu: {:?}",
        y.sea.hint
    );
}

/// B-977/R5: Ohne Route, mit unbekannter Insel, ohne abgeschlossenen Start oder ohne Grafik ist die Fahrt nicht zu
/// zeichnen (game zeigt den Ladebildschirm).
#[test]
fn voyage_not_ready() {
    type Edit = fn(&mut Voyage);
    let cases: [(&str, bool, Edit); 6] = [
        ("ohne Route", false, |_| {}),
        ("Ziel unbekannt", true, |v| v.to = "atlantis".to_string()),
        ("Start unbekannt", true, |v| {
            v.from = "atlantis".to_string();
            v.done = vec!["atlantis".to_string()];
        }),
        ("Start nicht abgelegt", true, |v| v.done.clear()),
        ("Start gleich Ziel", true, |v| v.to = v.from.clone()),
        ("ohne Ziel", true, |v| v.to.clear()),
    ];
    for (name, with_route, edit) in cases {
        let (mut y, mut v) = (VoyageView::default(), voyage_of());
        edit(&mut v);
        let route = if with_route { shot_route() } else { Vec::new() };
        assert!(!y.build("de", &route, &v), "{name}: Karte gebaut");
    }
    let d = super::new_pending(None);
    let f = Frame {
        route: shot_route(),
        ..Frame::default()
    };
    let mut c = NullCanvas::default();
    assert!(
        !d.voyage_ready(&f, &voyage_of()) && !d.draw_voyage(&mut c, &f, &voyage_of()),
        "ohne Grafik: Fahrt bereit oder gezeichnet"
    );
    assert_eq!(
        (c.rects, c.sprites, c.texts),
        (0, 0, 0),
        "nichts gezeichnet, wenn nicht bereit"
    );
}

/// checkVoyageNodes prüft die Inseln der Karte der Fahrt von Grünhain nach Theben.
fn check_voyage_nodes(y: &VoyageView) {
    let (from, to) = (y.node(y.from), y.node(y.to));
    assert!(
        from.kind == SeaKind::Done && !from.current && to.chosen && to.kind == SeaKind::Target,
        "Start {from:?}, Ziel {to:?}"
    );
    for n in y.sea.rows.iter().flat_map(|r| &r.nodes) {
        assert!(
            !n.cursor && !n.current,
            "{}: Cursor {}, Schiff {}",
            n.id,
            n.cursor,
            n.current
        );
        let want = ["kyoto", "theben", "avalon"].contains(&n.id.as_str());
        assert_eq!(n.kind == SeaKind::Target, want, "{}: Zustand {:?}", n.id, n.kind);
    }
}

/// B und Esc: In der Fahrt (Vollbild) gibt es keine Eingabe. `Back` und `Menu` gehen an game (Pause) und werden vom HUD
/// nicht verbraucht, auch wenn die Fahrt gerade bereit ist.
#[test]
fn voyage_consumes_no_input() {
    let (_, f) = two_players(2);
    let mut h = hud();
    let mut s = HudSession::default();
    let route = shot_route();
    assert!(h.voy.build("de", &route, &voyage_of()), "Fahrt bereit");
    for acts in [&[Back][..], &[Menu][..], &[Back, Menu][..]] {
        let st = h.update(&input(acts, acts), &f, &mut s);
        assert!(!st.consumed[0] && !st.consumed[1], "{acts:?}: Eingabe verbraucht");
    }
    assert!(s.sent.is_empty() && s.picks.is_empty(), "nichts an die Session");
}

/// Die bewegte vordere Welle wird auf die Tafel gestutzt (Ersatz für den `SubImage`-Beschnitt von Go): Ziel und Quelle
/// schrumpfen im gleichen Verhältnis, außerhalb bleibt nichts.
#[test]
fn wave_tile_trimmed_to_board() {
    let src = PRect::new(10.0, 0.0, 100.0, 50.0);
    let clip = Rect::new(0.0, 0.0, 200.0, 100.0);
    let (r, s) = trim_tile(Rect::new(-50.0, 10.0, 100.0, 50.0), src, clip).expect("Teil innen");
    assert_eq!((r.x, r.w, r.y, r.h), (0.0, 50.0, 10.0, 50.0), "Ziel gestutzt");
    assert_eq!((s.x, s.w, s.y, s.h), (60.0, 50.0, 0.0, 50.0), "Quelle gleich gestutzt");
    let (r, s) = trim_tile(Rect::new(150.0, 0.0, 100.0, 50.0), src, clip).expect("Teil innen");
    assert_eq!((r.x, r.w, s.x, s.w), (150.0, 50.0, 10.0, 50.0), "rechts gestutzt");
    assert!(
        trim_tile(Rect::new(-120.0, 0.0, 100.0, 50.0), src, clip).is_none(),
        "ganz außerhalb"
    );
    let (r, s) = trim_tile(Rect::new(20.0, 20.0, 100.0, 50.0), src, clip).expect("ganz innen");
    assert_eq!(
        (r, s),
        (Rect::new(20.0, 20.0, 100.0, 50.0), src),
        "ganz innen unverändert"
    );
}

/// Mit geladener Grafik ist die Fahrt bereit und zeichnet Papier, Tafel, Wellen, Schiff und Texte, zu jeder Zeit der
/// Fahrt. Ohne gerenderte Grafik läuft der Test nicht (wie `t.Skip`); das Bild vergleicht RU23.6.
#[test]
fn voyage_draws_with_assets() {
    let Some(mut h) = sea_lib() else {
        eprintln!("übersprungen: Grafik fehlt");
        return;
    };
    let f = Frame {
        route: shot_route(),
        ..Frame::default()
    };
    let v = voyage_of();
    assert!(h.voyage_ready(&f, &v), "Fahrt mit Grafik nicht bereit");
    for t in [0.0, 1.5, 3.0, 9.0] {
        let mut c = NullCanvas::default();
        let at = Voyage {
            elapsed: t,
            ..v.clone()
        };
        assert!(h.draw_voyage(&mut c, &f, &at), "t={t}: nicht gezeichnet");
        assert!(c.rects > 0 && c.sprites > 0 && c.texts > 0, "t={t}: {c:?}");
    }
}
