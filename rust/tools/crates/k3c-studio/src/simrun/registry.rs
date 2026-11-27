//! Register der Läufe (Port von Go `registry.go`): Plätze je Checkout, Warteschlange, Abbruch, Ende mit Bericht.

use std::fs;
use std::panic::{AssertUnwindSafe, catch_unwind};
use std::path::{Path, PathBuf};
use std::sync::{Arc, Mutex, MutexGuard, Weak};
use std::thread;
use std::time::SystemTime;

use k3c_kit::logger::Logger;
use serde_json::{Value, json};

use super::info::Info;
use super::notify::Notify;
use super::offline::Exec;
use super::offline_run::exe_exec;
use super::run::{CANCELED, CancelToken, DONE, FAILED, RUNNING, Run, Runner, WAITING, lock};
use super::spec::Spec;
use crate::balance::writeguard;

/// Zahl gleichzeitiger Läufe je Checkout, gleich für alle Modi.
pub const MAX_PER_ROOT: usize = 2;

/// Warum das Register einen Aufruf ablehnt.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RegistryError {
    /// Alle Plätze des Checkouts belegt und der Aufrufer will nicht warten.
    Full,
    /// Keine Lauf-ID in diesem Checkout.
    Unknown,
}

impl std::fmt::Display for RegistryError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            RegistryError::Full => "alle Plätze belegt",
            RegistryError::Unknown => "lauf unbekannt",
        })
    }
}

impl std::error::Error for RegistryError {}

/// Berichtsordner eines Laufs relativ zum Checkout.
pub fn report_dir(run: &Run) -> PathBuf {
    Path::new("reports").join(format!("simtest-{}", run.id()))
}

pub(super) struct Inner {
    next: i64,
    pub(super) runs: Vec<Arc<Run>>,
    /// Rückrufe für die Ereignisse der Seite (notify.rs).
    pub(super) notify: Notify,
}

/// Hält alle Läufe des Studios.
pub struct Registry {
    pub(super) me: Weak<Registry>,
    exec: Mutex<Exec>,
    pub(super) log: Logger,
    inner: Mutex<Inner>,
}

impl Registry {
    /// Leeres Register; Läufe rechnet die Spiel-EXE.
    pub fn new(log: Logger) -> Arc<Registry> {
        Arc::new_cyclic(|me| Registry {
            me: me.clone(),
            exec: Mutex::new(exe_exec(log.clone())),
            log,
            inner: Mutex::new(Inner {
                next: 0,
                runs: Vec::new(),
                notify: Notify::default(),
            }),
        })
    }

    /// Ersetzt, was einen Lauf der Engine rechnet (Test-Naht); vor dem ersten Start setzen.
    pub fn set_exec(&self, exec: Exec) {
        *lock(&self.exec) = exec;
    }

    pub(super) fn exec(&self) -> Exec {
        Arc::clone(&lock(&self.exec))
    }

