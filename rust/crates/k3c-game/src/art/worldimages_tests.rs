//! Tests der Weltbilder und der Skalierung (Go: `worldimages_test.go`, `TestApplyScales`) gegen das echte `data/`.

use super::datafile::read_in;
use super::scales::parse_scales;
use super::testutil::real_data;
use super::worldimages::WorldImages;

fn paper() -> Vec<u8> {
    read_in(&real_data(), "paper.json").unwrap()
}

/// B-937 AC-02: data/paper.json › weltbilder ordnet jeder Welt zwei Bilder zu; unbekannte Welten bekommen das der
/// Standard-Welt.
#[test]
fn data_world_images() {
    let w = WorldImages::parse(&paper()).expect("weltbilder lesbar");
    for id in ["standard", "sengoku", "nil", "britannien"] {
        let i = w.0.get(id).unwrap_or_else(|| panic!("{id}: fehlt"));
        assert_eq!(i.big, format!("hintergruende/weltbild-{id}"), "{id}");
        assert_eq!(i.small, format!("hintergruende/weltbild-{id}-klein"), "{id}");
    }
    assert_eq!(w.of("unbekannt"), w.0["standard"]);
    assert_eq!(w.of(""), w.0["standard"]);
    let small = w.assets(false);
    assert!(
        small.len() == w.0.len() && !small.contains(&w.0["nil"].big),
        "kleine Bilder: {small:?}"
    );
    assert_eq!(w.assets(true).len(), 2 * w.0.len());
}

#[test]
fn parse_world_images_rejects() {
    let cases = [
        ("kein standard", r#"{"weltbilder":{"nil":{"gross":"a","klein":"b"}}}"#),
        ("ohne klein", r#"{"weltbilder":{"standard":{"gross":"a"}}}"#),
        ("kaputt", "{"),
        ("fehlt", "{}"),
    ];
    for (name, raw) in cases {
        assert!(WorldImages::parse(raw.as_bytes()).is_err(), "{name}: kein Fehler");
    }
}

/// B-948: Die Faktoren stehen in data/paper.json › skalierung; ohne Abschnitt gilt überall Maßstab 1.
#[test]
fn data_scales_aus_daten() {
    let sc = parse_scales(&paper()).expect("skalierung lesbar");
    assert!(sc.values().all(|&f| f > 0.0), "Faktoren über 0: {sc:?}");
    assert!(parse_scales(b"{}").unwrap().is_empty());
    assert!(parse_scales(b"{").is_err());
}
