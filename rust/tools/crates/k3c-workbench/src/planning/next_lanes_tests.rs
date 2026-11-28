//! Go `next_lanes_test.go`.

use std::collections::BTreeMap;

use super::create::{NewDoc, create};
use super::edit::set;
use super::next::{NextFilter, next};
use super::next_fixtures::{LaneSession, Plan, check_text, lane_repo};

fn run(plan: &Plan) -> String {
    next(&plan.root, &NextFilter::default()).expect("next")
}

/// Zwei offene DEV-Sessions mit verschiedenen Lanes laufen parallel.
#[test]
fn lanes_parallel() {
    let plan = lane_repo(&[("L1.1", "offen", "DEV", "mcp", ""), ("L2.1", "offen", "DEV", "ui", "")]);
    let out = run(&plan);
    assert!(out.starts_with("Nächste Session: L1.1 ·"), "{out}");
    check_text(
        &out,
        &[
            "Domäne: DEV · Lane: mcp · Umgebung: offline",
            "── Parallel möglich ──\n- DEV/ui · L2.1 · Titel L2.1",
        ],
        &[],
    );
}

/// DEV/mcp in Arbeit sperrt eine offene DEV/mcp, nicht DEV/ui.
#[test]
fn gleiche_lane_gesperrt() {
    let running = ("L1.1", "in Arbeit", "DEV", "mcp", "");
    let same = ("L2.1", "offen", "DEV", "mcp", "");
    let out = run(&lane_repo(&[running, same, ("L3.1", "offen", "DEV", "ui", "")]));
    assert!(out.starts_with("Nächste Session: L3.1 ·"), "{out}");
    let out = run(&lane_repo(&[running, same]));
    check_text(
        &out,
        &["L2.1 Domäne DEV, Lane mcp gesperrt (L1.1 in Arbeit)"],
        &["Nächste Session"],
    );
}

/// Eine Session ohne Lane sperrt die Domäne und wird von jeder Lane gesperrt.
#[test]
fn ohne_lane_sperrt_die_domaene() {
    for sessions in [
        [("L1.1", "in Arbeit", "DEV", "", ""), ("L2.1", "offen", "DEV", "ui", "")],
        [
            ("L1.1", "in Arbeit", "DEV", "ui", ""),
            ("L2.1", "offen", "DEV", "–", ""),
        ],
    ] {
        let out = run(&lane_repo(&sessions));
        check_text(
            &out,
            &["L2.1 Domäne DEV gesperrt (L1.1 in Arbeit)"],
            &["Nächste Session"],
        );
    }
    // Parallel: eine Session ohne Lane steht nie neben einer anderen derselben Domäne.
    let out = run(&lane_repo(&[
        ("L1.1", "offen", "DEV", "", ""),
        ("L2.1", "offen", "DEV", "ui", ""),
    ]));
    check_text(
        &out,
        &["Nächste Session: L1.1", "Parallel möglich ──\nkeine"],
        &["- DEV/ui"],
    );
}

/// Modell aus dem Feld, ohne Feld „sonnet (Standard)“, auch in der Parallel-Liste.
#[test]
fn zeigt_das_modell() {
    let out = run(&lane_repo(&[
        ("L1.1", "offen", "PAP", "renderer", "opus"),
        ("L2.1", "offen", "DEV", "", ""),
    ]));
    check_text(
        &out,
        &[
            "Domäne: PAP · Lane: renderer · Umgebung: offline · Modell: opus",
            "- DEV · L2.1 · Titel L2.1 (Sprint L2, offline) · Modell: sonnet (Standard)",
        ],
        &[],
    );
    let out = run(&lane_repo(&[("L1.1", "offen", "DEV", "", "–")]));
    check_text(
        &out,
        &["Domäne: DEV · Umgebung: offline · Modell: sonnet (Standard)"],
        &["Lane:"],
    );
}

/// Fünf freie Lanes: neben der Auswahl höchstens drei weitere.
#[test]
fn hoechstens_vier_parallel() {
    let sessions: Vec<LaneSession> = vec![
        ("L1.1", "offen", "PAP", "a", ""),
        ("L2.1", "offen", "PAP", "b", ""),
        ("L3.1", "offen", "PAP", "c", ""),
        ("L4.1", "offen", "PAP", "d", ""),
        ("L5.1", "offen", "PAP", "e", ""),
    ];
    let out = run(&lane_repo(&sessions));
    let (_, rest) = out.split_once("── Parallel möglich ──\n").expect("Parallel");
    let parallel = rest.split("\n\n").next().unwrap_or_default();
    assert_eq!(parallel.matches("- PAP/").count(), 3, "{out}");
    assert!(!parallel.contains("L5.1"), "{out}");
}

const SESSION_TEMPLATE: &str = "# SP00.1 · Titel\n\n- **Status:** offen\n- **Typ:** Umsetzung\n- **Agent:** autonom\n\
- **Domäne:** REG | DEV | PAP\n- **Lane:** – (oder Kurzname)\n- **Modell:** – | haiku | sonnet | opus | fable (Empfehlung)\n\
- **Umgebung:** offline | live | ?\n- **Branch:** sp00/1-x\n- **Abhängig von:** –\n- **Tickets:** B-000\n\
- **Kriterien:** AC-01\n\n## Ziel\n\nZiel.\n";

fn values(pairs: &[(&str, &str)]) -> BTreeMap<String, String> {
    pairs.iter().map(|(k, v)| ((*k).to_owned(), (*v).to_owned())).collect()
}

/// Domäne PAP beim Anlegen, Lane und Modell setzen (auch in Dateien ohne die Felder), ungültige Werte mit Wertebereich
/// abgelehnt.
#[test]
fn domaene_pap_gueltig() {
    let plan = lane_repo(&[("L1.1", "offen", "DEV", "", "")]);
    plan.write("vorlagen/session.md", SESSION_TEMPLATE);
    plan.write(
        "sprints/README.md",
        "# Fahrplan\n\n## Aktiv\n\n| Sprint | Domäne | Projekt | Thema | Am Ende sichtbar | Ordner |\n\
         |---|---|---|---|---|---|\n| L1 | DEV | LAN | Sprint L1 | – | `aktiv/L1-s/` |\n",
    );
    let new = NewDoc {
        kind: "session".to_owned(),
        id: "L1.2".to_owned(),
        slug: "neu".to_owned(),
        title: "Neu".to_owned(),
        fields: values(&[("Domäne", "PAP")]),
    };
    create(&plan.root, &new).expect("anlegen");
    check_text(
        &plan.read("sprints/aktiv/L1-s/L1.2-neu.md"),
        &["- **Domäne:** PAP\n- **Lane:** –\n- **Modell:** –\n- **Umgebung:** ?"],
        &[],
    );
    set(&plan.root, "L1.1", &values(&[("Lane", "mcp"), ("Modell", "opus")])).expect("setzen");
    check_text(
        &plan.read("sprints/aktiv/L1-s/L1.1-x.md"),
        &["- **Domäne:** DEV\n- **Lane:** mcp\n- **Modell:** opus\n- **Umgebung:** offline"],
        &[],
    );
    for bad in [("Lane", "MCP!"), ("Modell", "gpt"), ("Domäne", "XYZ")] {
        let error = set(&plan.root, "L1.2", &values(&[bad])).expect_err("angenommen");
        assert!(error.contains("ungültig"), "{bad:?}: {error}");
    }
    let error = set(&plan.root, "L1.2", &values(&[("Modell", "gpt")])).expect_err("angenommen");
    assert!(error.contains("haiku, sonnet, opus, fable"), "{error}");
}
