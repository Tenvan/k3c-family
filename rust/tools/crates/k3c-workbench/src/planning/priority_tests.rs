//! Go `priority_test.go`.

use super::priority::{order, rank};
use super::{Data, Project, Session, Sprint};

fn sprint(id: &str, status: &str, wanted: Option<&str>) -> Sprint {
    let sessions = wanted
        .map(|deps| Session {
            nr: format!("{id}.1"),
            text: format!("- **Abhängig von:** {deps}\n"),
            ..Session::default()
        })
        .into_iter()
        .collect();
    Sprint {
        id: id.to_owned(),
        status: status.to_owned(),
        sessions,
        ..Sprint::default()
    }
}

/// Sprints eines Projekts nach Rang und Tabellenplatz, die ohne Projekt dahinter; eine Voraussetzung erbt den Platz
/// ihres Abnehmers.
#[test]
fn rank_erbt_platz_und_folgt_abhaengigkeiten() {
    let mut data = Data {
        projects: vec![Project {
            id: "GRA".to_owned(),
            rang: "1".to_owned(),
            sprints: vec!["C1".to_owned(), "B1".to_owned()],
            ..Project::default()
        }],
        sprints: vec![
            sprint("A1", "geplant", Some("–")),
            sprint("B1", "geplant", Some("–")),
            sprint("C1", "aktiv", Some("A1.1, B-009")),
            sprint("X1", "erledigt", None),
            sprint("D1", "geplant", None),
        ],
        ..Data::default()
    };
    rank(&mut data);
    let ids: Vec<&str> = data.sprints.iter().map(|s| s.id.as_str()).collect();
    // A (ohne Projekt) erbt den Platz von C, das auf A1.1 wartet; D ohne Projekt zuletzt; Erledigte am Ende.
    assert_eq!(ids, ["A1", "C1", "B1", "D1", "X1"]);
    assert_eq!(data.sprints[1].deps, ["A1"]);
    assert_eq!(data.sprints[1].sessions[0].deps, ["A1.1"]);
}

#[test]
fn order_bricht_einen_zyklus() {
    let items = vec!["a", "b", "c"];
    let deps = |s: &&str| match *s {
        "a" => vec!["b".to_owned()],
        "b" => vec!["a".to_owned()],
        _ => Vec::new(),
    };
    let got = order(items, |s| (*s).to_owned(), deps, |_, _| false);
    assert_eq!(got, ["c", "a", "b"]);
}
