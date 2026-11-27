//! Hilfen der Planungstests: Wurzeln unter `.work/` mit von Hand geschriebenen Dateien (ohne Vorlagen).

use std::path::{Path, PathBuf};

use super::store::STATES;
use crate::testutil::tmp_dir;

pub(super) const INDEX_MD: &str = "# Backlog\n\n## Offen\n\n| Nr. | Domäne | Typ | Prio | Status | Sprint | Titel |\n\
|---|---|---|---|---|---|---|\n| [B-001](B-001-alt.md) | SRV | Idee | mittel | offen | – | Altes Ticket |\n";

pub(super) fn write(root: &Path, rel: &str, text: &str) {
    let mut path = root.to_owned();
    path.extend(rel.split('/'));
    std::fs::create_dir_all(path.parent().expect("Elternordner")).expect("Ordner anlegen");
    std::fs::write(path, text).expect("Datei schreiben");
}

pub(super) fn read(root: &Path, rel: &str) -> String {
    std::fs::read_to_string(root.join(rel)).expect("Datei lesen")
}

/// Wurzel mit einem Ticket, Index und den leeren Sprint-Ordnern (Go `tempRepo` ohne Vorlagen).
pub(super) fn temp_repo() -> tempfile::TempDir {
    let tmp = tmp_dir();
    let root = tmp.path();
    write(root, "docs/backlog/README.md", INDEX_MD);
    write(
        root,
        "docs/backlog/B-001-alt.md",
        "# B-001 · Altes Ticket\n\n- **Status:** offen\n",
    );
    for state in STATES {
        std::fs::create_dir_all(root.join("docs").join("sprints").join(state)).expect("Sprint-Ordner");
    }
    tmp
}

/// Ticket mit vollem Kopf; `extra` sind weitere Kopfzeilen.
pub(super) fn ticket(root: &Path, rel: &str, nr: &str, title: &str, extra: &str) {
    let text = format!(
        "# {nr} · {title}\n\n- **Domäne:** DEV\n- **Typ:** Idee\n- **Prio:** mittel\n- **Status:** offen\n{extra}\n\
         ## Ausgangslage\n\nText\n"
    );
    write(root, &format!("docs/{rel}"), &text);
}

/// Sprint mit README unter `docs/sprints/geplant/<id>-s/`.
pub(super) fn sprint(root: &Path, id: &str, project: &str, tickets: &str) {
    let text = format!(
        "# {id} · SRV · Sprint {id}\n\n- **Status:** geplant\n- **Domäne:** SRV\n- **Projekt:** {project}\n\
         - **Tickets:** {tickets}\n\n## Sessions\n"
    );
    write(root, &format!("docs/sprints/geplant/{id}-s/README.md"), &text);
}

/// Wurzel dieses Checkouts, wenn es eine Planung hat. Nur zum Lesen.
pub(super) fn real_root() -> Option<PathBuf> {
    let root = Path::new(env!("CARGO_MANIFEST_DIR")).ancestors().nth(4)?.to_owned();
    root.join("docs").join("sprints").exists().then_some(root)
}
