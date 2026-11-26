//! Port von `hubplan_rules_test.go` (B-462/AC-03, AC-04): Abweichung der Regeltabelle (auf einer Kopie) und Vorschau
//! des Planer-Stands. Die Regeltabelle braucht keine Engine, die Tests laufen auch ohne Spiel-EXE.

use std::path::Path;
use std::sync::Arc;

use super::super::editor_tests::{planer_armory_from3, read_repo, repo_root, write};
use super::super::{Editor, Engine, HUB_FILE};
use super::{RULES_FILE, diff_lines};
use crate::test_helpers::tmp_dir;

fn planer(root: &Path) -> Editor {
    let engine: Engine = Arc::new(|_| Err("keine Engine".to_owned()));
    Editor::new(root, engine)
}

#[test]
fn hub_rules_ohne_abweichung() {
    let view = planer(&repo_root()).rules("");
    assert!(view.error.is_empty() && view.same && view.lines.is_empty(), "{view:?}");
    assert_eq!(view.ist, view.soll);
    assert_eq!((view.path.as_str(), view.task.as_str()), (RULES_FILE, "task gen:rules"));
}

/// Eine absichtlich abweichende Tabelle (Kopie) wird mit Zeile, Ist und Soll erkannt.
#[test]
fn hub_rules_abweichende_tabelle() {
    let dir = tmp_dir();
    let doc = read_repo(RULES_FILE);
    let (from, to) = ("| 4 | Eisen | Rüstkammer +40 |", "| 4 | Eisen | Rüstkammer +44 |");
    assert_eq!(
        doc.matches(from).count(),
        1,
        "Regeltabelle enthält {from:?} nicht genau einmal"
    );
    write(dir.path(), RULES_FILE, &doc.replacen(from, to, 1));
    write(dir.path(), HUB_FILE, &read_repo(HUB_FILE));
    let view = planer(dir.path()).rules("");
    assert!(view.error.is_empty() && !view.same && view.lines.len() == 1, "{view:?}");
    let line = &view.lines[0];
    assert!(
        line.ist.contains("Rüstkammer +44") && line.soll.contains("Rüstkammer +40"),
        "{line:?}"
    );
    assert_eq!(line.line, 7);
    assert_eq!(
        read_repo(RULES_FILE),
        doc,
        "die Regeltabelle des Checkouts wurde geändert"
    );
}

#[test]
fn hub_rules_planer_stand_und_fehler() {
    let view = planer(&repo_root()).rules(&planer_armory_from3());
    assert!(view.error.is_empty() && !view.same && view.lines.len() == 2, "{view:?}");
    assert!(view.lines[0].soll.contains("Rüstkammer +40"), "{:?}", view.lines);
    assert!(!view.lines[1].soll.contains("Rüstkammer"), "{:?}", view.lines);

    let dir = tmp_dir();
    let leer = planer(dir.path());
    assert!(leer.rules("").error.contains(HUB_FILE));
    assert!(leer.rules(&read_repo(HUB_FILE)).error.contains(RULES_FILE));
    write(dir.path(), RULES_FILE, "ohne Marken");
    let error = leer.rules(&read_repo(HUB_FILE)).error;
    assert!(error.contains("nicht vergleichbar"), "{error}");
}

/// Neu: Zeilen, die auf einer Seite fehlen, tragen dort leeren Text.
#[test]
fn zeilen_ohne_gegenstueck() {
    let lines = diff_lines("a\nb", "a\nc\nd");
    let got: Vec<(usize, &str, &str)> = lines
        .iter()
        .map(|l| (l.line, l.ist.as_str(), l.soll.as_str()))
        .collect();
    assert_eq!(got, [(2, "b", "c"), (3, "", "d")]);
}
