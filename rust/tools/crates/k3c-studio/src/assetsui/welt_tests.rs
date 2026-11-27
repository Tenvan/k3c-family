//! Port von `welt_test.go`.

use tempfile::TempDir;

use crate::assetsui::Dienst;
use crate::assetsui::test_helpers::schreibe;
use crate::test_helpers::tmp_dir;

/// standard (ohne Skins), sengoku (Skin Wolf → Ninja), dojo erbt von sengoku; nil steht nur in paper.json.
fn welt_root() -> TempDir {
    let tmp = tmp_dir();
    let dateien = [
        (
            "data/paper.json",
            r#"{"weltbilder":{"standard":{"gross":"hintergruende/weltbild-standard"},
            "sengoku":{"gross":"hintergruende/weltbild-sengoku"},"nil":{"gross":"hintergruende/weltbild-nil"}},
            "reittiere":{"horse":"pferd"},"leitmotive":{"sengoku":"hintergruende/leitmotiv-sengoku"}}"#,
        ),
        ("data/worlds/standard.json", r#"{"name":"Standard"}"#),
        (
            "data/worlds/sengoku.json",
            r#"{"extends":"standard","skins":{"gegner/wolf":"gegner/ninja"}}"#,
        ),
        (
            "data/worlds/dojo.json",
            r#"{"extends":"sengoku","skins":{"gegner/gier":"gegner/schatten"}}"#,
        ),
    ];
    for (pfad, inhalt) in dateien {
        schreibe(tmp.path().join(pfad), inhalt);
    }
    tmp
}

#[test]
fn welt_sicht_eigene_und_geerbte() {
    let root = welt_root();
    let s = Dienst::new(root.path()).welt_sicht("dojo").unwrap();
    for (asset, herkunft) in [
        ("gegner/ninja", "sengoku"),
        ("gegner/schatten", "dojo"),
        ("hintergruende/weltbild-sengoku", "sengoku"),
        ("hintergruende/leitmotiv-sengoku", "sengoku"),
        ("hintergruende/weltbild-standard", "standard"),
    ] {
        assert_eq!(s.eigene.get(asset).map(String::as_str), Some(herkunft), "{asset}");
    }
    // fremde Welt (nil) und ersetzte Basis-Assets sind aus
    for asset in ["hintergruende/weltbild-nil", "gegner/wolf", "gegner/gier"] {
        assert!(s.aus.iter().any(|a| a == asset), "{asset} fehlt in {:?}", s.aus);
    }
    assert!(!s.fehlt && s.basis == "standard", "{s:?}");
}

#[test]
fn welt_sicht_standard_sieht_keine_skins() {
    let root = welt_root();
    let s = Dienst::new(root.path()).welt_sicht("standard").unwrap();
    assert_eq!(s.eigene.len(), 1, "{:?}", s.eigene);
    assert_eq!(s.eigene["hintergruende/weltbild-standard"], "standard");
    assert!(!s.aus.iter().any(|a| a == "gegner/wolf"), "{:?}", s.aus);
    assert!(s.aus.iter().any(|a| a == "gegner/ninja"), "{:?}", s.aus);
}

#[test]
fn welt_sicht_feste_welt_ohne_datei() {
    let root = welt_root();
    let s = Dienst::new(root.path()).welt_sicht("nil").unwrap();
    assert!(s.fehlt, "{s:?}");
    assert_eq!(s.eigene["hintergruende/weltbild-nil"], "nil");
    assert!(!s.aus.iter().any(|a| a == "gegner/wolf"), "{:?}", s.aus);
}

#[test]
fn welt_sicht_lehnt_ungueltige_id_ab() {
    let root = tmp_dir();
    let e = Dienst::new(root.path()).welt_sicht("../x").unwrap_err();
    assert_eq!(e, r#"ungültige Welt-ID "../x""#);
}
