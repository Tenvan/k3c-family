//! Go `create_test.go` (B-595/AC-02): Ein Ticket aus dem Kern (Studio-Weg) erfüllt die Regeln der Planung, und `set`
//! zieht dieselbe Index-Zeile nach.

use k3c_assetkit::ticket::{Context, Input};

use super::create_fixtures::{index_links, new_project};
use super::edit::{head_fields, set, set_fields};
use super::fixtures::{doc, repo, values};
use super::planning::parse_ticket;
use super::tables::split_lines;

fn headings(text: &str) -> Vec<String> {
    split_lines(text).into_iter().filter(|l| l.starts_with("## ")).collect()
}

/// Felder, erlaubte Werte, Überschriften und Spec wie tests/planningDocs.ts › `checkTemplate`.
fn check_ticket_template(root: &std::path::Path, text: &str) {
    let template = doc(root, "vorlagen/ticket.md");
    let (known, fields) = (head_fields(&template), head_fields(text));
    let names = |map: &std::collections::HashMap<String, String>| {
        let mut names: Vec<String> = map.keys().cloned().collect();
        names.sort();
        names
    };
    assert_eq!(names(&fields), names(&known), "Felder wie die Vorlage");
    // `set_fields` prüft jeden Wert (Auswahl, Format), dazu Spec ↔ Freigabe und Platzhalter.
    let all = fields.into_iter().collect();
    set_fields(root, "ticket", text, &all).expect("Werte");
    assert_eq!(headings(text), headings(&template));
}

#[test]
fn kern_ticket_besteht_die_regeln() {
    let tmp = repo();
    let root = tmp.path();
    new_project(root, "WZG", "werkzeuge");
    let input = Input {
        titel: "Knopf fehlt".to_owned(),
        beschreibung: "Text".to_owned(),
        typ: "Idee".to_owned(),
        prio: "hoch".to_owned(),
        domaene: "DEV".to_owned(),
        projekt: "WZG".to_owned(),
        kontext: Context {
            app: "k3c-studio".to_owned(),
            seite: "grafik".to_owned(),
            ..Context::default()
        },
    };
    let got = k3c_assetkit::ticket::create(root, input).expect("Ticket");
    let rel = got.pfad.trim_start_matches("docs/").to_owned();
    let text = doc(root, &rel);
    check_ticket_template(root, &text);
    let ticket = parse_ticket(&text);
    assert_eq!(
        (ticket.nr.as_str(), ticket.title.as_str()),
        (got.id.as_str(), got.titel.as_str())
    );
    assert_ne!(ticket.project, "–");
    assert!(text.contains("## Akzeptanzkriterien\n\n- **AC-01**"), "{text}");
    let index = doc(root, "backlog/README.md");
    let file = rel.trim_start_matches("backlog/");
    assert!(
        index_links(&index)
            .iter()
            .any(|(nr, link, status)| *nr == got.id && link == file && status == "offen"),
        "Index-Zeile fehlt:\n{index}"
    );
    set(root, &got.id, &values(&[("Prio", "niedrig")])).expect("Set");
    let after = doc(root, "backlog/README.md");
    assert_eq!(after.matches(&format!("[{}]", got.id)).count(), 1, "{after}");
    assert!(after.contains("| DEV | Idee | niedrig | offen |"), "{after}");
}
