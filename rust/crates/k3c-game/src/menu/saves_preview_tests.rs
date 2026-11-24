//! GS3.8 (B-966): Vorschau des markierten Spielstands: Inhalt (Kacheln, Trophäen, Schiff, Zeilen), keine Vorschau bei
//! nicht ladbarem Stand oder ohne Route, Wechsel mit Auf/Ab und das Zeichnen.

use k3c_platform::NullCanvas;
use k3c_sim::island::chronicle::ChronicleEntry;

use super::draw::Painter;
use super::layout::geom_of;
use super::saves::new_saves;
use super::saves_preview::{PreviewNode, preview_of};
use super::testutil::{FakeSession, KEYS1, new_menus, press, save};
use crate::view::cell::rect;
use crate::view::frame::RouteIsland;
use crate::view::input::Action::Down;
use crate::view::screen::Screen;
use crate::view::session::SaveInfo;
use crate::view::settings::{DEFAULT_LANGUAGE, LANGUAGE_ENGLISH};

fn isl(id: &str, etappe: i64, done: &str) -> RouteIsland {
    RouteIsland {
        id: id.to_string(),
        name: id.to_uppercase(),
        etappe,
        done: done.to_string(),
        ..RouteIsland::default()
    }
}

/// Stand mit Route a (1), b und c (2): a abgeschlossen, das Schiff liegt bei b.
fn with_route(name: &str) -> SaveInfo {
    SaveInfo {
        route: vec![isl("a", 1, "Der Hafen ist frei."), isl("b", 2, ""), isl("c", 2, "")],
        island: "b".to_string(),
        completed: vec!["a".to_string()],
        chronicle: vec![ChronicleEntry {
            island: "a".to_string(),
            days: 5,
            monarchs: 3,
        }],
        ..save(name, 4, true)
    }
}

#[test]
fn inhalt_kacheln_trophaeen_schiff_und_zeilen() {
    let p = preview_of(DEFAULT_LANGUAGE, &with_route("s"));
    let n = |done, ship| PreviewNode { done, ship };
    assert!(p.on);
    assert_eq!(
        p.cols,
        vec![vec![n(true, false)], vec![n(false, true), n(false, false)]]
    );
    assert_eq!(
        (p.line.as_str(), p.numbers.as_str()),
        ("Der Hafen ist frei.", "5 Tage · 3 Monarchen")
    );
    let mut e = with_route("s");
    e.chronicle[0] = ChronicleEntry {
        island: "a".to_string(),
        days: 1,
        monarchs: 1,
    };
    assert_eq!(preview_of(LANGUAGE_ENGLISH, &e).numbers, "1 day · 1 monarch");
}

#[test]
fn ohne_abschluss_keine_trophaeen_ohne_eintrag_nur_zeile() {
    let mut e = with_route("s");
    e.completed.clear();
    e.chronicle.clear();
    let p = preview_of(DEFAULT_LANGUAGE, &e);
    assert!(p.on && p.cols.iter().flatten().all(|n| !n.done), "keine Trophäen");
    assert!(p.line.is_empty() && p.numbers.is_empty());
    let mut e = with_route("s");
    e.chronicle.clear();
    let p = preview_of(DEFAULT_LANGUAGE, &e);
    assert_eq!((p.line.as_str(), p.numbers.as_str()), ("Der Hafen ist frei.", ""));
}

#[test]
fn nicht_ladbar_oder_ohne_route_keine_vorschau() {
    let mut e = with_route("s");
    e.loadable = false;
    assert!(!preview_of(DEFAULT_LANGUAGE, &e).on);
    let mut e = with_route("s");
    e.route.clear();
    assert!(!preview_of(DEFAULT_LANGUAGE, &e).on);
}

/// Die Vorschau folgt der Markierung: Auf/Ab wechselt sie sofort, ein nicht ladbarer Stand hat keine.
#[test]
fn vorschau_folgt_der_markierung() {
    let mut s = FakeSession {
        saves: vec![with_route("neu"), save("kaputt", 0, false), save("ohne", 1, true)],
        ..Default::default()
    };
    let mut sc = new_saves(&new_menus());
    sc.update(&press(KEYS1, &[]), &mut s);
    sc.fill(DEFAULT_LANGUAGE);
    assert!(sc.pg.borrow().preview.on, "erster Stand mit Route");
    sc.update(&press(KEYS1, &[Down]), &mut s);
    sc.fill(DEFAULT_LANGUAGE);
    assert!(!sc.pg.borrow().preview.on, "nicht ladbar: keine Vorschau");
    sc.update(&press(KEYS1, &[Down]), &mut s);
    sc.fill(DEFAULT_LANGUAGE);
    assert!(!sc.pg.borrow().preview.on, "ohne Route: keine Vorschau");
}

/// Gezeichnet werden Tafel und Kacheln mit Trophäen; ohne Vorschau und in zu schmalem Rand nichts.
#[test]
fn zeichnen_nur_mit_vorschau_und_platz() {
    let mut pt = Painter::new(None, None);
    let g = geom_of(rect(0, 0, 1920, 1080));
    let p = preview_of(DEFAULT_LANGUAGE, &with_route("s"));
    let mut c = NullCanvas::default();
    pt.preview(&mut c, &g, &p);
    assert!(c.rects > 10, "Tafel, Kacheln, Trophäe, Schiff: {c:?}");
    let mut c = NullCanvas::default();
    pt.preview(&mut c, &g, &preview_of(DEFAULT_LANGUAGE, &SaveInfo::default()));
    assert_eq!(c.rects, 0);
    let mut c = NullCanvas::default();
    pt.preview(&mut c, &geom_of(rect(0, 0, 800, 1080)), &p);
    assert_eq!(
        c.rects, 0,
        "mit 5 % Rand rechts passt die Tafel nicht mehr (ohne Rand wären es 132 px)"
    );
}

fn font_painter() -> Painter {
    Painter::new(
        crate::art::load_font(std::path::Path::new("gibt-es-nicht.ttf"), false).ok(),
        None,
    )
}

/// Mindestschrift: Die Zeile schrumpft nie, sie bricht in höchstens zwei Zeilen um und kürzt mit „...“.
#[test]
fn zeile_bricht_um_und_kuerzt_ohne_zu_schrumpfen() {
    let pt = font_painter();
    let size = 32.0;
    let cw = pt.measure("a", size).0;
    let fits = |l: &String, w: f64| pt.measure(l, size).0 <= w;
    let w = 13.0 * cw;
    assert_eq!(pt.wrap("kurz", w, size, 2), ["kurz"]);
    let two = pt.wrap("Der Hafen ist frei", w, size, 2);
    assert_eq!(two, ["Der Hafen ist", "frei"]);
    let cut = pt.wrap("Ein sehr langer Satz ohne Ende der nicht passt", w, size, 2);
    assert_eq!(cut.len(), 2, "{cut:?}");
    assert!(cut.iter().all(|l| fits(l, w)), "{cut:?}");
    assert!(cut[1].ends_with("..."), "{cut:?}");
    let one = pt.wrap("Zwei Monarchen und viele Tage", w, size, 1);
    assert!(one.len() == 1 && fits(&one[0], w) && one[0].ends_with("..."), "{one:?}");
    assert!(pt.wrap("", w, size, 2).is_empty());
}
