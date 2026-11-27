use super::world_tests::{Fixture, fixture, read_world};

fn hanse() -> Option<Fixture> {
    let f = fixture()?;
    f.editor.create("hanse", "Hanse", "standard").unwrap();
    Some(f)
}

// WE6.3 (B-987, Studio-Anteil): Das Insel-Feld endBoss und das Welt-Feld skins lassen sich über world_set setzen und
// bleiben in der Datei erhalten.
#[test]
fn set_endboss_und_skins_bleiben_erhalten() {
    let Some(f) = hanse() else { return };
    let e = &f.editor;
    let patch = r#"{"skins":{"gegner/wolf":"gegner/ninja"},"islands":[{"id":"island1","endBoss":"forestChieftain"}]}"#;
    e.set("hanse", patch).unwrap();
    let text = read_world(e, "hanse");
    for want in [r#""endBoss": "forestChieftain""#, r#""skins""#, r#""gegner/ninja""#] {
        assert!(text.contains(want), "fehlt {want:?}:\n{text}");
    }
}

// Ein Endboss, den es nicht gibt, lehnt die Prüfung ab und die Datei bleibt, wie sie ist.
#[test]
fn set_endboss_unbekannt_lehnt_ab() {
    let Some(f) = hanse() else { return };
    let e = &f.editor;
    let before = read_world(e, "hanse");
    let patch = r#"{"islands":[{"id":"island1","endBoss":"gibtEsNicht"}]}"#;
    assert!(e.set("hanse", patch).is_err());
    assert_eq!(read_world(e, "hanse"), before);
}
