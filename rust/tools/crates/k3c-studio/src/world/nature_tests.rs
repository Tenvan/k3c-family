use super::super::world_tests::{STANDARD, fixture, read_world};
use super::patch_nature;

// B-891/AC-05: Festsetzen schreibt natureByDepth der Insel, die übrigen Bytes bleiben; Zurücksetzen stellt die Datei
// wieder her.
#[test]
fn natur_setzen_und_zuruecksetzen() {
    let Some(f) = fixture() else { return };
    let e = &f.editor;
    let v = e.set_nature("standard", "island1", 0, "forestDensity", Some(4), "k3c");
    assert!(
        v.rejected.is_empty() && v.error.is_empty(),
        "{} {}",
        v.rejected,
        v.error
    );
    let v = e.set_nature("standard", "island1", 0, "treeSpecies.tanne", Some(0), "k3c");
    let text = read_world(e, "standard");
    let want = r#""natureByDepth": {"0":{"traits":{"forestDensity":4},"weights":{"treeSpecies":{"tanne":0}}}} }"#;
    assert!(v.rejected.is_empty() && text.contains(want), "{}\n{text}", v.rejected);
    let head = &STANDARD[..STANDARD.find(" } ]").unwrap()];
    assert!(text.starts_with(head), "Kopf nicht byte-gleich:\n{text}");
    e.set_nature("standard", "island1", 0, "forestDensity", None, "k3c");
    e.set_nature("standard", "island1", 0, "treeSpecies.tanne", None, "k3c");
    assert_eq!(read_world(e, "standard"), STANDARD);
}

// Ein ungültiger Wert (Prüfung der Spiel-EXE) lässt die Datei unverändert und nennt den Grund.
#[test]
fn natur_lehnt_ungueltiges_ab() {
    let Some(f) = fixture() else { return };
    let e = &f.editor;
    let v = e.set_nature("standard", "island1", 0, "forestDensity", Some(9), "k3c");
    assert!(v.rejected.contains("außerhalb"), "{:?}", v.rejected);
    assert_eq!(read_world(e, "standard"), STANDARD);
    for (island, stage, key) in [
        ("fehlt", 0, "forestDensity"),
        ("island1", 5, "forestDensity"),
        ("island1", 0, "a.b.c"),
        ("island1", 0, ""),
    ] {
        let v = e.set_nature("standard", island, stage, key, Some(1), "k3c");
        assert!(!v.rejected.is_empty(), "{island} {stage} {key:?}: nicht abgelehnt");
    }
    assert_eq!(read_world(e, "standard"), STANDARD);
}

// Auch ein Fehlerfall (unbekannte Insel) hat Natur-Felder (leere Objekte und Listen, nie null).
#[test]
fn level_natur_nie_null() {
    let Some(f) = fixture() else { return };
    let v = f.editor.level("standard", "gibts-nicht", 0, "k3c");
    let got = [&v.nature, &v.nature_traits, &v.nature_weights, &v.nature_fixed].map(|raw| raw.get().to_owned());
    assert_eq!(got, ["{}", "[]", "[]", "{}"]);
}

// Format und Texte ohne Spiel-EXE (Gos `patchNature`).
#[test]
fn natur_patch_ohne_engine() {
    let raw = STANDARD.as_bytes();
    let set = patch_nature(raw, "island1", 1, "a<b", Some(2)).unwrap();
    // Wie Go `json.Marshal`: `<` im Schlüssel steht als Unicode-Escape in der Datei.
    let escaped = format!("a{}u003cb", char::from(92));
    let block = r#""harbor" } ] }, "natureByDepth": {"1":{"traits":{"KEY":2}}} }"#.replace("KEY", &escaped);
    let want = STANDARD.replacen(r#""harbor" } ] } }"#, &block, 1);
    assert_eq!(String::from_utf8_lossy(&set), want);
    assert_eq!(patch_nature(&set, "island1", 1, "a<b", None).unwrap(), raw);
    assert_eq!(patch_nature(raw, "island1", 0, "g.v", None).unwrap(), raw);
    let error = |island, stage, key| patch_nature(raw, island, stage, key, Some(1)).unwrap_err();
    assert_eq!(error("fehlt", 0, "x"), r#"die Insel "fehlt" gibt es nicht"#);
    assert_eq!(
        error("island1", -1, "x"),
        r#"die Stufe -1 gibt es auf der Insel "island1" nicht"#
    );
    assert_eq!(error("island1", 0, "a.b.c"), r#"ungültiger Natur-Schlüssel "a.b.c""#);
}
