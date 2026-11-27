//! Domäne aus den Sessions: die Funktion selbst mit gestellter Fahrplan-Pflege und die Wege von Go `domains_test.go`
//! über `set`. Die Schritte mit `create` und `delete` stehen in `create_cases_tests.rs`.

use super::domains::sync_sprint_domain;
use super::edit::set;
use super::fixtures::{self, doc, repo, session, snapshot, sprint, values};
use super::store::ChangeSet;
use super::testsupport::{temp_repo, write};

fn sprint_with_sessions(id: &str, domains: &[&str]) -> tempfile::TempDir {
    let tmp = repo();
    let root = tmp.path();
    sprint(root, id, "–");
    for (n, domain) in domains.iter().enumerate() {
        session(root, &format!("{id}.{}", n + 1), domain, true);
    }
    tmp
}

#[test]
fn sprint_domaene_folgt_set() {
    let tmp = sprint_with_sessions("X5", &["SIM", "SRV", "SIM"]);
    let root = tmp.path();
    set(root, "X5.1", &values(&[("Domäne", "CLI")])).expect("Set");
    let readme = doc(root, "sprints/geplant/X5-s/README.md");
    assert!(readme.starts_with("# X5 · CLI, SRV, SIM · "), "{readme}");
    assert!(readme.contains("- **Domäne:** CLI, SRV, SIM\n"), "{readme}");
    let roadmap = doc(root, "sprints/README.md");
    assert!(roadmap.contains("| X5 | CLI, SRV, SIM | Sprint X5 |"), "{roadmap}");
}

#[test]
fn session_domaene_und_verworfen() {
    let tmp = sprint_with_sessions("X6", &["SRV"]);
    let root = tmp.path();
    let before = fixtures::snapshot(root);
    for (id, value) in [("X6.1", "XYZ"), ("X6", "SIM")] {
        set(root, id, &values(&[("Domäne", value)])).expect_err(id);
        assert!(snapshot(root) == before, "{id}: Dateien geändert");
    }
    let readme = "docs/sprints/geplant/X6-s/README.md";
    let original = super::testsupport::read(root, readme);
    write(root, readme, &original.replacen("# X6 ·", "X6 ·", 1));
    let broken = snapshot(root);
    set(root, "X6.1", &values(&[("Domäne", "SIM")])).expect_err("README ohne Überschrift");
    assert!(snapshot(root) == broken, "Dateien geändert");
    write(root, readme, &original);
    set(root, "X6.1", &values(&[("Status", "verworfen")])).expect("verworfen");
    let text = doc(root, "sprints/geplant/X6-s/README.md");
    assert!(
        text.contains("| X6.1 | `X6.1-s.md` | Umsetzung | autonom | verworfen |"),
        "{text}"
    );
}

const DIR: &str = "sprints/geplant/X5-s";

fn setup(readme_head: &str) -> tempfile::TempDir {
    let tmp = temp_repo();
    let root = tmp.path();
    let rows: String = (1..=4)
        .map(|n| format!("| X5.{n} | `X5.{n}-s.md` | Umsetzung | autonom | offen |\n"))
        .collect();
    let readme = format!(
        "{readme_head}\n\n- **Domäne:** DEV\n\n## Sessions\n\n| Nr. | Datei | Typ | Agent | Status |\n\
         |---|---|---|---|---|\n{rows}"
    );
    write(root, &format!("docs/{DIR}/README.md"), &readme);
    for (n, domain) in [(1, "SIM"), (2, "SRV"), (3, "SIM"), (4, "XYZ")] {
        let text = format!("# X5.{n} · S\n\n- **Domäne:** {domain}\n");
        write(root, &format!("docs/{DIR}/X5.{n}-s.md"), &text);
    }
    tmp
}

#[test]
fn domaene_in_reihenfolge_des_auftretens() {
    let tmp = setup("# X5 · DEV · Sprint X5");
    let mut changes = ChangeSet::new(tmp.path());
    let mut synced = Vec::new();
    sync_sprint_domain(&mut changes, DIR, "geplant", |_, sprint, head, old, new| {
        synced.push((
            sprint.domain.clone(),
            head["Domäne"].clone(),
            old.to_owned(),
            new.to_owned(),
        ));
        Ok(())
    })
    .expect("sync");
    let readme = &changes.writes[&format!("{DIR}/README.md")];
    assert!(
        readme.starts_with("# X5 · SIM, SRV · Sprint X5\n\n- **Domäne:** SIM, SRV\n"),
        "{readme}"
    );
    assert_eq!(changes.notes, ["X5: Domäne SIM, SRV (aus den Sessions)"]);
    assert_eq!(
        synced,
        [(
            "SIM, SRV".to_owned(),
            "SIM, SRV".to_owned(),
            DIR.to_owned(),
            DIR.to_owned()
        )]
    );
    // Noch einmal: alles passt, nichts passiert.
    sync_sprint_domain(&mut changes, DIR, "geplant", |_, _, _, _, _| {
        Err("nicht erwartet".to_owned())
    })
    .expect("gleich");
    assert_eq!(changes.notes.len(), 1);
}

#[test]
fn ohne_ueberschrift_ein_fehler() {
    let tmp = setup("X5 · DEV · Sprint X5");
    let mut changes = ChangeSet::new(tmp.path());
    let error = sync_sprint_domain(&mut changes, DIR, "geplant", |_, _, _, _, _| Ok(())).expect_err("Überschrift");
    assert_eq!(error, format!("{DIR}/README.md: Überschrift `# X5 · … · …` fehlt"));
    assert!(changes.writes.is_empty());
}
