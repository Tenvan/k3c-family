use super::super::world_tests::{Fixture, fixture, read_world};
use super::super::{Editor, Info, kind_of};

// B-1217: Art der Welt ohne Angabe aus extends.
#[test]
fn art_ohne_angabe() {
    for (kind, extends, want) in [
        ("", "", "new"),
        ("", "standard", "spinoff"),
        ("new", "standard", "new"),
        ("spinoff", "standard", "spinoff"),
    ] {
        assert_eq!(kind_of(kind, extends), want, "kind_of({kind:?}, {extends:?})");
    }
}

/// Fixture mit der angelegten Welt.
fn with(id: &str, name: &str, extends: &str) -> Option<Fixture> {
    let f = fixture()?;
    f.editor.create(id, name, extends).unwrap();
    Some(f)
}

fn listed(e: &Editor, id: &str) -> Info {
    e.list().into_iter().find(|i| i.id == id).unwrap()
}

// B-1217: Art schreiben (Feld kind), ungültige Werte lehnt die Engine ab, Load und List nennen sie.
#[test]
fn art_setzen() {
    let Some(f) = with("ableger", "Ableger", "standard") else {
        return;
    };
    let e = &f.editor;
    assert_eq!(e.load("ableger").kind, "spinoff");
    let v = e.set_kind_view("ableger", "new");
    assert!(v.error.is_empty(), "new: {v:?}");
    assert_eq!(e.load("ableger").kind, "new");
    assert!(read_world(e, "ableger").contains(r#""kind": "new""#));
    assert_eq!(listed(e, "ableger").kind, "new");
    let before = read_world(e, "ableger");
    let v = e.set_kind_view("ableger", "episode");
    assert!(v.error.contains("kind"), "unbekannte Art: {v:?}");
    assert_eq!(read_world(e, "ableger"), before);
    let v = e.set_kind_view("ableger", "");
    assert!(v.error.is_empty(), "Feld entfernen: {v:?}");
    assert_eq!(e.load("ableger").kind, "spinoff");
}

// Ableger ohne extends: die Standard-Welt wird als Eltern-Welt eingetragen.
#[test]
fn ableger_bekommt_extends() {
    let Some(f) = with("frei", "Frei", "") else { return };
    let e = &f.editor;
    let v = e.set_kind_view("frei", "spinoff");
    assert!(v.error.is_empty(), "{v:?}");
    let d = e.load("frei");
    assert_eq!((d.kind.as_str(), d.extends.as_str()), ("spinoff", "standard"));
}

// Eltern-Welt wählen: eine andere Welt wird eingetragen, die eigene lehnt die Engine ab.
#[test]
fn eltern_welt_waehlen() {
    let Some(f) = with("kind2", "Kind 2", "standard") else {
        return;
    };
    let e = &f.editor;
    e.create("eltern", "Eltern", "").unwrap();
    let v = e.set_extends_view("kind2", "eltern");
    assert!(v.error.is_empty(), "{v:?}");
    assert_eq!(e.load("kind2").extends, "eltern");
    let v = e.set_extends_view("kind2", "kind2");
    assert!(v.error.contains("extends"), "selbst erwartet Fehler: {v:?}");
}

// Gewässer der Welt: Load und List nennen es, world_set schreibt es.
#[test]
fn gewaesser() {
    let Some(f) = with("fluss", "Fluss", "") else { return };
    let e = &f.editor;
    e.set("fluss", r#"{"gewaesser": "Nil"}"#).unwrap();
    assert_eq!(e.load("fluss").gewaesser, "Nil");
    assert_eq!(listed(e, "fluss").gewaesser, "Nil");
}
