//! Go `waves_test.go`, `waves_spec_test.go` und `waves_sprint_test.go`.

use super::next::{NextFilter, next};
use super::next_fixtures::{Plan, check_text, next_project, wave_sprint, waves_repo};
use super::waves::{WavesFilter, waves};
use super::waves_spec::{allowed_files, globs_overlap, parse_plan_spec};

fn run(plan: &Plan, wave_count: i64, project: &str) -> String {
    let filter = WavesFilter {
        wave_count,
        project: project.to_owned(),
        ..WavesFilter::default()
    };
    waves(&plan.root, &filter).expect("waves")
}

/// B-783/AC-01: Abhängigkeit (A1.2 nach A1.1), gleiche Sperre (B1.1 neben A1.1), überlappender Bereich (C1.1 unter
/// tools/a/**), volle Plätze (B-002) und Knoten ohne Welle mit Grund.
#[test]
fn abhaengigkeit_bereich_sperre() {
    let out = run(&waves_repo(), 4, "");
    check_text(
        &out,
        &[
            "Arbeitswellen: 1 in Arbeit, 7 eingeplant in 4 Wellen (4 gezeigt), 3 in keiner Welle · Lanes: zweit (Second Worker), haupt (Hauptrechner)",
            "── Welle 1, jetzt (2 Knoten) ──\n- R1.1 · in Arbeit · Titel R1.1 (DEV, Projekt AAA, Sprint R1) · Lane: zweit\n- B-001 · Ticket · Ticket B-001 (DEV, Projekt AAA) · Lane: haupt\n",
            "── Welle 2 (2 Knoten) ──\n- A1.1 · Session · Titel A1.1 (DEV, Projekt AAA, Sprint A1) · Lane: zweit · Sperre exe — Lanes belegt\n",
            "- B-003 · Ticket · Ticket B-003 — Bereich fehlt",
            "- Fehler: B-004 · Ticket · Ticket B-004 — unbekannte ID B-999 in „Abhängig von“",
            "- B-005 · Ticket · Ticket B-005 — wartet auf B-003 (nicht in einer Welle)",
        ],
        &["B-006", "B-007", "B-008", "A1.3", "C1.3"],
    );
}

/// Filter grenzen nur die Anzeige ein; `wave_count` zeigt nur die ersten N Wellen (Standard 1), negativ lehnt ab.
#[test]
fn filter_und_wellenzahl() {
    let plan = waves_repo();
    check_text(
        &run(&plan, 4, "aaa"),
        &["A1.1", "Anzeige gefiltert"],
        &["C1.2", "- B1.1"],
    );
    check_text(&run(&plan, 0, ""), &["Welle 1, jetzt", "(1 gezeigt)"], &["── Welle 2"]);
    check_text(&run(&plan, 2, ""), &["── Welle 2 "], &["── Welle 3"]);
    let negative = WavesFilter {
        wave_count: -1,
        ..WavesFilter::default()
    };
    assert_eq!(
        waves(&plan.root, &negative).expect_err("angenommen"),
        "wave_count -1 ungültig: mindestens 1"
    );
}

/// `Lane: zweit` gehört dem Second Worker, Lane-Wert `a` dem Hauptrechner, sonst die erste freie Lane (zweit zuerst).
/// Drei Sprints mit getrennten Bereichen: zwei Lanes, also passt der dritte Knoten nicht in Welle 1.
#[test]
fn lanes_verteilung() {
    let plan = Plan::new();
    plan.write(
        "projekte/AAA-a.md",
        &next_project("AAA", "aktiv", "1", &["A1", "B1", "C1"]),
    );
    for (id, nr, files) in [
        ("A1", "A1.1", "tools/a/**"),
        ("B1", "B1.1", "tools/b/**"),
        ("C1", "C1.1", "tools/c/**"),
    ] {
        wave_sprint(
            &plan,
            id,
            "AAA",
            "aktiv",
            "–",
            &[(nr, "autonom", "offen", "–", files, "")],
        );
    }
    for (rel, lane) in [
        ("sprints/aktiv/A1-s/A1.1-x.md", "zweit"),
        ("sprints/aktiv/B1-s/B1.1-x.md", "a"),
    ] {
        let with = format!("- **Domäne:** DEV\n- **Lane:** {lane}\n");
        plan.replace(rel, "- **Domäne:** DEV\n", &with);
    }
    check_text(
        &run(&plan, 2, ""),
        &[
            "- A1.1 · Session · Titel A1.1 (DEV, Projekt AAA, Sprint A1) · Lane: zweit",
            "- B1.1 · Session · Titel B1.1 (DEV, Projekt AAA, Sprint B1) · Lane: haupt",
            "── Welle 2 (1 Knoten) ──\n- C1.1 · Session · Titel C1.1 (DEV, Projekt AAA, Sprint C1) · Lane: zweit",
        ],
        &[],
    );
}

