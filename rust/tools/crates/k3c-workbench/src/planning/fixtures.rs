//! Wurzeln für die Schreibtests unter `.work/` (Go `tempRepo`, `relRepo`, `newProject`, `newSprint`). Go baut sie
//! über `Create`; bis RU69.10 entstehen die Dateien hier von Hand. Die Vorlagen sind Kopien von docs/vorlagen/.

use std::collections::BTreeMap;
use std::fs;
use std::path::Path;

use super::index::{ARCHIVE_HEAD, TABLE_HEAD};
use super::store::{STATES, docs_path, read_text};
use super::testsupport::write;
use crate::testutil::tmp_dir;

pub(super) const ROADMAP_MD: &str = "# Fahrplan\n\n## Aktiv\n\n| Sprint | Domäne | Thema | Am Ende sichtbar | Ordner |\n\
|---|---|---|---|---|\n\n## Offen am Gerät\n\n| Session | Gerät | Kriterium | Ordner |\n|---|---|---|---|\n\
| X1.3 | Xbox | AC-04 | `erledigt/X1-test/` |\n\n## Geplant (in dieser Reihenfolge)\n\n\
| Sprint | Domäne | Thema | Am Ende sichtbar | Reife | Ordner |\n|---|---|---|---|---|---|\n\n## Erledigt\n\n\
| Sprint | Thema | Ordner |\n|---|---|---|\n";

pub(super) const ROADMAP_PROJEKT_MD: &str = "# Fahrplan\n\n## Aktiv\n\n\
| Sprint | Domäne | Projekt | Thema | Am Ende sichtbar | Ordner |\n|---|---|---|---|---|---|\n\n\
## Geplant (nach Projekt-Rang)\n\n| Sprint | Domäne | Projekt | Thema | Am Ende sichtbar | Reife | Ordner |\n\
|---|---|---|---|---|---|---|\n\n## Erledigt\n\n| Sprint | Thema | Ordner |\n|---|---|---|\n";

const PROJEKTE_MD: &str = "# Projekte\n\nRangliste aller Projekte.\n\n## Aktiv\n\n| Rang | Projekt | Ziel | Datei |\n\
|---|---|---|---|\n\nNoch keine Projekte: Der Umzug der Planung folgt in PJ3 (B-359).\n\n## Ruht\n\n\
| Projekt | Grund | Datei |\n|---|---|---|\n\n## Erledigt\n\n| Projekt | Datei |\n|---|---|\n";

/// `docs/` mit den Vorlagen, einem Ticket, Index, Fahrplan und Projekt-Übersicht.
pub(super) fn repo() -> tempfile::TempDir {
    let tmp = tmp_dir();
    let root = tmp.path();
    let templates = Path::new(env!("CARGO_MANIFEST_DIR"))
        .ancestors()
        .nth(4)
        .expect("Repo-Wurzel")
        .join("docs")
        .join("vorlagen");
    for kind in ["ticket", "sprint", "session", "projekt"] {
        let text = fs::read_to_string(templates.join(format!("{kind}.md"))).expect("Vorlage lesen");
        write(root, &format!("docs/vorlagen/{kind}.md"), &text);
    }
    let index = format!(
        "# Backlog\n\n## Offen\n\n{TABLE_HEAD}{}\n## Archiv\n\nText\n\n{TABLE_HEAD}",
        row("B-001")
    );
    write(root, "docs/backlog/README.md", &index);
    write(
        root,
        "docs/backlog/B-001-alt.md",
        "# B-001 · Altes Ticket\n\n- **Status:** offen\n",
    );
    write(root, "docs/sprints/README.md", ROADMAP_MD);
    write(root, "docs/projekte/README.md", PROJEKTE_MD);
    for state in STATES {
        fs::create_dir_all(root.join("docs").join("sprints").join(state)).expect("Sprint-Ordner");
    }
    tmp
}

fn row(nr: &str) -> String {
    format!("| [{nr}]({nr}-alt.md) | SRV | Idee | mittel | offen | – | Altes Ticket |\n")
}

/// Ticket-Datei mit dem vollen Kopf der Vorlage.
pub(super) fn full_ticket(root: &Path, nr: &str, slug: &str, title: &str) {
    let text = format!(
        "# {nr} · {title}\n\n- **Domäne:** DEV\n- **Typ:** Idee\n- **Prio:** mittel\n- **Umgebung:** ?\n\
         - **Status:** offen\n- **Sprint:** –\n- **Projekt:** –\n- **Erstellt:** 2026-01-01\n- **Spec:** Entwurf\n\
         - **Revision:** 1\n- **Freigabe:** –\n- **Duplikat von:** –\n- **Ersetzt:** –\n\n## Ausgangslage\n\nText\n"
    );
    write(root, &format!("docs/backlog/{nr}-{slug}.md"), &text);
}

