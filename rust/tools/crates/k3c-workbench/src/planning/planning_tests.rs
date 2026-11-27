//! Go `planning_test.go`.

use serde_json::json;

use super::Sprint;
use super::planning::{available, doc, load, parse_sprint, parse_ticket};
use super::testsupport::{real_root, write};
use super::worktree::{mark_worktrees, parse_worktrees};
use crate::testutil::tmp_dir;

const SPRINT: &str = "# SP11 · SRV · Raspberry Pi\n\n- **Status:** aktiv\n- **Domäne:** SRV\n- **Tickets:** B-028, B-035\n\
- **Spec:** freigegeben\n\n## Sessions\n\n| Nr. | Datei | Typ | Agent | Status |\n|---|---|---|---|---|\n\
| SP11.1 | `a.md` | Umsetzung | autonom | fertig |\n| SP11.2 | `b.md` | Workshop | Mensch | offen |\n\n\
## Abnahme\n\n| X.1 | `a` | b | c | d |\n";

const DRAFT: &str = "# F1 · REG · Zielkorridore\n\n- **Reife:** Entwurf\n\n## Sessions\n\n\
- F1.1 🧑 Workshop Zielkorridore (AC-01)\n";

#[test]
fn parse_sprint_liest_kopf_und_sessions() {
    let s = parse_sprint(SPRINT, "SP11-raspberry-pi", "aktiv");
    assert_eq!(
        (s.id.as_str(), s.domain.as_str(), s.title.as_str()),
        ("SP11", "SRV", "Raspberry Pi")
    );
    assert_eq!(s.tickets, ["B-028", "B-035"]);
    assert_eq!(
        s.sessions.len(),
        2,
        "Tabelle unter Abnahme zählt nicht: {:?}",
        s.sessions
    );
    assert_eq!(
        (s.sessions[0].status.as_str(), s.sessions[0].file.as_str()),
        ("fertig", "a.md")
    );
    assert_eq!(s.sessions[1].agent, "Mensch");
    let d = parse_sprint(DRAFT, "F1-x", "geplant");
    assert_eq!(d.sessions.len(), 1, "{d:?}");
    assert_eq!(
        (d.sessions[0].status.as_str(), d.reife.as_str()),
        ("entwurf", "Entwurf")
    );
}

#[test]
fn stichpunkte_neben_der_tabelle() {
    let text = SPRINT.replacen(
        "\n\n## Abnahme",
        "\n\n- SP11.1 Stichpunkt zur Tabelle\n- SP11.2 noch einer\n- SP11.3 nur als Stichpunkt\n\n## Abnahme",
        1,
    );
    let s = parse_sprint(&text.replace('\n', "\r\n"), "SP11-raspberry-pi", "aktiv");
    let found: Vec<(&str, &str)> = s.sessions.iter().map(|x| (x.nr.as_str(), x.status.as_str())).collect();
    assert_eq!(
        found,
        [("SP11.1", "fertig"), ("SP11.2", "offen"), ("SP11.3", "entwurf")]
    );
    assert_eq!(s.sessions[2].titel, "nur als Stichpunkt");
}

#[test]
fn load_liest_sprints_und_tickets() {
    let tmp = tmp_dir();
    let root = tmp.path();
    write(root, "docs/sprints/aktiv/SP11-pi/README.md", SPRINT);
    write(root, "docs/sprints/aktiv/SP11-pi/a.md", "# SP11.1 · Image");
    write(root, "docs/sprints/geplant/F1-x/README.md", DRAFT);
    write(root, "docs/sprints/erledigt/M1-a/README.md", "# M1 · DEV · Alt");
    write(
        root,
        "docs/backlog/B-090-radar.md",
        "# B-090 · Radar\n\n- **Domäne:** CLI\n- **Status:** eingeplant\n- **Sprint:** U1\n",
    );
    write(root, "docs/backlog/README.md", "kein Ticket");
    let d = load(root).expect("load");
    assert_eq!((d.sprints.len(), d.tickets.len(), d.done), (3, 1, 1));
    assert_eq!(d.tickets[0].title, "Radar");
    let sp = d.sprints.iter().find(|s| s.id == "SP11").expect("SP11");
    assert_eq!(
        sp.sessions[0].text, "# SP11.1 · Image",
        "Session-Text aus der Spalte Datei"
    );
    assert_eq!(sp.sessions[1].text, "");
    assert!(load(tmp_dir().path()).is_err(), "falsche Wurzel muss scheitern");
    assert!(doc(root, "../x").is_err(), "Dokumentname nur aus der Liste");
}

