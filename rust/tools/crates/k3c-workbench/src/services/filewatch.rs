//! Watch-Modus (Port von `filewatch.go`): bei Änderungen beobachteter Dateien den Dienst neu starten. Wie Go ohne
//! Betriebssystem-Ereignisse, nur durch regelmäßiges Lesen von Änderungszeit und Größe.

use std::collections::BTreeMap;
use std::fs::Metadata;
use std::path::Path;
use std::sync::{Arc, PoisonError, RwLock};
use std::time::SystemTime;

use serde::Deserialize;
use serde_json::json;
use tokio::time::sleep;

use super::controller::{Controller, Unit};
use super::lifecycle::ticker;
use super::options::Cancel;
use super::status::State;

/// Hält [`snapshot`] an, solange jemand schreibend sperrt: Tests schreiben mehrere Dateien als eine Änderung. Im
/// Betrieb nimmt nur `snapshot` die Lesesperre.
pub(crate) static FILES_BATCH: RwLock<()> = RwLock::new(());

/// Eintrag `watch` in `services.json`.
#[derive(Clone, Debug, Default, PartialEq, Eq, Deserialize)]
#[serde(default, deny_unknown_fields)]
pub struct Watch {
    /// Ordner oder Dateien relativ zur Repo-Wurzel; Ordner samt Unterordnern.
    pub paths: Vec<String>,
    /// Begrenzt Dateien in Ordnern auf diese Endungen (mit Punkt); leer = alle. Einzeln genannte zählen immer.
    pub ext: Vec<String>,
}

/// Woran sich eine Änderung erkennen lässt: Änderungszeit und Größe.
pub(crate) type FileSig = (Option<SystemTime>, u64);
pub(crate) type Snapshot = BTreeMap<String, FileSig>;

fn signature(m: &Metadata) -> FileSig {
    (m.modified().ok(), m.len())
}

/// Endung wie Go `filepath.Ext`, klein.
fn extension(name: &str) -> String {
    name.rfind('.').map_or_else(String::new, |i| name[i..].to_lowercase())
}

/// Versteckte Ordner, `node_modules` und nicht lesbare Pfade zählen nicht.
fn walk(dir: &Path, rel: &str, watch: &Watch, out: &mut Snapshot) {
    let Ok(entries) = std::fs::read_dir(dir) else {
        return;
    };
    for entry in entries.flatten() {
        let name = entry.file_name().to_string_lossy().into_owned();
        let rel = if rel.is_empty() || rel == "." {
            name.clone()
        } else {
            format!("{rel}/{name}")
        };
        let Ok(kind) = entry.file_type() else { continue };
        if kind.is_dir() {
            if !name.starts_with('.') && name != "node_modules" {
                walk(&entry.path(), &rel, watch, out);
            }
        } else if (watch.ext.is_empty() || watch.ext.contains(&extension(&name)))
            && let Ok(m) = entry.metadata()
        {
            out.insert(rel, signature(&m));
        }
    }
}

/// Liest alle beobachteten Dateien; ein fehlender Pfad kann später entstehen.
pub(crate) fn snapshot(root: &Path, watch: &Watch) -> Snapshot {
    let _batch = FILES_BATCH.read().unwrap_or_else(PoisonError::into_inner);
    let mut out = Snapshot::new();
    for p in &watch.paths {
        let abs = root.join(p);
        let Ok(m) = std::fs::metadata(&abs) else {
            continue;
        };
        if m.is_dir() {
            walk(&abs, p.trim_end_matches('/'), watch, &mut out);
        } else {
            out.insert(p.clone(), signature(&m));
        }
    }
    out
}

/// Die erste Datei in Namensreihenfolge, die neu, geändert oder weg ist.
pub(crate) fn changed(before: &Snapshot, after: &Snapshot) -> Option<String> {
    let new_or_changed = after
        .iter()
        .filter(|(p, sig)| before.get(*p) != Some(sig))
        .map(|(p, _)| p);
    let gone = before.keys().filter(|p| !after.contains_key(*p));
    new_or_changed.chain(gone).min().cloned()
}