/// B-001 (alter Kopf) und B-002 bis B-004 mit beiden Index-Dateien (Go `relRepo` nach `RebuildIndex`).
pub(super) fn rel_repo() -> tempfile::TempDir {
    let tmp = repo();
    let root = tmp.path();
    let old = "# B-001 · Altes Ticket\n\n- **Domäne:** SRV\n- **Typ:** Idee\n- **Prio:** mittel\n- **Status:** offen\n\
               - **Sprint:** –\n";
    write(root, "docs/backlog/B-001-alt.md", old);
    let mut index = format!("# Backlog\n\n## Offen\n\n{TABLE_HEAD}{}", row("B-001"));
    for (nr, title) in [
        ("B-002", "Zweites Ticket"),
        ("B-003", "Drittes Ticket"),
        ("B-004", "Viertes Ticket"),
    ] {
        full_ticket(root, nr, "t", title);
        index.push_str(&format!(
            "| [{nr}]({nr}-t.md) | DEV | Idee | mittel | offen | – | {title} |\n"
        ));
    }
    write(root, "docs/backlog/README.md", &index);
    write(
        root,
        "docs/backlog/archiv/README.md",
        &format!("{ARCHIVE_HEAD}\n## Archiv\n\n{TABLE_HEAD}"),
    );
    tmp
}

/// Projekt-Datei; `sprints` sind die IDs ihrer Sprint-Tabelle.
pub(super) fn project(root: &Path, id: &str, rang: &str, sprints: &[&str]) {
    let rows: String = sprints
        .iter()
        .map(|s| format!("| {s} | Sprint {s} | geplant |\n"))
        .collect();
    let text = format!(
        "# {id} · Titel {id}\n\n- **Status:** aktiv\n- **Rang:** {rang}\n- **Ziel-Tickets:** –\n\n## Ziel\n\nText\n\n\
         ## Sprints\n\n| Sprint | Thema | Status |\n|---|---|---|\n{rows}\n## Notizen\n\n–\n"
    );
    write(root, &format!("docs/projekte/{id}-{}.md", id.to_lowercase()), &text);
}

/// Sprint unter `geplant/<id>-s/` samt Zeile am Ende der Fahrplan-Tabelle „Geplant“.
pub(super) fn sprint(root: &Path, id: &str, project: &str) {
    let text = format!(
        "# {id} · SRV · Sprint {id}\n\n- **Status:** geplant\n- **Projekt:** {project}\n- **Domäne:** SRV\n\
         - **Reife:** Entwurf\n- **Tickets:** B-001\n- **Start-Commit:** –\n- **Spec:** Entwurf\n- **Revision:** 1\n\
         - **Freigabe:** –\n\n## Ziel\n\nText\n\n## Sessions\n\n| Nr. | Datei | Typ | Agent | Status |\n\
         |---|---|---|---|---|\n"
    );
    write(root, &format!("docs/sprints/geplant/{id}-s/README.md"), &text);
    let mut lines: Vec<String> = doc(root, "sprints/README.md").split('\n').map(str::to_owned).collect();
    let start = lines
        .iter()
        .position(|l| l.starts_with("## Geplant"))
        .expect("Tabelle Geplant");
    let mut end = start + 2;
    let with_project = lines[end].contains("Projekt");
    while end < lines.len() && lines[end].starts_with('|') {
        end += 1;
    }
    let column = if with_project {
        format!(" {project} |")
    } else {
        String::new()
    };
    lines.insert(
        end,
        format!("| {id} | SRV |{column} Sprint {id} | – | Entwurf | `geplant/{id}-s/` |"),
    );
    write(root, "docs/sprints/README.md", &lines.join("\n"));
}

/// Session-Datei im geplanten Sprint `<präfix>-s`; die Zeile in der Session-Tabelle setzt der Test selbst oder `row`.
pub(super) fn session(root: &Path, id: &str, domain: &str, row: bool) {
    let sprint = id.split('.').next().expect("Sprint-Präfix");
    let text = format!(
        "# {id} · Session {id}\n\n- **Status:** offen\n- **Typ:** Umsetzung\n- **Agent:** autonom\n\
         - **Domäne:** {domain}\n- **Lane:** –\n- **Modell:** –\n- **Umgebung:** offline\n- **Branch:** x/1\n\
         - **Abhängig von:** –\n- **Tickets:** B-001\n- **Kriterien:** AC-01\n\n## Ziel\n\nText\n"
    );
    write(root, &format!("docs/sprints/geplant/{sprint}-s/{id}-s.md"), &text);
    if row {
        let readme = format!("sprints/geplant/{sprint}-s/README.md");
        let table = doc(root, &readme) + &format!("| {id} | `{id}-s.md` | Umsetzung | autonom | offen |\n");
        write(root, &format!("docs/{readme}"), &table);
    }
}

/// Inhalt von `docs/<rel>` mit `\n`.
pub(super) fn doc(root: &Path, rel: &str) -> String {
    read_text(root, rel).expect("Datei lesen").0
}

pub(super) fn values(pairs: &[(&str, &str)]) -> BTreeMap<String, String> {
    pairs.iter().map(|(k, v)| ((*k).to_owned(), (*v).to_owned())).collect()
}

/// Alle Dateien unter der Wurzel mit ihren Bytes.
pub(super) fn snapshot(root: &Path) -> BTreeMap<String, Vec<u8>> {
    let mut out = BTreeMap::new();
    let mut open = vec![docs_path(root, "")];
    while let Some(dir) = open.pop() {
        for entry in fs::read_dir(&dir).expect("Ordner lesen") {
            let path = entry.expect("Eintrag").path();
            if path.is_dir() {
                open.push(path);
            } else {
                let bytes = fs::read(&path).expect("Datei lesen");
                out.insert(path.to_string_lossy().into_owned(), bytes);
            }
        }
    }
    out
}
