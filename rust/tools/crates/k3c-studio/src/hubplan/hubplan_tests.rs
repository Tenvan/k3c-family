//! Port von `hubplan_test.go` (B-462/AC-01, AC-08): Matrix-Modell des Gebäude-Planers aus `data/hub.json` bzw. der
//! 7-Stufen-Fixture.

use std::sync::Arc;

use super::editor_tests::{editor, read_repo, repo_root, write};
use super::{Editor, Engine, HUB_FILE};
use crate::test_helpers::tmp_dir;

/// Die 7-Stufen-Fixture liefert 7 Spalten, ohne feste 5 im Code.
#[test]
fn hub_plan_sieben_stufen() {
    let Some(planer) = editor(&repo_root()) else { return };
    let view = planer.plan_from("testdata/hub/hub7.json");
    assert!(
        view.error.is_empty() && view.findings.is_empty(),
        "{} {:?}",
        view.error,
        view.findings
    );
    assert_eq!(view.matrix.levels, 7);
}

#[test]
fn hub_plan_befunde_und_fehler() {
    let dir = tmp_dir();
    let Some(planer) = editor(dir.path()) else { return };
    let view = planer.plan_from(HUB_FILE);
    assert!(view.error.contains(HUB_FILE) && view.matrix.rows.is_empty(), "{view:?}");
    write(dir.path(), HUB_FILE, "{kaputt");
    let view = planer.plan_from(HUB_FILE);
    assert!(view.error.contains("nicht lesbar als hub.json"), "{}", view.error);
    write(dir.path(), HUB_FILE, r#"{"levels": "fünf"}"#);
    let view = planer.plan_from(HUB_FILE);
    assert!(
        view.error.contains("nicht lesbar als hub.json") && view.matrix.rows.is_empty(),
        "{}",
        view.error
    );

    let raw = read_repo(HUB_FILE).replacen(r#""kind": "workshop""#, r#""kind": "smithy""#, 1);
    write(dir.path(), HUB_FILE, &raw);
    let view = planer.plan_from(HUB_FILE);
    assert!(
        view.error.is_empty() && !view.findings.is_empty() && !view.matrix.rows.is_empty(),
        "{view:?}"
    );
    assert!(
        view.findings.iter().any(|f| f.rule == "pflichtplatz"),
        "{:?}",
        view.findings
    );
    // Geplante Chunks werden mitgeprüft (B-479).
    assert!(view.notes.is_empty(), "{:?}", view.notes);
}

/// Neu (ohne Spiel-EXE): Fehler der Engine und die JSON-Form der Antwort der Oberfläche.
#[test]
fn fehler_der_engine_und_json_form() {
    let engine: Engine = Arc::new(|_| Err("kaputt".to_owned()));
    let view = Editor::new("unbenutzt", engine).check("{}");
    assert_eq!(view.error, "data/hub.json nicht geprüft: kaputt");
    assert_eq!(view.raw, "{}");
    let json = serde_json::to_string(&view).unwrap();
    let want = concat!(
        r#"{"path":"data/hub.json","raw":"{}","matrix":{"levels":0,"rows":[]},"findings":[],"notes":[],"#,
        r#""error":"data/hub.json nicht geprüft: kaputt","stand":{"hub":"","rules":""},"kinds":[]}"#
    );
    assert_eq!(json, want);
}
