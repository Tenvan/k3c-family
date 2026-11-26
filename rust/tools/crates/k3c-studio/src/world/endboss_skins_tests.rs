use std::collections::BTreeMap;

use super::super::world_tests::{Fixture, fixture, read_world};

fn hanse() -> Option<Fixture> {
    let f = fixture()?;
    f.editor.create("hanse", "Hanse", "standard").unwrap();
    Some(f)
}

fn skins(from: &str, to: &str) -> BTreeMap<String, String> {
    [(from.to_owned(), to.to_owned())].into()
}

// WE6.4 (B-981): Der Endboss je Insel lässt sich setzen und entfernen; ein unbekannter Endboss lehnt ab.
#[test]
fn endboss_setzen_und_entfernen() {
    let Some(f) = hanse() else { return };
    let e = &f.editor;
    let got = e.set_island_end_boss("hanse", "island1", "forestChieftain");
    assert!(got.error.is_empty(), "setzen: {}", got.error);
    assert_eq!(e.load("hanse").islands[0].end_boss, "forestChieftain");
    assert!(
        !e.set_island_end_boss("hanse", "island1", "gibtEsNicht")
            .error
            .is_empty()
    );
    let got = e.set_island_end_boss("hanse", "island1", "");
    assert!(got.error.is_empty(), "entfernen: {}", got.error);
    let d = e.load("hanse");
    assert_eq!(d.islands[0].end_boss, "");
    let bosses = d.end_bosses.unwrap_or_default();
    assert!(
        bosses.first().is_some_and(|b| !b.name.is_empty()),
        "Endboss-Liste fehlt: {bosses:?}"
    );
}

// WE6.4 (B-984): Skins setzen, lesen und leeren; ein ungültiges Format lehnt ab und die Datei bleibt unverändert.
#[test]
fn skins_setzen_lesen_und_ablehnen() {
    let Some(f) = hanse() else { return };
    let e = &f.editor;
    let got = e.set_skins("hanse", &skins("gegner/wolf", "gegner/ninja"));
    assert!(got.error.is_empty(), "setzen: {}", got.error);
    let read = e.load("hanse").skins.unwrap_or_default();
    assert_eq!(read.get("gegner/wolf").map(String::as_str), Some("gegner/ninja"));
    let before = read_world(e, "hanse");
    let got = e.set_skins("hanse", &skins("wolf", "gegner/ninja"));
    assert!(got.error.contains("wolf"), "{:?}", got.error);
    assert_eq!(read_world(e, "hanse"), before);
    let got = e.set_skins("hanse", &BTreeMap::new());
    assert!(got.error.is_empty(), "leeren: {}", got.error);
    assert!(e.load("hanse").skins.unwrap_or_default().is_empty());
}

// Eine unbekannte Insel oder ein unbekannter Endboss lehnt ab; die Datei bleibt, es entsteht keine neue Insel.
#[test]
fn endboss_lehnt_unbekannte_insel_und_boss_ab() {
    let Some(f) = hanse() else { return };
    let e = &f.editor;
    let before = read_world(e, "hanse");
    let got = e.set_island_end_boss("hanse", "gibtEsNicht", "forestChieftain");
    assert_eq!(got.error, r#"die Insel "gibtEsNicht" gibt es in hanse nicht"#);
    let got = e.set_island_end_boss("hanse", "island1", "gibtEsNicht");
    assert_eq!(got.error, r#"unbekannter Endboss "gibtEsNicht""#);
    // Wie Go: Antwort ohne Warnungen (`null`).
    assert_eq!(got.warnings, None);
    assert_eq!(read_world(e, "hanse"), before);
}
