//! Hilfen der Tests (aus Go `stamp_test.go`, `manage_test.go`, `offline_test.go`, `registry_test.go`). Kein Test
//! startet die Spiel-EXE: das Rechnen ist eine Attrappe.

use std::path::{Path, PathBuf};
use std::sync::Arc;
use std::thread;
use std::time::{Duration, Instant};

use k3c_kit::logger::Logger;
use tempfile::TempDir;

use super::info::Info;
use super::offline::{Proc, ProcResult};
use super::registry::{Registry, report_dir};
use super::run::{CancelToken, DONE, Run, Runner};

/// Repo-Wurzel für die Daten unter `data/`.
pub fn repo_root() -> PathBuf {
    PathBuf::from(concat!(env!("CARGO_MANIFEST_DIR"), "/../../../.."))
}

/// Temporärer Checkout unter `.work/` ohne `..` im Pfad (der Schreibschutz lehnt `..` ab).
pub fn temp_root() -> (TempDir, PathBuf) {
    let dir = crate::test_helpers::tmp_dir();
    let root = std::path::absolute(dir.path().join("repo")).expect("absolut");
    std::fs::create_dir_all(&root).expect("Wurzel");
    (dir, root)
}

pub fn registry() -> Arc<Registry> {
    Registry::new(Logger::discard())
}

pub fn write_report(root: &Path, rel: &str, body: &str) {
    let path = root.join(rel);
    std::fs::create_dir_all(path.parent().expect("Ordner")).expect("anlegen");
    std::fs::write(path, body).expect("schreiben");
}

/// Wert des Schalters `name` in den Argumenten.
pub fn flag<'a>(args: &'a [String], name: &str) -> &'a str {
    let at = args.iter().position(|a| a == name).expect("Schalter");
    &args[at + 1]
}

/// Register, dessen Rechnen sofort `body` als `balance-1.json` in den Berichtsordner des Laufs schreibt.
pub fn writing_registry(body: &str) -> Arc<Registry> {
    let g = registry();
    let body = body.to_owned();
    g.set_exec(Arc::new(move |_: CancelToken, p: Proc| {
        let dir = p.dir.join(flag(&p.args, "--report-dir"));
        write_report(&dir, "balance-1.json", &body);
        ProcResult::default()
    }));
    g
}

/// Fragt den Lauf ab, bis `ok` gilt.
pub fn wait_for(g: &Registry, root: &Path, id: &str, ok: impl Fn(&Info) -> bool) -> Info {
    let deadline = Instant::now() + Duration::from_secs(5);
    loop {
        let info = g.get(root, id).expect("Lauf bekannt");
        if ok(&info) {
            return info;
        }
        assert!(Instant::now() < deadline, "Zustand nicht erreicht: {info:?}");
        thread::sleep(Duration::from_millis(10));
    }
}

/// Runner, der bis zum Abbruch oder bis `release` wartet und dann „fertig“ meldet.
pub fn blocking(g: &Arc<Registry>, release: &CancelToken) -> Runner {
    let (g, release) = (Arc::downgrade(g), release.clone());
    Arc::new(move |cancel: CancelToken, run: Arc<Run>| {
        while !cancel.is_canceled() && !release.wait_timeout(Duration::from_millis(5)) {}
        if let Some(g) = g.upgrade() {
            g.finish(&run, DONE, "Pass", Vec::new(), &report_dir(&run));
        }
    })
}

/// Gibt den blockierenden Lauf am Testende frei und wartet, bis `finish` seinen Bericht geschrieben hat; sonst schriebe
/// er unter Windows noch in `reports/`, während der Temp-Ordner gelöscht wird (B-695). Nach dem Temp-Ordner anlegen.
pub struct Release {
    pub g: Arc<Registry>,
    pub run: Arc<Run>,
    pub release: CancelToken,
}

impl Drop for Release {
    fn drop(&mut self) {
        self.release.cancel();
        if !thread::panicking() {
            wait_for(&self.g, self.run.root(), self.run.id(), |i| !i.report.is_empty());
        }
    }
}
