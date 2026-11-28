//! Hilfen der Unit-Tests; die Integrationstests binden dieselben Dateien aus `tests/support/` ein.

#[path = "../tests/support/mcp_client.rs"]
pub mod mcp_client;
#[path = "../tests/support/tmp.rs"]
mod tmp;

pub use tmp::tmp_dir;

/// Macht `dir` zu einer Repo-Wurzel für `find_root`: Datei `rust/Cargo.toml` und Ordner `data/` mit einer Datei (Git
/// hält keine leeren Ordner, sonst fehlte `data/` im Worktree).
pub fn mark_root(dir: &std::path::Path) {
    for (rel, text) in [("rust/Cargo.toml", "[workspace]\n"), ("data/economy.json", "{}\n")] {
        let path = dir.join(rel);
        std::fs::create_dir_all(path.parent().expect("Ordner")).expect("anlegen");
        std::fs::write(path, text).expect("schreiben");
    }
}
