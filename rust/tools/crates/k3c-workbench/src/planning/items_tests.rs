//! Go `items_test.go`.

use serde::Serialize;

use super::items::{MAX_ITEMS, items, load_list};
use super::planning::{Data, load};
use super::testsupport::{real_root, write};
use crate::testutil::tmp_dir;

/// B-909/AC-01: Listen-Daten unter 1,5 MiB, eine ID unter 500 KiB.
const LIST_LIMIT: usize = 1536 << 10;
const ITEM_LIMIT: usize = 500 << 10;

const README: &str = "# X1 · DEV · A\n\n## Sessions\n\n| Nr. | Datei | Typ | Agent | Status |\n|---|---|---|---|---|\n\
| X1.1 | X1.1-s.md | Umsetzung | autonom | offen |\n";

fn json_size(value: &impl Serialize) -> usize {
    serde_json::to_vec(value).expect("JSON").len()
}

/// Zahl der Sessions und ihre rohen Texte in Bytes.
fn text_stats(data: &Data) -> (usize, usize) {
    let sessions = data.sprints.iter().flat_map(|s| &s.sessions);
    sessions.fold((0, 0), |(n, bytes), x| (n + 1, bytes + x.text.len()))
}

/// Die Sprints, die 40 längsten Sessions, alle Tickets und Projekte: Kandidaten für das größte einzelne Dokument.
fn candidate_ids(full: &Data) -> Vec<String> {
    let mut sessions: Vec<_> = full.sprints.iter().flat_map(|s| &s.sessions).collect();
    sessions.sort_by_key(|x| std::cmp::Reverse(x.text.len()));
    let mut ids: Vec<String> = full.sprints.iter().map(|s| s.id.clone()).collect();
    ids.extend(sessions.iter().take(40).map(|x| x.nr.clone()));
    ids.extend(full.tickets.iter().map(|t| t.nr.clone()));
    ids.extend(full.projects.iter().map(|p| p.id.clone()));
    ids
}

/// Misst `load` und `load_list` gegen die echte Planung (nur lesend). Wie Go nicht im kurzen Lauf: die Grenzen hängen am
/// Stand der Planung. Aufruf: `cargo test -p k3c-workbench planungsdaten -- --ignored --nocapture`.
#[test]
#[ignore = "liest die ganze echte Planung, Grenzen hängen an ihrem Stand"]
fn planungsdaten_groesse() {
    let Some(root) = real_root() else {
        return;
    };
    let full = load(&root).expect("load");
    let slim = load_list(&root).expect("load_list");
    let (sessions, texts) = text_stats(&full);
    assert_eq!(text_stats(&slim), (sessions, 0));
    let size = json_size(&slim);
    println!(
        "voll {} KiB, Liste {} KiB, Texte {} KiB bei {sessions} Sessions",
        json_size(&full) >> 10,
        size >> 10,
        texts >> 10
    );
    assert!(size < LIST_LIMIT, "Listen-Daten {} KiB", size >> 10);
    for chunk in candidate_ids(&full).chunks(MAX_ITEMS) {
        for item in items(&root, chunk)
            .expect("items")
            .iter()
            .filter(|i| i.error.is_empty())
        {
            assert!(
                json_size(item) < ITEM_LIMIT,
                "{}: {} KiB",
                item.id,
                json_size(item) >> 10
            );
        }
    }
}

#[test]
fn items_je_id() {
    let tmp = tmp_dir();
    let root = tmp.path();
    write(root, "docs/sprints/aktiv/X1-a/README.md", README);
    write(
        root,
        "docs/sprints/aktiv/X1-a/X1.1-s.md",
        "# X1.1 · Session\r\n\r\n- **Status:** offen\r\n",
    );
    write(root, "docs/sprints/geplant/Y1-b/README.md", "# Y1 · DEV · B\n");
    write(
        root,
        "docs/backlog/B-001-a.md",
        "# B-001 · Ticket\n\n- **Status:** offen\n",
    );
    let ids: Vec<String> = ["X1.1", "B-001", "X1", "B-001", "nix", "B-777"]
        .map(str::to_owned)
        .into();
    let found = items(root, &ids).expect("items");
    assert_eq!(found.len(), 5, "Doppelte zählen einmal: {found:?}");
    assert_eq!(found[0].kind, "session");
    assert_eq!(found[0].path, "docs/sprints/aktiv/X1-a/X1.1-s.md");
    assert_eq!(found[0].text, "# X1.1 · Session\n\n- **Status:** offen\n");
    assert_eq!(
        (found[1].kind.as_str(), found[1].path.as_str()),
        ("ticket", "docs/backlog/B-001-a.md")
    );
    assert_eq!(
        (found[2].kind.as_str(), found[2].path.as_str()),
        ("sprint", "docs/sprints/aktiv/X1-a/README.md")
    );
    assert_eq!(
        found[3].error,
        "ungültige ID \"nix\" (B-123, M8, M8.1 oder Projekt GRA)"
    );
    assert_eq!(found[4].error, "B-777 nicht gefunden");
    assert!(
        items(root, &vec![String::new(); MAX_ITEMS + 1]).is_err(),
        "zu viele IDs"
    );
}

#[test]
fn liste_ohne_text() {
    let tmp = tmp_dir();
    let root = tmp.path();
    write(root, "docs/sprints/aktiv/X1-a/README.md", README);
    write(
        root,
        "docs/sprints/aktiv/X1-a/X1.1-s.md",
        "# X1.1 · Session\n\n- **Umgebung:** offline\n- **Domäne:** DEV\n",
    );
    write(root, "docs/sprints/geplant/Y1-b/README.md", "# Y1 · DEV · B\n");
    write(root, "docs/backlog/B-001-a.md", "# B-001 · Ticket\n");
    let slim = load_list(root).expect("load_list");
    let x1 = slim.sprints.iter().find(|s| s.id == "X1").expect("X1");
    let session = &x1.sessions[0];
    assert_eq!(session.text, "", "Text weg");
    assert_eq!(
        (session.titel.as_str(), session.env.as_str(), session.domain.as_str()),
        ("Session", "offline", "DEV")
    );
}
