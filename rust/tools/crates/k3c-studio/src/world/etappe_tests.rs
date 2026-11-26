use super::Editor;
use super::world_tests::{Fixture, fixture, read_world};

/// Fixture mit der angelegten Welt „etappen“.
fn with_world() -> Option<Fixture> {
    let f = fixture()?;
    f.editor.create("etappen", "Etappen", "").unwrap();
    Some(f)
}

fn second_etappe(e: &Editor) -> Option<i64> {
    let d = e.load("etappen");
    assert!(d.error.is_empty(), "{}", d.error);
    d.islands[1].etappe
}

// B-963/AC-02: etappe je Insel schreiben (id, name, etappe, depths), eine Lücke lehnt ab, Load liefert sie.
#[test]
fn etappe_setzen() {
    let Some(f) = with_world() else { return };
    let e = &f.editor;
    let patch = r#"{"islands":[{"id":"island1","etappe":1,"plannedChunks":{"0":[{"slot":-7,"kind":"harbor"}]}},{"id":"island2","etappe":2}]}"#;
    e.set("etappen", patch).unwrap();
    let text = read_world(e, "etappen");
    for want in [
        r#""name": "Insel 1", "etappe": 1, "depths""#,
        r#""name": "island2", "etappe": 2, "depths""#,
    ] {
        assert!(text.contains(want), "fehlt {want:?}:\n{text}");
    }
    assert_eq!(second_etappe(e), Some(2));
    let d = e.load("etappen");
    let forest = &d.biome_info["forest"];
    assert!(forest.portal_hp > 0 && !forest.palette["sky"].is_empty(), "{forest:?}");
    // Etappe ändern und wieder entfernen.
    e.set("etappen", r#"{"islands":[{"id":"island2","etappe":null}]}"#)
        .unwrap();
    assert!(!read_world(e, "etappen").contains(r#""etappe": 2"#));
    // Lücke: Fehler, Datei bleibt.
    let before = read_world(e, "etappen");
    let error = e
        .set("etappen", r#"{"islands":[{"id":"island2","etappe":3}]}"#)
        .unwrap_err();
    assert!(error.contains("etappe"), "{error}");
    assert_eq!(read_world(e, "etappen"), before);
}

#[test]
fn etappe_view() {
    let Some(f) = with_world() else { return };
    let e = &f.editor;
    let v = e.set_etappe_view("etappen", "island1", Some(3));
    assert!(v.error.contains("etappe"), "{v:?}");
    let v = e.set_etappe_view("etappen", "island1", Some(1));
    assert!(v.error.is_empty(), "{v:?}");
}

// Review B-963: unbekannte Insel legt keine Insel an; die Etappe ist eine ganze Zahl.
#[test]
fn etappe_view_nur_vorhandene_insel_und_ganze_zahl() {
    let Some(f) = with_world() else { return };
    let e = &f.editor;
    let before = read_world(e, "etappen");
    let v = e.set_etappe_view("etappen", "gibtsnicht", Some(1));
    assert!(!v.error.is_empty(), "unbekannte Insel angenommen: {v:?}");
    assert_eq!(read_world(e, "etappen"), before);
    let patch = r#"{"islands":[{"id":"island1","etappe":1.5}]}"#;
    assert!(e.set("etappen", patch).is_err(), "nicht ganze Zahl angenommen");
    assert_eq!(read_world(e, "etappen"), before);
}
