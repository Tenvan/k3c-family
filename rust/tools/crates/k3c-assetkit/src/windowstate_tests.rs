use std::cell::RefCell;

use super::*;
use crate::testutil::tempdir;

#[test]
fn load() {
    let dir = tempdir();
    let path = dir.path().join("sub").join("k3c-test.json");
    assert_eq!(super::load(&path), (DEFAULT, false), "fehlende Datei");
    let want = State {
        x: 40,
        y: 60,
        width: 1400,
        height: 900,
    };
    save(&path, want).expect("speichern");
    assert_eq!(super::load(&path), (want, true), "Rundlauf");
    let negative = State {
        x: -1200,
        y: 40,
        width: 1500,
        height: 900,
    };
    save(&path, negative).expect("speichern");
    assert_eq!(super::load(&path), (negative, true), "negative Position");
    fs::write(&path, "{kaputt").expect("schreiben");
    assert_eq!(super::load(&path), (DEFAULT, false), "kaputte Datei");
    let small = State {
        x: -8,
        y: -3000,
        width: 10,
        height: 20,
    };
    save(&path, small).expect("speichern");
    let lifted = State {
        width: MIN_WIDTH,
        height: MIN_HEIGHT,
        ..small
    };
    assert_eq!(super::load(&path).0, lifted, "Minimum");
}

#[test]
fn place_rechnet_versatz_aus() {
    // Position, die das Fenster nach set(0, 0) meldet (Arbeitsbereich-Versatz).
    let calls = RefCell::new(Vec::new());
    place(|| (30, 50), |x, y| calls.borrow_mut().push((x, y)), 200, 300);
    assert_eq!(*calls.borrow(), [(0, 0), (170, 250)]);
}

#[cfg(windows)]
#[test]
fn on_screen_fehlender_bildschirm() {
    let far = State {
        x: -100_000,
        y: 100_000,
        width: DEFAULT_WIDTH,
        height: DEFAULT_HEIGHT,
    };
    assert!(!on_screen(far), "Position weit außerhalb gilt als sichtbar");
}

/// Eigener Test: Dateiform wie Go `json.Marshal`, Teilangaben behalten die Vorgabe, eine Liste ist kaputt.
#[test]
fn dateiform_und_teilangaben() {
    let dir = tempdir();
    let path = dir.path().join("w.json");
    save(&path, DEFAULT).expect("speichern");
    assert_eq!(
        fs::read_to_string(&path).expect("lesen"),
        r#"{"x":0,"y":0,"width":1280,"height":800}"#
    );
    fs::write(&path, r#"{"x":7,"width":null}"#).expect("schreiben");
    assert_eq!(super::load(&path), (State { x: 7, ..DEFAULT }, true));
    fs::write(&path, "[1,2,3,4]").expect("schreiben");
    assert_eq!(super::load(&path), (DEFAULT, false));
    assert!(config_path("w.json").ends_with(Path::new("k3c").join("w.json")));
}
