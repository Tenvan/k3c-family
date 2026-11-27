//! Go `wtdiff_test.go`. Die git-Fälle laufen nur auf Wegwerf-Repos unter `.work/`, git abgeschottet wie in `gitops`.

use std::fs;
use std::path::{Path, PathBuf};

use super::planning::{Data, Session, Sprint, Ticket};
use super::projects::Project;
use super::store::STATES;
use super::testhooks::isolate_git;
use super::testsupport::{INDEX_MD, write};
use super::wtdiff::{Change, check_worktrees, diff_data, parse_worktree_paths};
use crate::gitops::fixture_tests::Fixture;

fn project(id: &str, title: &str, status: &str, rang: &str) -> Project {
    Project {
        id: id.to_owned(),
        title: title.to_owned(),
        status: status.to_owned(),
        rang: rang.to_owned(),
        ..Project::default()
    }
}

fn session(nr: &str, status: &str, titel: &str) -> Session {
    Session {
        nr: nr.to_owned(),
        status: status.to_owned(),
        titel: titel.to_owned(),
        ..Session::default()
    }
}

fn sprint(id: &str, title: &str, status: &str, project: &str, sessions: Vec<Session>) -> Sprint {
    Sprint {
        id: id.to_owned(),
        title: title.to_owned(),
        status: status.to_owned(),
        project: project.to_owned(),
        sessions,
        ..Sprint::default()
    }
}

fn ticket(nr: &str, title: &str, status: &str, prio: &str, sprint: &str) -> Ticket {
    Ticket {
        nr: nr.to_owned(),
        title: title.to_owned(),
        status: status.to_owned(),
        prio: prio.to_owned(),
        sprint: sprint.to_owned(),
        ..Ticket::default()
    }
}

fn data(projects: Vec<Project>, sprints: Vec<Sprint>, tickets: Vec<Ticket>) -> Data {
    Data {
        projects,
        sprints,
        tickets,
        ..Data::default()
    }
}

/// `Item ID` → `Art: Text`, sortiert.
fn lines(changes: &[Change]) -> Vec<String> {
    let mut out: Vec<String> = changes
        .iter()
        .map(|c| format!("{} {} {}: {}", c.item, c.id, c.kind, c.text))
        .collect();
    out.sort();
    out
}

/// Art, Objekt, ID, Text, Projekt, Sprint.
fn rows(changes: &[Change]) -> Vec<[&str; 6]> {
    changes
        .iter()
        .map(|c| [&c.kind, &c.item, &c.id, &c.text, &c.project, &c.sprint].map(String::as_str))
        .collect()
}

#[test]
fn diff_data_nennt_neu_und_geaendert() {
    let base = data(
        vec![project("TST", "", "aktiv", "8")],
        vec![sprint("TR4", "", "geplant", "", vec![session("TR4.1", "offen", "")])],
        vec![ticket("B-1", "", "offen", "hoch", "")],
    );
    let sessions = vec![session("TR4.1", "fertig", ""), session("TR4.2", "", "neu")];
    let wt = data(
        vec![project("TST", "", "ruht", "8")],
        vec![
            sprint("TR4", "", "geplant", "", sessions),
            sprint("TR5", "Dummy", "", "", Vec::new()),
        ],
        vec![ticket("B-1", "", "offen", "hoch", ""), ticket("B-2", "x", "", "", "")],
    );
    assert_eq!(
        lines(&diff_data(&base, &wt)),
        [
            "Projekt TST geändert: Status aktiv → ruht",
            "Session TR4.1 geändert: Status offen → fertig",
            "Session TR4.2 neu: neu",
            "Sprint TR5 neu: Dummy",
            "Ticket B-2 neu: x",
        ]
    );
    assert!(
        diff_data(&base, &base).is_empty(),
        "gleiche Stände ergeben keine Unterschiede"
    );
}

#[test]
fn worktree_liste_ohne_die_wurzel() {
    let out = "worktree C:/k3c\nHEAD 1\nbranch refs/heads/main\n\nworktree C:/k3c/wt\r\nbranch refs/heads/claude/x\r\n";
    let found = parse_worktree_paths(out, Path::new("C:/k3c"));
    assert_eq!(found.len(), 1, "{found:?}");
    assert_eq!(found[0].branch, "claude/x");
}