#[test]
fn planungsangaben_und_globs() {
    let spec = parse_plan_spec(&super::next_fixtures::rules(
        "`a/**`, `docs/glossar.md`, `b.go`",
        "B-1, SP12.3, irgendwas",
        "–",
        "M",
        "`exe`, golden",
    ));
    assert_eq!(spec.area, ["a/**", "b.go"]);
    assert_eq!(spec.deps, ["B-1", "SP12.3"]);
    assert_eq!(spec.bad_deps, ["irgendwas"]);
    assert_eq!((spec.needs.as_str(), spec.size.as_str()), ("–", "M"));
    assert_eq!(spec.locks, ["exe", "golden"]);
    for (a, b, want) in [
        ("tools/a/**", "tools/a/x.go", true),
        ("tools/assets/style/**", "tools/assets/sound/**", false),
        ("tools/paper/cli/gen.ts", "tools/paper/cli/gen.ts", true),
        ("tools/paper/cli/gen.ts", "tools/paper/cli/gen.tsx", false),
        ("x/tools_planning*.go", "x/tools_sim*.go", false),
        ("x/**", "x/tools_sim*.go", true),
        ("docs/", "docs/arbeitsweise.md", true),
    ] {
        assert_eq!(globs_overlap(a, b), want, "{a} {b}");
    }
}

/// Mehrere Pfade in einem Stichpunkt zählen alle (Review WZ44.5), Backticks ohne Pfad nicht.
#[test]
fn erlaubte_dateien_alle_pfade_je_zeile() {
    let text = "# X\n\n## Erlaubte Dateien\n\n- `engine/sim/**`, `engine/net/**`\n\
                - `docs/sprints/**` (nur über `plan_*`)\n- keine\n\n## Nicht-Ziele\n\n- `src/nicht.ts`\n";
    assert_eq!(
        allowed_files(text),
        ["engine/sim/**", "engine/net/**", "docs/sprints/**"]
    );
}

/// B-838/AC-01: Sessions desselben Sprints laufen nie parallel (ein Sprint-Branch), auch ohne Sperre und mit getrennten
/// Erlaubten Dateien.
fn same_sprint_repo(first_status: &str) -> Plan {
    let plan = Plan::new();
    plan.write("projekte/CCC-c.md", &next_project("CCC", "aktiv", "1", &["C1", "D1"]));
    wave_sprint(
        &plan,
        "C1",
        "CCC",
        "aktiv",
        "–",
        &[
            ("C1.1", "autonom", first_status, "–", "a/**", ""),
            ("C1.2", "autonom", "offen", "–", "b/**", ""),
        ],
    );
    wave_sprint(
        &plan,
        "D1",
        "CCC",
        "aktiv",
        "–",
        &[("D1.1", "autonom", "offen", "–", "d/**", "")],
    );
    plan
}

#[test]
fn sessions_eines_sprints_nacheinander() {
    let plan = same_sprint_repo("offen");
    check_text(
        &run(&plan, 2, ""),
        &[
            "── Welle 1, jetzt (2 Knoten) ──\n- C1.1 · Session · Titel C1.1 (DEV, Projekt CCC, Sprint C1) · Lane: zweit\n- D1.1 · Session · Titel D1.1 (DEV, Projekt CCC, Sprint D1) · Lane: haupt",
            "── Welle 2 (1 Knoten) ──\n- C1.2 · Session · Titel C1.2 (DEV, Projekt CCC, Sprint C1) · Lane: zweit — kollidiert mit C1.1: Sprint C1",
        ],
        &[],
    );
    let out = next(&plan.root, &NextFilter::default()).expect("next");
    check_text(
        &out,
        &["Nächste Session: C1.1", "── Parallel möglich ──\n- DEV · D1.1"],
        &["C1.2 ·"],
    );
}

#[test]
fn sprint_gesperrt_solange_eine_session_laeuft() {
    let out = next(&same_sprint_repo("in Arbeit").root, &NextFilter::default()).expect("next");
    check_text(
        &out,
        &["Nächste Session: D1.1"],
        &["Nächste Session: C1.2", "- DEV · C1.2"],
    );
}

/// Eigener Test: Abhängigkeiten im Kreis brechen nicht ab, beide stehen in keiner Welle.
#[test]
fn abhaengigkeiten_im_kreis() {
    let plan = Plan::new();
    plan.write("projekte/AAA-a.md", &next_project("AAA", "aktiv", "1", &["A1", "B1"]));
    wave_sprint(
        &plan,
        "A1",
        "AAA",
        "aktiv",
        "–",
        &[("A1.1", "autonom", "offen", "B1.1", "a/**", "")],
    );
    wave_sprint(
        &plan,
        "B1",
        "AAA",
        "aktiv",
        "–",
        &[("B1.1", "autonom", "offen", "A1.1", "b/**", "")],
    );
    check_text(
        &run(&plan, 3, ""),
        &["0 eingeplant", "2 in keiner Welle", "Abhängigkeiten im Kreis"],
        &[],
    );
}