impl Controller {
    async fn snapshot_of(&self, watch: &Watch) -> Snapshot {
        let (root, watch) = (self.opts.root.clone(), watch.clone());
        tokio::task::spawn_blocking(move || snapshot(&root, &watch))
            .await
            .unwrap_or_default()
    }

    /// Beginnt (falls noch nicht), die Dateien eines Dienstes mit `watch` zu beobachten.
    pub(crate) async fn watch_files(self: &Arc<Self>, u: &Arc<Unit>) {
        let Some(watch) = u.svc().watch.clone() else {
            return;
        };
        // Der Ausgangsstand entsteht hier, nicht erst im Beobachter: sonst ginge eine Änderung direkt nach dem Start
        // verloren oder würde halb geschrieben zum Ausgangsstand.
        let base = self.snapshot_of(&watch).await;
        let cancel = Cancel::default();
        {
            let mut data = u.data();
            if data.unfiles.is_some() {
                return;
            }
            data.unfiles = Some(cancel.clone());
        }
        tokio::spawn(Arc::clone(self).files_loop(cancel, Arc::clone(u), base));
    }

    /// Beendet die Beobachtung (der Dienst wurde ausdrücklich gestoppt).
    pub(crate) fn unwatch_files(u: &Unit) {
        if let Some(cancel) = u.data().unfiles.take() {
            cancel.cancel();
        }
    }

    /// Eine Änderung wird erst nach `files_quiet` Ruhe wirksam (Speichern in mehreren Schritten, Formatierer).
    async fn files_loop(self: Arc<Self>, cancel: Cancel, u: Arc<Unit>, mut base: Snapshot) {
        let mut tick = ticker(self.opts.files_every);
        loop {
            tokio::select! {
                biased;
                () = cancel.cancelled() => return,
                _ = tick.tick() => {}
            }
            // Je Durchlauf neu: „Neu laden“ kann die Einstellung ändern. Ohne `watch` gibt es nichts zu prüfen (Go
            // stürzt hier ab).
            let Some(watch) = u.svc().watch.clone() else {
                continue;
            };
            let current = self.snapshot_of(&watch).await;
            let Some(file) = changed(&base, &current) else {
                continue;
            };
            let Some(settled) = self.settle(&cancel, &watch, current).await else {
                return;
            };
            base = settled;
            self.restart_for_change(&cancel, &u, &file).await;
        }
    }

    /// Wartet, bis sich die Dateien `files_quiet` lang nicht mehr ändern.
    async fn settle(&self, cancel: &Cancel, watch: &Watch, mut current: Snapshot) -> Option<Snapshot> {
        loop {
            tokio::select! {
                biased;
                () = cancel.cancelled() => return None,
                () = sleep(self.opts.files_quiet) => {}
            }
            let next = self.snapshot_of(watch).await;
            if changed(&current, &next).is_none() {
                return Some(next);
            }
            current = next;
        }
    }

    /// Ein Stopp gewinnt: er beendet die Beobachtung und hält die Befehlssperre.
    async fn restart_for_change(self: &Arc<Self>, cancel: &Cancel, u: &Arc<Unit>, file: &str) {
        let _cmd = u.cmd.lock().await;
        if cancel.is_cancelled() {
            return;
        }
        if matches!(u.status().state, State::Stopped | State::Adopted | State::Stopping) {
            return;
        }
        let message = format!("🔁 dienst {}: Neustart wegen Änderung an {file}", u.svc().name);
        self.opts.log.info(&message, &[("ns", json!("svc"))]);
        let at = (self.opts.now)();
        // Sofort sichtbar, auch während des Builds.
        self.set(u, |s| {
            file.clone_into(&mut s.change_file);
            s.change_at = Some(at);
        });
        let mut result = self.stop_unit(u).await;
        if result.is_ok() {
            result = self.start_unit(cancel, u).await;
        }
        let source = self.source(u);
        let line = format!("k3c-workbench: Neustart wegen Änderung an {file}");
        self.opts.console.add(&source, "stdout", &line);
        if let Err(e) = result {
            let line = format!("k3c-workbench: Neustart fehlgeschlagen: {}", e.message);
            self.opts.console.add(&source, "stderr", &line);
        }
    }
}
