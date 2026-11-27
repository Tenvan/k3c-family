//! Domäne aus den Sessions. Go `domains_test.go` prüft sie über `Create`, `Set` und `Delete` (RU69.9, RU69.10); hier
//! nur die Funktion selbst mit gestellter Fahrplan-Pflege.

use super::domains::sync_sprint_domain;
use super::store::ChangeSet;
use super::testsupport::{temp_repo, write};

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