    pub(super) fn lock(&self) -> MutexGuard<'_, Inner> {
        lock(&self.inner)
    }

    /// Legt einen Lauf an. Ist ein Platz frei, rechnet er sofort; sonst wartet er (`queue`) oder es kommt
    /// [`RegistryError::Full`].
    pub fn start(
        &self,
        root: &Path,
        place: &str,
        label: &str,
        spec: Option<Spec>,
        runner: Runner,
        queue: bool,
    ) -> Result<Arc<Run>, RegistryError> {
        let mut g = self.lock();
        let head = spec.as_ref().is_some_and(|s| s.head);
        let free = can_launch(&g, root, head);
        if !free && !queue {
            return Err(RegistryError::Full);
        }
        g.next = g.next.max(highest_run_dir(root)).saturating_add(1);
        let id = format!("run-{}", g.next);
        let run = Arc::new(Run::new(id, root, place, label, spec, Some(runner)));
        g.runs.push(Arc::clone(&run));
        if free {
            self.launch(&g, &run);
        } else {
            self.log
                .info("⏳ testlauf wartet", &attrs(&run, &[("mode", json!(label))]));
            self.changed(&g, &run);
        }
        Ok(run)
    }

    /// Startet den Runner in eigenem Thread (er überlebt den Aufruf).
    fn launch(&self, g: &Inner, run: &Arc<Run>) {
        let token = CancelToken::new();
        let slot = free_slot(g, run.root());
        let runner = {
            let mut s = run.state();
            s.slot = slot;
            s.state = RUNNING;
            "start".clone_into(&mut s.phase);
            s.started = SystemTime::now();
            s.active = true;
            s.cancel = Some(token.clone());
            s.runner.take()
        };
        self.log
            .info("🚀 testlauf gestartet", &attrs(run, &[("mode", json!(run.label()))]));
        self.changed(g, run);
        let (me, run) = (self.me.upgrade(), Arc::clone(run));
        thread::spawn(move || {
            let Some(me) = me else { return };
            // eine Panic im Runner beendet den Lauf, nicht das Studio, und gibt den Platz frei
            if let Some(runner) = runner
                && let Err(p) = catch_unwind(AssertUnwindSafe(|| runner(token, Arc::clone(&run))))
            {
                let text = p.downcast_ref::<&str>().map(|s| (*s).to_owned());
                let text = text.or_else(|| p.downcast_ref::<String>().cloned()).unwrap_or_default();
                me.log
                    .error("💥 testlauf abgestürzt", &attrs(&run, &[("panic", json!(text))]));
            }
            me.ended(&run);
        });
    }

    /// Gibt den Platz eines zurückgekehrten Runners frei und startet Wartende.
    fn ended(&self, run: &Arc<Run>) {
        let g = self.lock();
        let (state, verdict) = {
            let mut s = run.state();
            s.active = false;
            if s.state == RUNNING {
                // Runner ohne finish
                s.state = FAILED;
                s.ended = Some(SystemTime::now());
            }
            (s.state, s.verdict.clone())
        };
        let msg = if state == DONE {
            "✅ testlauf beendet"
        } else {
            "❌ testlauf beendet"
        };
        self.log.info(
            msg,
            &attrs(run, &[("state", json!(state)), ("verdict", json!(verdict))]),
        );
        self.changed(&g, run);
        self.launch_next(&g, run.root());
    }

    /// Startet wartende Läufe des Checkouts, solange Plätze frei sind.
    fn launch_next(&self, g: &Inner, root: &Path) {
        for run in &g.runs {
            if run.root() != root || run.state().state != WAITING {
                continue;
            }
            if !can_launch(g, root, run.head()) {
                return; // der Älteste wartet zuerst: ein wartender Fensterlauf lässt keine späteren vorbei
            }
            self.launch(g, run);
        }
    }

    /// Bricht einen Lauf ab und nennt seinen Zustand davor: ein wartender verschwindet aus dem Register, ein laufender
    /// heißt „abgebrochen“ und räumt im Hintergrund auf; sonst ändert sich nichts.
    pub fn stop(&self, root: &Path, id: &str) -> Result<&'static str, RegistryError> {
        let mut g = self.lock();
        let run = find(&g, root, id).ok_or(RegistryError::Unknown)?;
        let before = run.state().state;
        if before == WAITING {
            g.runs.retain(|r| !Arc::ptr_eq(r, &run));
            run.state().runner = None;
            self.log.info("🧹 wartender testlauf entfernt", &attrs(&run, &[]));
            self.changed(&g, &run);
        } else if before == RUNNING {
            {
                let mut s = run.state();
                s.state = CANCELED;
                if let Some(token) = &s.cancel {
                    token.cancel();
                }
            }
            self.log.info("🛑 testlauf abgebrochen", &attrs(&run, &[]));
            self.changed(&g, &run);
        }
        Ok(before)
    }

    /// Beendet alle Läufe und verwirft die wartenden (Studio stoppt).
    pub fn cancel_all(&self) {
        let mut g = self.lock();
        g.runs.retain(|r| {
            let mut s = r.state();
            if s.state == WAITING {
                s.runner = None;
                return false;
            }
            if s.state == RUNNING
                && let Some(token) = &s.cancel
            {
                token.cancel();
            }
            true
        });
    }

    /// Zahl der gemerkten Läufe.
    pub fn count(&self) -> usize {
        self.lock().runs.len()
    }

    /// Setzt den Schritt des Runners, z. B. „bauen“.
    pub fn set_phase(&self, run: &Arc<Run>, phase: &str) {
        let g = self.lock();
        phase.clone_into(&mut run.state().phase);
        self.changed(&g, run);
    }

    /// Setzt die Kennzahlen eines laufenden Laufs (Zwischenstand).
    pub fn set_lines(&self, run: &Arc<Run>, lines: Vec<String>) {
        let g = self.lock();
        run.state().lines = lines;
        self.changed(&g, run);
    }

    /// Setzt den Fortschritt (fertige und alle Szenarien).
    pub fn set_progress(&self, run: &Arc<Run>, done: i64, total: i64) {
        let g = self.lock();
        {
            let mut s = run.state();
            s.done = done;
            s.total = total;
        }
        self.changed(&g, run);
    }

    /// Setzt das Ende und schreibt den eigenen Bericht `dir/simtest.md` (`dir` relativ zum Checkout). Ein Abbruch über
    /// [`Registry::stop`] gewinnt gegen ein späteres „fertig“ des Runners. Den Bericht nennt der Lauf erst, wenn die
    /// Datei steht.
    pub fn finish(&self, run: &Arc<Run>, state: &'static str, verdict: &str, lines: Vec<String>, dir: &Path) {
        let text = {
            let _g = self.lock();
            let body = lines.join("\n- ");
            {
                let mut s = run.state();
                if s.state == RUNNING {
                    s.state = state;
                }
                s.ended = Some(SystemTime::now());
                verdict.clone_into(&mut s.verdict);
                s.lines = lines;
            }
            let head = run.info().head(chrono::Local::now());
            format!("# Testlauf {}\n\n{head}\n\n- {body}\n", run.id())
        };
        let full = run.root().join(dir);
        let written = writeguard::mkdir_all(run.root(), &full)
            .and_then(|()| writeguard::write_file(run.root(), full.join("simtest.md"), text.as_bytes()));
        if let Err(e) = written {
            self.log.error(
                "❌ testlauf-bericht nicht geschrieben",
                &attrs(run, &[("error", json!(e.to_string()))]),
            );
        }
        let g = self.lock();
        run.state().report = dir.join("simtest.md").to_string_lossy().replace('\\', "/");
        self.changed(&g, run);
        Self::reports_changed(&g, run.root());
    }

    /// Stand eines Laufs.
    pub fn get(&self, root: &Path, id: &str) -> Result<Info, RegistryError> {
        let g = self.lock();
        find(&g, root, id).map(|r| r.info()).ok_or(RegistryError::Unknown)
    }

    /// Läufe des Checkouts, neueste zuerst.
    pub fn list(&self, root: &Path) -> Vec<Info> {
        let g = self.lock();
        g.runs
            .iter()
            .rev()
            .filter(|r| r.root() == root)
            .map(|r| r.info())
            .collect()
    }
}