/// Wegwerf-Repo mit einem Ticket und einem Worktree `wt` auf dem Branch `branch`.
struct Repo {
    fixture: Fixture,
    root: PathBuf,
    wt: PathBuf,
}

impl Repo {
    async fn git(&self, args: &[&str]) -> String {
        self.fixture.sh(&self.root, args).await
    }

    async fn new(branch: &str) -> Repo {
        let fixture = Fixture::empty();
        isolate_git(fixture.env.clone());
        let root = fixture.tmp.path().join("work");
        let wt = fixture.tmp.path().join("wt");
        write(&root, "docs/backlog/README.md", INDEX_MD);
        write(
            &root,
            "docs/backlog/B-001-alt.md",
            "# B-001 · Altes Ticket\n\n- **Status:** offen\n",
        );
        for state in STATES {
            fs::create_dir_all(root.join("docs").join("sprints").join(state)).expect("Sprint-Ordner");
        }
        let repo = Repo { fixture, root, wt };
        repo.git(&["init", "-q"]).await;
        // Sicherung: alles Folgende trifft wirklich das Wegwerf-Repo.
        let top = repo.git(&["rev-parse", "--show-toplevel"]).await;
        assert!(top.ends_with("/work"), "{top}");
        repo.git(&["add", "-A"]).await;
        repo.git(&["commit", "-q", "-m", "init"]).await;
        let target = repo.wt.display().to_string();
        repo.git(&["worktree", "add", "-q", "-b", branch, &target]).await;
        // git legt leere Ordner nicht an.
        for state in STATES {
            fs::create_dir_all(repo.wt.join("docs").join("sprints").join(state)).expect("Sprint-Ordner");
        }
        repo
    }

    /// Die Änderungen des einen Worktrees.
    fn changes(&self) -> Vec<Change> {
        let mut diffs = check_worktrees(&self.root).expect("check");
        assert_eq!(diffs.len(), 1, "{diffs:?}");
        let diff = diffs.remove(0);
        assert_eq!(diff.error, "");
        diff.changes
    }
}

/// Gegen die Merge-Base: Der Vorsprung der Wurzel (B-006) erscheint nicht, nur was der Worktree ändert (B-007).
#[tokio::test]
async fn check_worktrees_gegen_merge_base() {
    let repo = Repo::new("claude/x").await;
    write(
        &repo.root,
        "docs/backlog/B-006-wurzel.md",
        "# B-006 · In der Wurzel\n\n- **Status:** offen\n",
    );
    repo.git(&["add", "-A"]).await;
    repo.git(&["commit", "-q", "-m", "wurzel weiter"]).await;
    write(
        &repo.wt,
        "docs/backlog/B-007-worktree.md",
        "# B-007 · Im Worktree\n\n- **Status:** offen\n",
    );
    let changes = repo.changes();
    assert_eq!(changes.len(), 1, "{changes:?}");
    assert_eq!((changes[0].id.as_str(), changes[0].kind.as_str()), ("B-007", "neu"));
    // Der Wegwerf-Ordner der entpackten Merge-Base ist wieder weg.
    let left = fs::read_dir(repo.root.join(".work").join("tmp")).expect(".work/tmp");
    assert_eq!(left.count(), 0);
}

/// Ein im Worktree archiviertes Ticket erscheint als Statuswechsel, nicht als Nichts.
#[tokio::test]
async fn check_worktrees_archiviertes_ticket() {
    let repo = Repo::new("claude/y").await;
    fs::remove_file(repo.wt.join("docs/backlog/B-001-alt.md")).expect("löschen");
    write(
        &repo.wt,
        "docs/backlog/archiv/B-001-alt.md",
        "# B-001 · Altes Ticket\n\n- **Status:** erledigt\n",
    );
    let changes = repo.changes();
    assert_eq!(changes.len(), 1, "{changes:?}");
    assert_eq!(
        (changes[0].id.as_str(), changes[0].text.as_str()),
        ("B-001", "Status offen → erledigt")
    );
}

#[test]
fn diff_data_entfernt() {
    let base = data(
        vec![project("TST", "", "", "")],
        vec![
            sprint("TR4", "", "", "", vec![session("TR4.1", "", "")]),
            sprint("TR9", "", "", "", Vec::new()),
        ],
        vec![ticket("B-1", "", "", "", "")],
    );
    let wt = data(Vec::new(), vec![sprint("TR4", "", "", "", Vec::new())], Vec::new());
    assert_eq!(
        lines(&diff_data(&base, &wt)),
        [
            "Projekt TST entfernt: ",
            "Session TR4.1 entfernt: ",
            "Sprint TR9 entfernt: ",
            "Ticket B-1 entfernt: ",
        ]
    );
}

