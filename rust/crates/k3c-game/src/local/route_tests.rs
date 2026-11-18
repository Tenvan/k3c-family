use super::route::load_world_def;
use super::testutil::TempDir;

// Die Standard-Welt liefert ihre Inseln mit Etappe (>= 1) für die Seekarte; eine unbekannte Welt-Datei ist ein Fehler.
// Go setzt `K3C_WORLDS_DIR` im Test; Rust übergibt den Ordner (`set_var` ist unsafe, der Crate verbietet `unsafe`).
#[test]
#[ignore = "wartet auf RU19 (decode_world_story: standard_world)"]
fn load_world_def_route() {
    let def = load_world_def(None, "").expect("Standard-Welt");
    assert!(!def.islands.is_empty(), "Standard-Welt ohne Inseln");
    let dir = TempDir::new();
    assert!(
        load_world_def(Some(dir.path()), "gibt-es-nicht").is_err(),
        "unbekannte Welt: Fehler"
    );
}

// Der Fehlerteil von `TestLoadWorldDefRoute`, der die Standard-Welt nicht braucht (läuft schon vor RU19).
#[test]
fn load_world_def_unknown_world() {
    let dir = TempDir::new();
    assert!(
        load_world_def(Some(dir.path()), "gibt-es-nicht").is_err(),
        "unbekannte Welt: Fehler"
    );
    assert_eq!(
        load_world_def(None, "gibt-es-nicht").expect_err("ohne Ordner"),
        "kein Welten-Ordner"
    );
    let err = load_world_def(Some(dir.path()), "../x").expect_err("ungültige ID");
    assert!(err.contains("ungültig"), "ungültige ID: {err}");
}