/// Log-Felder eines Laufs.
fn attrs<'a>(run: &Run, more: &[(&'a str, Value)]) -> Vec<(&'a str, Value)> {
    let mut out = vec![("ns", json!("simtest")), ("id", json!(run.id()))];
    out.extend_from_slice(more);
    out
}

/// Höchste Nummer N der Ordner `reports/simtest-run-N` im Checkout (0 ohne), damit IDs nach einem Neustart keinen
/// vorhandenen Ordner wiederverwenden (B-616).
fn highest_run_dir(root: &Path) -> i64 {
    let Ok(entries) = fs::read_dir(root.join("reports")) else {
        return 0;
    };
    entries
        .filter_map(Result::ok)
        .filter_map(|e| e.file_name().to_str()?.strip_prefix("simtest-run-")?.parse().ok())
        .fold(0, i64::max)
}

/// Darf im Checkout ein weiterer Lauf starten? Ein Lauf mit Fenster (`head`) läuft allein: ein Fenster, und nichts
/// rechnet daneben (RU40.3).
fn can_launch(g: &Inner, root: &Path, head: bool) -> bool {
    let busy: Vec<&Arc<Run>> = g.runs.iter().filter(|r| r.root() == root && r.state().active).collect();
    if busy.len() >= MAX_PER_ROOT {
        return false;
    }
    if head {
        return busy.is_empty();
    }
    !busy.iter().any(|r| r.head())
}

/// Kleinster Platz, den kein aktiver Lauf des Checkouts hält.
fn free_slot(g: &Inner, root: &Path) -> usize {
    let used: Vec<usize> = g
        .runs
        .iter()
        .filter(|r| r.root() == root)
        .filter_map(|r| {
            let s = r.state();
            s.active.then_some(s.slot)
        })
        .collect();
    (0..).find(|n| !used.contains(n)).unwrap_or_default()
}

fn find(g: &Inner, root: &Path, id: &str) -> Option<Arc<Run>> {
    g.runs.iter().find(|r| r.root() == root && r.id() == id).cloned()
}

#[cfg(test)]
#[path = "registry_tests.rs"]
mod tests;
