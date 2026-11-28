//! Go `roadmap_refresh_test.go` (B-842).

use super::fixtures::doc;
use super::roadmap_refresh::{ROADMAP_REL, refresh_roadmap};
use super::tables::{split_lines, take_roadmap_row};
use super::testsupport::write;
use crate::testutil::tmp_dir;

const ROADMAP: &str = "# Fahrplan\n\n## Aktiv\n\n| Sprint | Domäne | Projekt | Thema | Am Ende sichtbar | Ordner |\n\
|---|---|---|---|---|---|\n| A1 | DEV | – | Sprint A1 | – | `aktiv/A1-a/` |\n\
| NG15 | CLI | NAT | Ton | – | `aktiv/NG15-ton/` |\n\n\
## Offen am Gerät\n\nAbnahmen sammelt das Projekt ABN im Sprint HW1.\n\n\
## Geplant (nach Projekt-Rang)\n\n| Sprint | Domäne | Projekt | Thema | Am Ende sichtbar | Reife | Ordner |\n\
|---|---|---|---|---|---|---|\n| LG1 | SRV | LST | Logs | – | Entwurf | `geplant/LG1-logs/` |\n\n\
## Erledigt\n\n| Sprint | Thema | Ordner |\n|---|---|---|\n| KB6 | SIM, REG | `erledigt/KB6-kampf/` |\n\
| LG1 | Logs | `erledigt/LG1-logs/` |\n| NG15 | Ton | `erledigt/NG15-ton/` |\n";

/// Fahrplan mit veralteten Zeilen, KB6 mit Domänen als Thema und aktivem Sprint A1 mit Mensch-Sessions.
pub(super) fn refresh_repo() -> tempfile::TempDir {
    let tmp = tmp_dir();
    let root = tmp.path();
    write(root, "docs/backlog/README.md", "# Backlog\n");
    write(root, "docs/sprints/geplant/.keep", "");
    for (name, domain) in [("NG15-ton", "CLI"), ("LG1-logs", "SRV"), ("KB6-kampf", "SIM, REG")] {
        let id = name.split('-').next().expect("ID");
        let readme = format!("# {id} · {domain} · Titel {id}\n\n- **Status:** erledigt\n");
        write(root, &format!("docs/sprints/erledigt/{name}/README.md"), &readme);
    }
    let sessions = [
        ("A1.1", "Abnahme", "Mensch", "offen", "Gerät: PC"),
        ("A1.2", "Abnahme", "Mensch", "fertig", "Gerät: PC"),
        ("A1.3", "Umsetzung", "autonom", "offen", ""),
        ("A1.4", "Abnahme", "Mensch", "offen", "Gerät: Tablet"),
        ("A1.5", "Abnahme", "Mensch", "offen", ""),
    ];
    let mut readme = "# A1 · DEV · Sprint A1\n\n- **Status:** aktiv\n\n## Sessions\n\n\
                      | Nr. | Datei | Typ | Agent | Status |\n|---|---|---|---|---|\n"
        .to_owned();
    for (nr, typ, agent, status, context) in sessions {
        readme.push_str(&format!("| {nr} | `{nr}-x.md` | {typ} | {agent} | {status} |\n"));
        let text = format!("# {nr} · Titel {nr}\n\n- **Status:** offen\n\n## Kontext\n\n{context}\n");
        write(root, &format!("docs/sprints/aktiv/A1-a/{nr}-x.md"), &text);
    }
    write(root, "docs/sprints/aktiv/A1-a/README.md", &readme);
    write(root, "docs/sprints/README.md", ROADMAP);
    tmp
}

#[test]
fn fahrplan_wird_geheilt() {
    let tmp = refresh_repo();
    let root = tmp.path();
    refresh_roadmap(root).expect("refresh");
    let text = doc(root, ROADMAP_REL);
    for wanted in [
        "### PC\n\n- A1.1 · Titel A1.1 (Sprint A1, offen, Agent Mensch)\n\n### Tablet\n\n- A1.4 · Titel A1.4",
        "### unbekannt\n\n- A1.5 · Titel A1.5",
        "| KB6 | Titel KB6 | `erledigt/KB6-kampf/` |",
        "| LG1 | Logs | `erledigt/LG1-logs/` |",
        "| A1 | DEV | – | Sprint A1 | – | `aktiv/A1-a/` |",
    ] {
        assert!(text.contains(wanted), "fehlt {wanted:?}:\n{text}");
    }
    for gone in [
        "A1.2 ·",
        "A1.3 ·",
        "HW1",
        "`aktiv/NG15-ton/`",
        "`geplant/LG1-logs/`",
        "SIM, REG",
    ] {
        assert!(!text.contains(gone), "noch da {gone:?}:\n{text}");
    }
    for nr in ["A1.1 ·", "A1.4 ·", "A1.5 ·"] {
        assert_eq!(text.matches(nr).count(), 1, "{nr}");
    }
    refresh_roadmap(root).expect("zweiter Lauf");
    assert_eq!(doc(root, ROADMAP_REL), text);
}

/// Eine Zeile mit falscher Spaltenzahl liefert keine alten Werte (sonst wird die zweite Zelle zum Thema).
#[test]
fn falsche_spaltenzahl_liefert_keine_werte() {
    let mut lines = split_lines(
        "## Erledigt\n\n| Sprint | Thema | Ordner |\n|---|---|---|\n\
         | KB6 | SIM, REG | KMP | x | – | – | `erledigt/KB6-kampf/` |\n",
    );
    let (old, at) = take_roadmap_row(&mut lines, "KB6", "sprints/erledigt/KB6-kampf");
    assert!(at.is_some() && old.is_empty(), "at={at:?} old={old:?}");
}
