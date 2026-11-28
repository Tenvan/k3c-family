//! Go `next_test.go` und `next_waves_test.go`.

use super::next::{NextFilter, next};
use super::next_fixtures::{check_text, next_repo, waves_repo};
use super::next_lanes::{Scoped, busy};
use super::next_origin::sprint_branch;
use super::planning::Session;

fn domain(d: &str) -> NextFilter {
    NextFilter { domain: d.to_owned() }
}

/// BBB vor AAA; B1.1 Mensch, B1.2 und B1.3 warten, B1.4 ist frei (E1.1 fertig, B-011 im Archiv). Parallel nur andere
/// Domänen, je Sprint eine (B-838), ohne gesperrten Sprint A1 und ohne ruhendes Projekt.
#[test]
fn nach_rang_und_abhaengigkeit() {
    let plan = next_repo();
    let out = next(&plan.root, &NextFilter::default()).expect("next");
    assert!(out.starts_with("Nächste Session: B1.4 · Titel B1.4\n"), "{out}");
    check_text(
        &out,
        &[
            "Pfad: docs/sprints/aktiv/B1-s/B1.4-x.md",
            "Sprint: B1 · Sprint B1 (Projekt BBB, Rang 1)",
            "Domäne: CLI · Umgebung: offline",
            "Branch: sprint/b1",
            "Kriterien: AC-02",
            "Ziel B1.4.",
            "- **AC-02** Zweites\n  mit Folgezeile.",
            "Stand der Sprints",
            "── Parallel möglich ──\n- INF · A2.3 · Titel A2.3 (Sprint A2, offline)",
        ],
        &["- **AC-01**", "Nachsatz", "A2.4", "A2.5", "R1.1", "SRV ·"],
    );
}

/// Filter wählt A2.3 (Kriterien alle), parallel steht die freie CLI-Session aus BBB.
#[test]
fn mit_domaene() {
    let plan = next_repo();
    let out = next(&plan.root, &domain("inf")).expect("next");
    assert!(out.starts_with("Nächste Session: A2.3 ·"), "{out}");
    check_text(
        &out,
        &["- **AC-01** Erstes.\n- **AC-02** Zweites", "- CLI · B1.4"],
        &["A2.5", "- INF"],
    );
    assert!(
        next(&plan.root, &domain("XYZ")).is_err(),
        "unbekannte Domäne angenommen"
    );
}

/// Sprint A1 ist durch A1.1 gesperrt (B-838); die Gründe stehen je Projekt in Rang-Reihenfolge, gekürzt.
#[test]
fn keine_frei() {
    let plan = next_repo();
    let out = next(&plan.root, &domain("SRV")).expect("next");
    let want = "Keine Session frei in Domäne SRV. Gründe je Projekt:\n\
                - BBB: B1.1 Agent Mensch; B1.2 wartet auf B1.1 (offen); B1.3 wartet auf B-010 (offen); … 1 weitere\n\
                - AAA: A1.2 Sprint A1 gesperrt (A1.1 in Arbeit); A2.3 andere Domäne INF; A2.4 andere Domäne CLI; … 1 weitere\n\
                - CCC: kein aktiver Sprint\n";
    assert!(out.starts_with(want) && !out.contains("RUH"), "{out}");
}

/// B-782/AC-01: Ein Branch mit führendem sprint/ bleibt, sonst gilt sprint/ + Teil vor dem ersten /.
#[test]
fn sprint_branch_aus_dem_feld() {
    for (id, field, want) in [
        ("CU1", "sprint/cu1", "sprint/cu1"),
        ("CU1", "`sprint/cu1`", "sprint/cu1"),
        ("W15", "w15/1-x", "sprint/w15"),
        ("W15", "w15/1-fackeln", "sprint/w15"),
        ("W15", "", "sprint/w15"),
        ("W15", "ohne-strich", "sprint/w15"),
        ("W15", "sprint/", "sprint/w15"),
    ] {
        assert_eq!(sprint_branch(id, field), want, "{id} {field:?}");
    }
}

/// B-783/AC-03: `plan_next` prüft wie `plan_waves` nach Dateimengen und Sperren: R1.1 (DEV, in Arbeit) sperrt A1.1
/// (DEV) nicht, C1.2 (disjunkte Erlaubte Dateien) ist parallel möglich; B1.1 (Sperre exe) und C1.1 (Bereich) nicht.
#[test]
fn parallel_gleiche_domaene_disjunkte_dateien() {
    let plan = waves_repo();
    let out = next(&plan.root, &NextFilter::default()).expect("next");
    check_text(
        &out,
        &[
            "Nächste Session: A1.1 · Titel A1.1",
            "── Parallel möglich ──\n- DEV · C1.2 · Titel C1.2 (Sprint C1, offline)",
        ],
        &["- DEV · B1.1", "- DEV · C1.1", "gesperrt"],
    );
}

fn scoped(nr: &str, area: &[&str]) -> Scoped {
    Scoped {
        session: Session {
            nr: nr.to_owned(),
            domain: "DEV".to_owned(),
            ..Session::default()
        },
        area: area.iter().map(|a| (*a).to_owned()).collect(),
        ..Scoped::default()
    }
}

/// Ohne Dateimengen bleibt die alte Sperre nach Domäne; mit Dateimengen nennt der Grund die Kollision.
#[test]
fn grund_bei_kollision() {
    let plan = waves_repo();
    let out = next(&plan.root, &domain("DEV")).expect("next");
    check_text(&out, &["Nächste Session: A1.1"], &[]);
    let running = [scoped("R1.1", &["tools/a/**"])];
    assert_eq!(
        busy(&running, &scoped("C1.1", &["tools/a/sub/file.go"])).as_deref(),
        Some("Bereich `tools/a/**` / `tools/a/sub/file.go` gesperrt (R1.1 in Arbeit)")
    );
    assert_eq!(
        busy(&running, &scoped("C1.1", &[])).as_deref(),
        Some("Domäne DEV gesperrt (R1.1 in Arbeit)")
    );
}