/// Scheitert git, kommt ein Fehler zurück statt einer leeren Liste.
#[test]
fn check_worktrees_git_fehler() {
    let fixture = Fixture::empty();
    isolate_git(fixture.env.clone());
    let missing = fixture.tmp.path().join("gibt-es-nicht");
    let error = check_worktrees(&missing).expect_err("Fehler erwartet");
    assert!(error.starts_with("git worktree list: exit status"), "{error}");
}

/// Je Art erst der Worktree (neu oder Feldänderungen in fester Reihenfolge, beim Sprint danach die Sessions), dann
/// das Entfernte; Arten in der Folge Projekt, Sprint, Ticket (B-465).
#[test]
fn diff_data_reihenfolge() {
    let base = data(
        vec![project("WZG", "Werkzeuge", "ruht", "2"), project("ALT", "Alt", "", "")],
        vec![sprint(
            "WZ6",
            "Studio",
            "geplant",
            "–",
            vec![session("WZ6.1", "offen", "Glossar")],
        )],
        vec![
            ticket("B-1", "Eins", "offen", "mittel", "–"),
            ticket("B-2", "Zwei", "", "", ""),
        ],
    );
    let wt = data(
        vec![project("NEU", "Neu", "", ""), project("WZG", "Werkzeuge", "aktiv", "1")],
        vec![sprint(
            "WZ6",
            "Studio",
            "aktiv",
            "WZG",
            vec![session("WZ6.1", "fertig", "Glossar")],
        )],
        vec![ticket("B-1", "Eins", "eingeplant", "hoch", "WZ6")],
    );
    assert_eq!(
        rows(&diff_data(&base, &wt)),
        [
            ["neu", "Projekt", "NEU", "Neu", "", ""],
            ["geändert", "Projekt", "WZG", "Status ruht → aktiv", "", ""],
            ["geändert", "Projekt", "WZG", "Rang 2 → 1", "", ""],
            ["entfernt", "Projekt", "ALT", "Alt", "", ""],
            ["geändert", "Sprint", "WZ6", "Status geplant → aktiv", "", ""],
            ["geändert", "Sprint", "WZ6", "Projekt – → WZG", "", ""],
            ["geändert", "Session", "WZ6.1", "Status offen → fertig", "", ""],
            ["geändert", "Ticket", "B-1", "Status offen → eingeplant", "", ""],
            ["geändert", "Ticket", "B-1", "Prio mittel → hoch", "", ""],
            ["geändert", "Ticket", "B-1", "Sprint – → WZ6", "", ""],
            ["entfernt", "Ticket", "B-2", "Zwei", "", ""],
        ]
    );
}

#[test]
fn diff_data_neue_eintraege_tragen_projekt_und_sprint() {
    let mut new = ticket("B-9", "Neu", "", "", "WZ20");
    new.project = "WZG".to_owned();
    let wt = data(
        Vec::new(),
        vec![sprint("WZ20", "Dummy", "", "WZG", Vec::new())],
        vec![new],
    );
    assert_eq!(
        rows(&diff_data(&Data::default(), &wt)),
        [
            ["neu", "Sprint", "WZ20", "Dummy", "WZG", ""],
            ["neu", "Ticket", "B-9", "Neu", "WZG", "WZ20"],
        ]
    );
}

/// Ein Sprint nur im Worktree bringt seine Sessions als neu mit, direkt hinter dem Sprint (B-484).
#[test]
fn diff_data_neuer_sprint_mit_sessions() {
    let sessions = vec![session("X1.1", "", "Eins"), session("X1.2", "", "Zwei")];
    let wt = data(Vec::new(), vec![sprint("X1", "Neu", "", "WZG", sessions)], Vec::new());
    assert_eq!(
        rows(&diff_data(&Data::default(), &wt)),
        [
            ["neu", "Sprint", "X1", "Neu", "WZG", ""],
            ["neu", "Session", "X1.1", "Eins", "", ""],
            ["neu", "Session", "X1.2", "Zwei", "", ""],
        ]
    );
}
