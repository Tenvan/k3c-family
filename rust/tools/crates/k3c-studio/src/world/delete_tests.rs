use tempfile::TempDir;

use super::super::Editor;
use super::super::world_tests::failing;
use crate::test_helpers::tmp_dir;

/// Eigener Temp-Checkout; der Ordner lebt so lange wie der zweite Wert.
fn delete_root() -> (Editor, TempDir) {
    let dir = tmp_dir();
    for (rel, text) in [
        ("data/paper.json", r#"{"weltbilder":{"standard":{},"sengoku":{}}}"#),
        ("data/worlds/standard.json", r#"{"id":"standard","name":"Standard"}"#),
        ("data/worlds/sengoku.json", r#"{"id":"sengoku","name":"Sengoku"}"#),
        ("data/worlds/eltern.json", r#"{"id":"eltern","name":"Eltern"}"#),
        (
            "data/worlds/kind.json",
            r#"{"id":"kind","name":"Kind","extends":"eltern"}"#,
        ),
        ("data/worlds/frei.json", r#"{"id":"frei","name":"Frei"}"#),
        ("saves/alt.json", r#"{"world":"frei"}"#),
        ("saves/neu.json", r#"{"world":"standard"}"#),
    ] {
        let path = dir.path().join(rel);
        std::fs::create_dir_all(path.parent().unwrap()).unwrap();
        std::fs::write(path, text).unwrap();
    }
    (Editor::new(dir.path(), failing("keine Engine")), dir)
}

#[test]
fn loescht_freie_welt_und_warnt_vor_spielstaenden() {
    let (e, _dir) = delete_root();
    let warnings = e.delete("frei").unwrap().unwrap_or_default();
    assert!(!e.file("frei").exists(), "Datei noch da");
    assert_eq!(
        warnings,
        ["Spielstände nutzen die gelöschte Welt und lassen sich nicht mehr laden: alt"]
    );
    assert!(e.root.join("saves").join("alt.json").exists(), "Spielstand angefasst");
}

#[test]
fn lehnt_feste_welten_erben_und_unbekanntes_ab() {
    let (e, _dir) = delete_root();
    for (id, want) in [
        ("standard", "feste Welt"),
        ("sengoku", "feste Welt"),
        ("eltern", "Erben (kind)"),
        ("gibt-es-nicht", "gibt es nicht"),
        ("../x", "ungültige Welt-ID"),
    ] {
        let error = e.delete(id).unwrap_err();
        assert!(error.contains(want), "delete({id:?}): {error}, will {want:?}");
    }
    for id in ["standard", "sengoku", "eltern"] {
        assert!(e.file(id).exists(), "{id} gelöscht trotz Ablehnung");
    }
}

// Review RU70.5: Ein doppeltes `extends` darf den Erben nicht verstecken.
#[test]
fn doppeltes_extends_bleibt_erbe() {
    let (e, _dir) = delete_root();
    std::fs::write(
        e.file("kind"),
        r#"{"id":"kind","name":"Kind","extends":"frei","extends":"eltern"}"#,
    )
    .unwrap();
    let error = e.delete("eltern").unwrap_err();
    assert!(error.contains("Erben (kind)"), "{error}");
    assert!(e.file("eltern").exists(), "Eltern trotz Erbe gelöscht");
}

#[test]
fn eltern_nach_dem_kind_loeschbar() {
    let (e, _dir) = delete_root();
    assert_eq!(e.delete("kind").unwrap(), None);
    e.delete("eltern").unwrap();
    // Antwort für die Seite: Warnungen nie `null`.
    let view = e.delete_view("eltern");
    assert_eq!(view.warnings, Some(Vec::new()));
    assert_eq!(view.error, "data/worlds/eltern.json gibt es nicht");
}

#[test]
fn ohne_paper_json_loescht_nichts() {
    let (e, _dir) = delete_root();
    std::fs::remove_file(e.root.join("data").join("paper.json")).unwrap();
    let error = e.delete("frei").unwrap_err();
    assert!(error.contains("paper.json"), "{error}");
    assert!(e.file("frei").exists(), "Datei trotz Ablehnung gelöscht");
}

// Review WZ62.5: Kaputtes oder leeres paper.json verhält sich wie ein fehlendes.
#[test]
fn kaputtes_paper_json_loescht_nichts() {
    for (name, raw) in [
        ("kaputt", r#"{"weltbilder":"#),
        ("ohne weltbilder", "{}"),
        ("leer", r#"{"weltbilder":{}}"#),
    ] {
        let (e, _dir) = delete_root();
        std::fs::write(e.root.join("data").join("paper.json"), raw).unwrap();
        let error = e.delete("frei").unwrap_err();
        assert!(error.contains("paper.json"), "{name}: {error}");
        assert!(e.file("frei").exists(), "{name}: Datei trotz Ablehnung gelöscht");
    }
}
