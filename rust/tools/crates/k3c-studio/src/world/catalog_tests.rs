use std::path::Path;

use super::super::Editor;
use super::super::world_tests::{Fixture, checkout, failing, fixture};
use super::{PICKS, worlds_of};

fn read(e: &Editor, name: &str) -> String {
    std::fs::read_to_string(e.root.join("data").join(name)).unwrap()
}

/// Fixture mit der angelegten Welt „hanse“.
fn with_hanse() -> Option<Fixture> {
    let f = fixture()?;
    f.editor.create("hanse", "Hanse", "standard").unwrap();
    Some(f)
}

// SI7.1 (B-1003): Wetter, Reittier und Einsiedler je Welt über world_set; die Zuordnung steht in den Katalog-Dateien,
// die Reihenfolge und der Rest der Datei bleiben, „null“ entfernt den Eintrag.
#[test]
fn picks_setzen_und_entfernen() {
    let Some(f) = with_hanse() else { return };
    let e = &f.editor;
    e.set("hanse", r#"{"weather":"taifun","mount":"camel","hermit":"priestess"}"#)
        .unwrap();
    let got = e.picks("hanse").unwrap();
    assert_eq!(
        (got["weather"].as_str(), got["mount"].as_str(), got["hermit"].as_str()),
        ("taifun", "camel", "priestess")
    );
    let weather = read(e, "weather.json");
    assert!(
        weather.contains(r#""sengoku": "taifun""#) && weather.contains(r#""hanse": "taifun""#),
        "weather.json: {weather}"
    );
    e.set("hanse", r#"{"mount":null}"#).unwrap();
    let mounts = read(e, "mounts.json");
    assert!(
        !mounts.contains(r#""hanse""#) && mounts.contains(r#""sengoku": "sengokuHorse""#),
        "mounts.json nach keins: {mounts}"
    );
}

#[test]
fn pick_unbekannte_id_lehnt_ab() {
    let Some(f) = with_hanse() else { return };
    let e = &f.editor;
    let before = read(e, "hermits.json");
    let error = e.set("hanse", r#"{"hermit":"gibtEsNicht"}"#).unwrap_err();
    assert!(error.contains("gibtEsNicht"), "{error}");
    assert_eq!(read(e, "hermits.json"), before);
}

#[test]
fn picks_view_liefert_optionen_und_lehnt_ab() {
    let Some(f) = with_hanse() else { return };
    let e = &f.editor;
    let view = e.picks_view("sengoku");
    assert!(view.error.is_empty(), "{}", view.error);
    assert_eq!(view.values["weather"], "taifun");
    assert!(view.options["mount"].iter().any(|id| id == "camel"), "{view:?}");
    assert!(!e.set_pick_view("hanse", "mount", "gibtEsNicht").error.is_empty());
    let got = e.set_pick_view("hanse", "mount", "camel");
    assert!(got.error.is_empty(), "{}", got.error);
}

#[test]
fn pick_patch_mit_ungueltiger_art_schreibt_nichts() {
    let Some(f) = with_hanse() else { return };
    let e = &f.editor;
    let before = (read(e, "weather.json"), read(e, "mounts.json"));
    assert!(
        e.set("hanse", r#"{"weather":"taifun","hermit":"gibtEsNicht"}"#)
            .is_err()
    );
    assert_eq!((read(e, "weather.json"), read(e, "mounts.json")), before);
}

/// Kataloge des Repos (Kopie, auf LF wie eingecheckt): denselben Eintrag erneut setzen ändert kein Byte; neu setzen und
/// entfernen stellt die Datei wieder her. Wie Go schreibt der Block `worlds` immer LF, auch in einer CRLF-Datei, darum
/// wird vorher normalisiert. Läuft ohne Spiel-EXE.
#[test]
fn kataloge_bleiben_byte_gleich() {
    let f = checkout(|_| failing("keine Engine"));
    let e = &f.editor;
    std::fs::write(e.file("sengoku"), "{}").unwrap();
    std::fs::write(e.file("hanse"), "{}").unwrap();
    for p in &PICKS {
        let name = Path::new(p.file).file_name().unwrap().to_str().unwrap();
        let before = read(e, name).replace("\r\n", "\n");
        std::fs::write(e.root.join("data").join(name), &before).unwrap();
        let worlds = worlds_of(before.as_bytes()).unwrap();
        let (_, value) = worlds.iter().find(|(w, _)| w == "sengoku").unwrap();
        e.set_pick("sengoku", p.key, value).unwrap();
        assert_eq!(read(e, name), before, "{name}: gleicher Wert");
        e.set_pick("hanse", p.key, value).unwrap();
        assert_ne!(read(e, name), before, "{name}: neuer Eintrag");
        e.set_pick("hanse", p.key, "").unwrap();
        assert_eq!(read(e, name), before, "{name}: entfernt");
    }
}

#[test]
fn patch_schluessel_werden_geprueft() {
    let f = checkout(|_| failing("keine Engine"));
    let e = &f.editor;
    assert_eq!(
        e.set("standard", "null").unwrap_err(),
        "patch muss ein JSON-Objekt sein"
    );
    assert_eq!(
        e.set("standard", r#"{"story":{}}"#).unwrap_err(),
        "story ändert world_story_set, nicht world_set"
    );
    assert_eq!(
        e.set("standard", r#"{"x":1}"#).unwrap_err(),
        "unbekanntes Feld \"x\" im patch (erlaubt: name, gewaesser, notes, extends, kind, combat, skins, islands, \
         weather, mount, hermit)"
    );
    assert_eq!(
        e.set_pick("standard", "x", "").unwrap_err(),
        "unbekannte Art \"x\" (erlaubt: hermit, mount, weather)"
    );
}
