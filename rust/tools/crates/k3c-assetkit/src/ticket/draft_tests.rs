//! Port von `ticket/draft_test.go`.

use super::*;

/// Relationsfelder (B-696) bekommen `–`, wenn die Vorlage sie kennt; eine ältere Vorlage ohne sie bleibt gültig.
#[test]
fn draft_optionale_felder() {
    let relations = "- **Duplikat von:** – (Erklärung)\n- **Ersetzt:** – (Erklärung)\n";
    let tpl = format!(
        "# B-000 · Titel\n\n- **Status:** x\n- **Sprint:** x\n- **Projekt:** x\n- **Erstellt:** x\n- **Umgebung:** x\n\
         - **Spec:** x\n- **Revision:** x\n- **Freigabe:** x\n{relations}\n## A\n"
    );
    let today = || chrono::Local::now().fixed_offset();
    let got = draft(&tpl, "B-001", "Neu", &defaults(today())).unwrap();
    assert!(
        got.contains("- **Duplikat von:** –\n- **Ersetzt:** –\n"),
        "mit Feldern:\n{got}"
    );
    let old = tpl.replacen(relations, "", 1);
    let got = draft(&old, "B-001", "Neu", &defaults(today())).unwrap();
    assert!(!got.contains("Ersetzt"), "alte Vorlage:\n{got}");
}
