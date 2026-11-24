//! GS3.8 (B-966): Andenken der abgeschlossenen Inseln auf der Seekarte: Zeiger auf abgeschlossenen Inseln, `done`-Zeile
//! und Zahlen aus der Chronik, A wirkt dort nicht, kein Neubau im Ruhezustand.

use super::seekarte::{SeaKind, SeaView};
use super::testutil::{open_switch, sea_route};
use crate::view::frame::RouteIsland;
use crate::view::input::Action::{Confirm, Left, Right};
use crate::view::trophy::{Part, parts};
use k3c_sim::island::chronicle::ChronicleEntry;

fn route_with_done() -> Vec<RouteIsland> {
    let mut r = sea_route();
    r[0].done = "Der Hafen ist frei.".to_string();
    r
}

fn entry(id: &str, days: i64, monarchs: i64) -> ChronicleEntry {
    ChronicleEntry {
        island: id.to_string(),
        days,
        monarchs,
    }
}

fn node<'a>(v: &'a SeaView, id: &str) -> &'a super::seekarte::SeaNode {
    v.rows.iter().flat_map(|r| &r.nodes).find(|n| n.id == id).unwrap()
}

/// Eingabeschritt, danach wie im Spiel das nächste `refresh_with` (setzt die Marker).
fn go(v: &mut SeaView, acts: &[crate::view::input::Action], route: &[RouteIsland], chr: &[ChronicleEntry]) {
    v.step(acts);
    v.refresh_with("de", "", Some(&open_switch("")), route, chr);
}

/// Ring: zuerst die Ziele (b, c), dann die abgeschlossene Insel a; Rechts, Rechts erreicht a.
fn point_at_a(v: &mut SeaView, route: &[RouteIsland], chr: &[ChronicleEntry]) {
    go(v, &[Right, Right], route, chr);
    assert!(node(v, "a").pointed, "Zeiger auf a");
}

#[test]
fn zeiger_erreicht_abgeschlossene_insel_mit_zeile_und_zahlen() {
    let (route, chr) = (route_with_done(), [entry("a", 12, 2)]);
    let mut v = SeaView::default();
    v.refresh_with("de", "", Some(&open_switch("")), &route, &chr);
    assert!(!node(&v, "a").pointed, "Zeiger beginnt auf einem Ziel");
    assert!(node(&v, "b").cursor);
    point_at_a(&mut v, &route, &chr);
    let a = node(&v, "a");
    assert_eq!(a.kind, SeaKind::Done);
    assert_eq!(
        (a.done_line.as_str(), a.numbers.as_str()),
        ("Der Hafen ist frei.", "12 Tage · 2 Monarchen")
    );
    assert!(!node(&v, "b").cursor && !node(&v, "b").pointed);
    assert!(node(&v, "b").done_line.is_empty(), "Ziele tragen kein Andenken");
    go(&mut v, &[Left], &route, &chr);
    assert!(
        !node(&v, "a").pointed && node(&v, "c").cursor,
        "Links zurück auf ein Ziel"
    );
}

#[test]
fn einzahl_und_englisch() {
    let (route, chr) = (route_with_done(), [entry("a", 1, 1)]);
    let mut v = SeaView::default();
    v.refresh_with("de", "", Some(&open_switch("")), &route, &chr);
    assert_eq!(node(&v, "a").numbers, "1 Tag · 1 Monarch");
    v.refresh_with("en", "", Some(&open_switch("")), &route, &chr);
    assert_eq!(node(&v, "a").numbers, "1 day · 1 monarch");
}

#[test]
fn ohne_done_nur_zahlen_ohne_eintrag_nur_zeile() {
    let mut v = SeaView::default();
    v.refresh_with("de", "", Some(&open_switch("")), &sea_route(), &[entry("a", 5, 3)]);
    let a = node(&v, "a");
    assert_eq!((a.done_line.as_str(), a.numbers.as_str()), ("", "5 Tage · 3 Monarchen"));
    v.refresh_with("de", "", Some(&open_switch("")), &route_with_done(), &[]);
    let a = node(&v, "a");
    assert_eq!(
        (a.kind, a.numbers.as_str()),
        (SeaKind::Done, ""),
        "Trophäe auch ohne Eintrag"
    );
    assert_eq!(a.done_line, "Der Hafen ist frei.");
}

#[test]
fn a_wirkt_auf_abgeschlossener_insel_nicht() {
    let route = route_with_done();
    let mut v = SeaView::default();
    v.refresh_with("de", "", Some(&open_switch("")), &route, &[]);
    assert_eq!(v.step(&[Confirm]).pick, "b", "auf einem Ziel wählt A");
    point_at_a(&mut v, &route, &[]);
    let r = v.step(&[Confirm]);
    assert!(r.pick.is_empty() && !r.moved, "auf einer abgeschlossenen Insel nichts");
    assert!(r.denied, "Ablehnen-Feedback wie bei einer abgelehnten Wahl");
    assert!(!v.step(&[Left]).denied);
}

#[test]
fn ruhezustand_baut_nicht_neu_und_chronik_aendert_den_text() {
    let (route, chr) = (route_with_done(), [entry("a", 12, 2)]);
    let sw = open_switch("");
    let mut v = SeaView::default();
    v.refresh_with("de", "", Some(&sw), &route, &chr);
    let p = node(&v, "a").numbers.as_ptr();
    for _ in 0..3 {
        v.refresh_with("de", "", Some(&sw), &route, &chr);
    }
    assert_eq!(
        node(&v, "a").numbers.as_ptr(),
        p,
        "gleicher Stand: kein Neubau, keine Allokation"
    );
    v.refresh_with("de", "", Some(&sw), &route, &[entry("a", 13, 2)]);
    assert_eq!(node(&v, "a").numbers, "13 Tage · 2 Monarchen");
}

#[test]
fn zeiger_bleibt_nach_neubau_gueltig() {
    let route = route_with_done();
    let sw = open_switch("");
    let mut v = SeaView::default();
    v.refresh_with("de", "", Some(&sw), &route, &[]);
    point_at_a(&mut v, &route, &[]);
    v.refresh_with("de", "", Some(&sw), &route, &[entry("a", 2, 1)]);
    assert!(
        node(&v, "a").pointed,
        "der Neubau durch die Chronik nimmt den Zeiger nicht weg"
    );
}

#[test]
fn trophaee_liegt_ueber_der_standlinie() {
    let mut n = 0;
    parts(100.0, 200.0, 40.0, true, &mut |p| {
        n += 1;
        match p {
            Part::Rect(_, y, _, h, _) => assert!(y + h <= 200.0 + 1e-9 && y >= 160.0 - 1e-9, "{p:?}"),
            Part::Disc(_, y, r, _) | Part::Ring(_, y, r, _) => assert!(y + r <= 200.0 && y - r >= 160.0, "{p:?}"),
        }
    });
    assert_eq!(n, 6 + 1 + 3, "Trophäe samt Fahne");
    let mut m = 0;
    parts(0.0, 0.0, 1.0, false, &mut |_| m += 1);
    assert_eq!(m, 6, "ohne Fahne");
}