/// Die echten Docs des Repos, nur lesend: das Format darf nicht still aus dem Parser fallen. Anders als Go ohne
/// Erwartung an einzelne Sprints oder Tickets.
#[test]
fn repo_wird_gelesen() {
    let Some(root) = real_root() else {
        return;
    };
    let d = load(&root).expect("load");
    assert!(d.sprints.iter().any(|s| !s.title.is_empty() && !s.sessions.is_empty()));
    assert!(
        d.tickets
            .iter()
            .any(|t| !t.nr.is_empty() && !t.title.is_empty() && !t.status.is_empty())
    );
    assert!(!doc(&root, "plan").expect("Plan").is_empty());
}

#[test]
fn worktrees_gehoeren_zu_sprints() {
    let branches = parse_worktrees(
        "worktree C:/k3c\nHEAD 1\nbranch refs/heads/main\n\nworktree C:/w1\r\nbranch refs/heads/sprint/s4\r\n\n\
         worktree C:/w2\nbranch refs/heads/gr4-4-work\n\nworktree C:/w3\ndetached\n\n\
         worktree C:/w4\nbranch refs/heads/claude/s4-review-ab12\n\nworktree C:/w5\nbranch refs/heads/claude/sprint-s4-cd34\n",
    );
    assert_eq!(branches.len(), 5, "{branches:?}");
    let mut sprints: Vec<Sprint> = ["S4", "GR4", "S40", "F1"]
        .iter()
        .map(|id| Sprint {
            id: (*id).to_owned(),
            ..Sprint::default()
        })
        .collect();
    mark_worktrees(&mut sprints, &branches);
    assert_eq!(
        (sprints[0].worktree.as_str(), sprints[0].worktrees.len()),
        ("sprint/s4", 3)
    );
    assert_eq!(sprints[1].worktree, "gr4-4-work");
    assert_eq!((sprints[2].worktree.as_str(), sprints[3].worktree.as_str()), ("", ""));
}

#[test]
fn dokumente_nur_mit_datei() {
    let tmp = tmp_dir();
    let root = tmp.path();
    assert!(available(root).is_empty(), "ohne Dateien keine Dokumente");
    write(root, "docs/glossar.md", "# glossar.md");
    write(root, "docs/plan-weiterentwicklung.md", "# plan");
    assert_eq!(available(root), ["plan", "glossar"]);
    assert_eq!(doc(root, "glossar").expect("Glossar"), "# glossar.md");
}

/// Feldnamen und weggelassene leere Felder wie die JSON-Tags in Go (`frontend/src/api/types.ts`).
#[test]
fn json_form_wie_go() {
    let sprint = parse_sprint(DRAFT, "F1-x", "geplant");
    assert_eq!(
        serde_json::to_value(&sprint).expect("JSON"),
        json!({"id": "F1", "title": "Zielkorridore", "domain": "REG", "project": "", "status": "geplant",
            "reife": "Entwurf", "spec": "", "tickets": [], "sessions": [{"nr": "F1.1", "typ": "", "agent": "",
            "status": "entwurf", "titel": "🧑 Workshop Zielkorridore (AC-01)"}]})
    );
    let ticket = parse_ticket("# B-002 · T\n\n- **Duplikat von:** –\n");
    assert_eq!(
        serde_json::to_value(&ticket).expect("JSON"),
        json!({"nr": "B-002", "title": "T", "domain": "", "typ": "", "prio": "", "env": "", "status": "",
            "sprint": "", "project": "", "spec": ""})
    );
}
