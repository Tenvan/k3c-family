//! Go `check_test.go`.

use super::check::{CheckIn, check};
use super::next_fixtures::{Plan, check_text, rules, waves_repo};

fn run(plan: &Plan, id: &str) -> String {
    check(&plan.root, &CheckIn { id: id.to_owned() }).expect("check")
}

/// B-784/AC-01: Bereich leer (B-003) und an B-020 Umgebung ?, Größe L, unbekannte ID und die Überlappung mit dem
/// offenen Ticket B-001 (tools/d/**).
#[test]
fn meldet_maengel() {
    let plan = waves_repo();
    let text = format!(
        "# B-020 · Ticket B-020\n\n- **Domäne:** DEV\n- **Typ:** Idee\n- **Prio:** mittel\n- **Umgebung:** ?\n\
         - **Status:** offen\n- **Sprint:** –\n- **Projekt:** BBB\n- **Spec:** Entwurf\n\n## Ziel\n\nZiel.\n\n{}",
        rules("`tools/d/x.go`", "B-999, A1.1, irgendwann", "–", "L", "–")
    );
    plan.write("backlog/B-020-t.md", &text);
    let want = "B-020: 5 Mängel\n- Umgebung: ?\n- Größe: L (vor der Freigabe in Tickets je Teil zerlegen)\n\
                - unbekannte ID B-999 in „Abhängig von“\n- keine ID in „Abhängig von“: irgendwann\n\
                - Bereich überlappt mit B-001 (`tools/d/x.go` / `tools/d/**`)";
    assert_eq!(run(&plan, "B-020"), want);
    check_text(&run(&plan, "B-003"), &["B-003: 1 Mängel\n- Bereich leer"], &[]);
    check_text(&run(&plan, "B-002"), &["B-002: keine Mängel"], &[]);
    check_text(
        &run(&plan, "B-001"),
        &["B-001: 1 Mängel\n- Bereich überlappt mit B-020"],
        &[],
    );
    check_text(&run(&plan, "A1"), &["A1: keine Mängel"], &[]);
}

/// Fehlende Zeilen, archivierte Tickets und falsche IDs.
#[test]
fn zeilen_archiv_und_fehler() {
    let plan = waves_repo();
    plan.write(
        "backlog/B-021-t.md",
        "# B-021 · Ohne Angaben\n\n- **Umgebung:** offline\n- **Status:** offen\n\n## Ziel\n\nZiel.\n",
    );
    plan.write(
        "backlog/B-022-t.md",
        "# B-022 · Halb\n\n- **Umgebung:** offline\n- **Status:** offen\n\n\
         ## Regeln und Einschränkungen\n\n- **Bereich:** `x/**`\n- **Sperre:** –\n",
    );
    plan.write("backlog/archiv/B-023-t.md", "# B-023 · Alt\n\n- **Status:** erledigt\n");
    check_text(&run(&plan, "B-021"), &["Planungsangaben fehlen"], &["Zeile"]);
    check_text(
        &run(&plan, "B-022"),
        &[
            "Zeile „Abhängig von“ fehlt",
            "Zeile „Braucht 🧑“ fehlt",
            "Zeile „Größe“ fehlt",
        ],
        &["Zeile „Bereich“", "Zeile „Sperre“"],
    );
    check_text(&run(&plan, "B-023"), &["archiviert, nicht geprüft"], &[]);
    for id in ["B-404", "A1.1", "quatsch"] {
        assert!(
            check(&plan.root, &CheckIn { id: id.to_owned() }).is_err(),
            "{id} angenommen"
        );
    }
}
