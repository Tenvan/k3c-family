//! Go `next_restbefunde_test.go`: Restbefunde Review WZ44.5 (B-839): Plätze, Ordner-Pfade, laufende Sessions ohne
//! Domäne, Branch-Zeile.

use super::next::{NextFilter, next};
use super::next_fixtures::{Plan, check_text, lane_repo, next_project, next_sprint};
use super::waves_spec::globs_overlap;

fn run(plan: &Plan) -> String {
    next(&plan.root, &NextFilter::default()).expect("next")
}

/// Drei Sessions in Arbeit: neben der Auswahl kein weiterer Vorschlag (vier Agenten).
#[test]
fn parallel_zaehlt_laufende() {
    let plan = lane_repo(&[
        ("L1.1", "in Arbeit", "SIM", "", ""),
        ("L2.1", "in Arbeit", "SRV", "", ""),
        ("L3.1", "in Arbeit", "CLI", "", ""),
        ("L4.1", "offen", "DEV", "", ""),
        ("L5.1", "offen", "INF", "", ""),
        ("L6.1", "offen", "PAP", "", ""),
    ]);
    check_text(
        &run(&plan),
        &["Nächste Session: L4.1 ·", "── Parallel möglich ──\nkeine"],
        &["L5.1 · Titel", "L6.1 · Titel"],
    );
    let plan = lane_repo(&[
        ("L1.1", "in Arbeit", "SIM", "", ""),
        ("L4.1", "offen", "DEV", "", ""),
        ("L5.1", "offen", "INF", "", ""),
        ("L6.1", "offen", "PAP", "", ""),
        ("L7.1", "offen", "REG", "", ""),
    ]);
    check_text(&run(&plan), &["- INF · L5.1", "- PAP · L6.1"], &["L7.1 · Titel"]);
}

/// Ein fester Pfad ohne `/` am Ende gilt auch als Ordner.
#[test]
fn ordner_ohne_schraegstrich() {
    for (a, b, want) in [
        ("tools/paper", "tools/paper/cli/**", true),
        ("tools/paper/cli/**", "tools/paper", true),
        ("tools/paper", "tools/paper/cli/main.ts", true),
        ("tools/paper", "tools/paper2/**", false),
        ("tools/paper", "tools/paper.go", false),
        ("a/b.go", "a/b.go", true),
    ] {
        assert_eq!(globs_overlap(a, b), want, "{a} {b}");
    }
}

/// Eine Session in Arbeit ohne Domäne sperrt mit ihren Dateien.
#[test]
fn laufende_ohne_domaene_sperrt() {
    let plan = lane_repo(&[("L1.1", "in Arbeit", "", "", ""), ("L2.1", "offen", "DEV", "", "")]);
    for nr in ["L1.1", "L2.1"] {
        let rel = format!("sprints/aktiv/{}-s/{nr}-x.md", &nr[..2]);
        let text = plan.read(&rel);
        plan.write(&rel, &format!("{text}\n## Erlaubte Dateien\n\n- `tools/x/**`\n"));
    }
    check_text(
        &run(&plan),
        &["L2.1 Bereich `tools/x/**` gesperrt (L1.1 in Arbeit)"],
        &["Nächste Session"],
    );
}

/// Die Branch-Zeile kommt aus derselben Session wie der Ref des origin-Overlays (die erste mit Feld Branch), nicht aus
/// der gewählten Session.
#[test]
fn branch_wie_im_overlay() {
    let plan = Plan::new();
    plan.write("projekte/DDD-d.md", &next_project("DDD", "aktiv", "1", &["D1"]));
    next_sprint(
        &plan,
        "aktiv",
        "D1",
        "DDD",
        &[
            ("D1.1", "autonom", "fertig", "DEV", "–", "AC-01"),
            ("D1.2", "autonom", "offen", "DEV", "D1.1", "AC-01"),
        ],
    );
    plan.write(
        "sprints/aktiv/D1-s/D1.1-x.md",
        "# D1.1 · Titel D1.1\n\n- **Agent:** autonom\n- **Domäne:** DEV\n- **Branch:** sprint/dd\n\
         - **Abhängig von:** –\n- **Kriterien:** AC-01\n\n## Ziel\n\nZiel.\n",
    );
    check_text(
        &run(&plan),
        &[
            "Nächste Session: D1.2 ·",
            "Branch: sprint/dd",
            "D1: kein origin/sprint/dd",
        ],
        &["Branch: sprint/d1"],
    );
}
